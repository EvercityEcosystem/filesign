#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    decl_event,
    decl_error, 
    decl_module, 
    decl_storage,
    dispatch::{
        DispatchResult,
        Vec,
    },
};
use frame_system::{
    ensure_signed,
};

use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
};

use file::{FileStruct, H256};

#[cfg(test)]
mod mock;

#[cfg(test)]    
mod tests;
pub mod file;

pub type FileId = u32;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as Filesign {
        /// Storage map for file IDs
        FileByID
            get(fn file_by_id):
            map hasher(blake2_128_concat) FileId => Option<FileStruct<T::AccountId>>;   

        /// Last Id of created file
        LastID: FileId;
    }
}

decl_event! (
    pub enum Event<T>
    where 
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// \[account, fileid, signer\]
        SignerAssigned(AccountId, FileId, AccountId),
        /// \[account, fileid\]
        FileCreated(AccountId, FileId),
        /// \[account, fileid, signer\]
        SignerDeleted(AccountId, FileId, AccountId),
        /// \[account, fileid\]
        FileSigned(AccountId, FileId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        AddressNotSigner,
        AddressNotOwner,
        FileNotFound,
        EmptyTag,
        FileHasNoSigners,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;
        type Error = Error<T>;

        #[weight = 10_000]
		pub fn sign_latest_version(origin, id: FileId) {
			let caller = ensure_signed(origin)?;
            match FileByID::<T>::get(id) {
                None => Err(Error::<T>::FileNotFound)?,
                Some(file) => {
                    ensure!(file.signers.iter().any(|x| *x == caller), Error::<T>::AddressNotSigner);
                }
            }

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    ensure!(file_by_id.as_ref().is_some(), Error::<T>::FileNotFound);
                    file_by_id.as_mut().unwrap().sign_latest_version(caller.clone());

                    Ok(())
                })?;

            Self::deposit_event(RawEvent::FileSigned(caller, id));
		}

        #[weight = 10_000]
        pub fn create_new_file(origin, tag: Vec<u8>, filehash: H256) -> DispatchResult {
            ensure!(tag.len() != 0, Error::<T>::EmptyTag);
            let caller = ensure_signed(origin)?;
            
            // Update last created file ID
            let new_id = LastID::get() + 1;
            let new_file = FileStruct::<<T as frame_system::Config>::AccountId>::new(caller.clone(), new_id, tag, &filehash);

            <FileByID<T>>::insert(new_id, new_file);
            LastID::mutate(|x| *x += 1);

            Self::deposit_event(RawEvent::FileCreated(caller, new_id));
            Ok(())
        }
        
        #[weight = 10_000]
        pub fn delete_signer(origin, id: FileId, signer: T::AccountId)  {
            let caller = ensure_signed(origin)?;
            match FileByID::<T>::get(id) {
                None => Err(Error::<T>::FileNotFound)?,
                Some(file) => {
                    ensure!(file.owner == caller, Error::<T>::AddressNotOwner);
                    ensure!(file.signers.iter().any(|x| *x == signer), Error::<T>::AddressNotSigner);
                }
            }

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    ensure!(file_by_id.as_ref().is_some(), Error::<T>::FileNotFound);
                    ensure!(file_by_id.as_mut().unwrap().delete_signer_from_file(signer.clone()).is_ok(), 
                        Error::<T>::FileHasNoSigners);

                    Ok(())
                }
            )?;

            Self::deposit_event(RawEvent::SignerDeleted(caller, id, signer));
        }

        #[weight = 10_000]
        pub fn assign_signer(origin, id: u32, signer: T::AccountId) {
            let caller = ensure_signed(origin)?;
            match FileByID::<T>::get(id) {
                None => Err(Error::<T>::FileNotFound)?,
                Some(file) => {
                    ensure!(file.owner == caller, Error::<T>::AddressNotOwner);
                }
            }

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    ensure!(file_by_id.as_ref().is_some(), Error::<T>::FileNotFound);
                    file_by_id.as_mut().unwrap().assign_signer_to_file(signer.clone());
                    Ok(())
                }
            )?;

            Self::deposit_event(RawEvent::SignerAssigned(caller, id, signer));
        }
    }
}

impl<T: Config> Module<T> {
    /// <pre>
    /// Method: address_is_auditor_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: FileId, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is an auditor for the given file
    /// </pre>
    pub fn address_is_signer_for_file(id: FileId, address: &T::AccountId) -> bool {
        match FileByID::<T>::get(id) {
            None => false,
            Some(file) => file.signers.iter().any(|x| x == address)
        }
    }

    /// <pre>
    /// Method: address_is_owner_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: FileId, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is the owner for the given file
    /// </pre>
    pub fn address_is_owner_for_file(id: FileId, address: &T::AccountId) -> bool {
        match FileByID::<T>::get(id) {
            None => false,
            Some(file) => file.owner == *address
        }
    }

    /// <pre>
    /// Method: get_file_by_id(id: FileId) -> Option<FileStruct<<T as frame_system::Config>::AccountId>> 
    /// Arguments: id: FileId - file ID
    ///
    /// Returns the file option
    /// </pre>
    pub fn get_file_by_id(id: FileId) -> Option<FileStruct<<T as frame_system::Config>::AccountId>> {
        FileByID::<T>::get(id)
    }
}