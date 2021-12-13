#![allow(clippy::unused_unit)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]    
mod tests;
pub mod file;

use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Randomness;
use crate::sp_api_hidden_includes_decl_storage::hidden_include::traits::Get;
use codec::Encode;
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
use file::{FileStruct, H256, FileId};

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Randomness: frame_support::traits::Randomness<Self::Hash>;
}

decl_storage! {
    trait Store for Module<T: Config> as Filesign {
        /// Storage map for file IDs
        FileByID
            get(fn file_by_id):
            map hasher(blake2_128_concat) FileId => Option<FileStruct<T::AccountId>>;

        /// Nonce for random file id generating 
        NonceId: u64;
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
        /// Address is not a signer 
        AddressNotSigner,
        /// Address is not an owner of a file
        AddressNotOwner,
        /// No such file in storage
        FileNotFound,
        /// Validation error - no tag
        EmptyTag,
        /// Validation error - no tag
        FileHasNoSigners,
        /// File id is busy
        IdAlreadyExists,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;
        type Error = Error<T>;

        #[weight = T::DbWeight::get().reads_writes(2, 1) + 10_000]
        pub fn create_new_file(origin, tag: Vec<u8>, filehash: H256, file_id_option: Option<FileId>) -> DispatchResult {
            ensure!(!tag.is_empty(), Error::<T>::EmptyTag);
            let caller = ensure_signed(origin)?;
            
            // Update last created file ID
            let file_id = match file_id_option {
                Some(id) => id,
                None => Self::get_random_id()
            };
            ensure!(<FileByID<T>>::get(file_id).is_none(), Error::<T>::IdAlreadyExists);
            let new_file = FileStruct::<<T as frame_system::Config>::AccountId>::new(caller.clone(), file_id, tag, &filehash);
            <FileByID<T>>::insert(file_id, new_file);
            Self::deposit_event(RawEvent::FileCreated(caller, file_id));
            Ok(())
        }

        #[weight = T::DbWeight::get().reads_writes(1, 1) + 10_000]
		pub fn sign_latest_version(origin, id: FileId) {
			let caller = ensure_signed(origin)?;
            FileByID::<T>::try_mutate(
                id, |file_option| -> DispatchResult {
                    match file_option {
                        None => return Err(Error::<T>::FileNotFound.into()),
                        Some(file) => {
                            ensure!(file.signers.iter().any(|x| *x == caller), Error::<T>::AddressNotSigner);
                            file.sign_latest_version(caller.clone());
                        }
                    }
                    Ok(())
                })?;

            Self::deposit_event(RawEvent::FileSigned(caller, id));
		}
        
        #[weight = T::DbWeight::get().reads_writes(1, 1) + 10_000]
        pub fn delete_signer(origin, id: FileId, signer: T::AccountId)  {
            let caller = ensure_signed(origin)?;

            FileByID::<T>::try_mutate(
                id, |file_option| -> DispatchResult {
                    match file_option {
                        None => return Err(Error::<T>::FileNotFound.into()),
                        Some(file) => {
                            ensure!(file.owner == caller, Error::<T>::AddressNotOwner);
                            ensure!(file.signers.iter().any(|x| *x == signer), Error::<T>::AddressNotSigner);
                            ensure!(file.delete_signer_from_file(signer.clone()).is_ok(), 
                                   Error::<T>::AddressNotSigner);
                        }
                    }
                    Ok(())
                }
            )?;

            Self::deposit_event(RawEvent::SignerDeleted(caller, id, signer));
        }

        #[weight = T::DbWeight::get().reads_writes(1, 1) + 10_000]
        pub fn assign_signer(origin, id: FileId, signer: T::AccountId) {
            let caller = ensure_signed(origin)?;

            FileByID::<T>::try_mutate(
                id, |file_option| -> DispatchResult {
                    match file_option {
                        None => return Err(Error::<T>::FileNotFound.into()),
                        Some(file) => {
                            ensure!(file.owner == caller, Error::<T>::AddressNotOwner);
                            file.assign_signer_to_file(signer.clone());
                        }
                    }
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
    /// Method: address_has_signed_the_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: FileId, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address has signed last version of the given file
    /// </pre>
    pub fn address_has_signed_the_file(id: FileId, address: &T::AccountId) -> bool {
        match FileByID::<T>::get(id) {
            None => false,
            Some(file) => {
                if let Some(vers_strunct) = file.versions.last() {
                    return vers_strunct.signatures.iter().any(|sign| sign.address == *address && sign.signed);
                }
                false
            }
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
    #[inline]
    pub fn get_file_by_id(id: FileId) -> Option<FileStruct<<T as frame_system::Config>::AccountId>> {
        FileByID::<T>::get(id)
    }

    fn get_random_id() -> FileId {
        let nonce = Self::get_and_increment_nonce();
        let rand = T::Randomness::random(&nonce);
        codec::Encode::using_encoded(&rand, sp_io::hashing::blake2_128)
    }

    fn get_and_increment_nonce() -> Vec<u8> {
        let nonce = NonceId::get();
        NonceId::put(nonce.wrapping_add(1));
        nonce.encode()
    }
}