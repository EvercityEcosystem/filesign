#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    decl_event,
    decl_error, 
    decl_module, 
    decl_storage,
    dispatch::{
        DispatchResult, 
        DispatchError, 
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
mod file;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as Filesign {
        /// Storage map for file IDs
        FileByID
            get(fn file_by_id):
            map hasher(blake2_128_concat) u32 => FileStruct<T::AccountId>;   

        /// Last Id of created file
        LastID: u32;
    }
}

decl_event! (
    pub enum Event<T>
    where 
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// \[account, fileid, signer\]
        SignerAssigned(AccountId, u32, AccountId),
        /// \[account, fileid\]
        FileCreated(AccountId, u32),
        /// \[account, fileid, signer\]
        SignerDeleted(AccountId, u32, AccountId),
        /// \[account, fileid\]
        FileSigned(AccountId, u32),
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
		pub fn sign_latest_version(origin, id: u32) {
			let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_signer_for_file(id, &caller), Error::<T>::AddressNotSigner);
            ensure!(FileByID::<T>::contains_key(id), Error::<T>::FileNotFound);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    file_by_id.sign_latest_version(caller.clone());

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
        pub fn delete_signer(origin, id: u32, signer: T::AccountId)  {
            let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_owner_for_file(id, &caller), Error::<T>::AddressNotOwner);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    ensure!(file_by_id.delete_signer_from_file(signer.clone()).is_ok(), 
                        Error::<T>::FileHasNoSigners);

                    Ok(())
                }
            )?;

            Self::deposit_event(RawEvent::SignerDeleted(caller, id, signer));
        }

        #[weight = 10_000]
        pub fn assign_signer(origin, id: u32, signer: T::AccountId) {
            let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_owner_for_file(id, &caller), Error::<T>::AddressNotOwner);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    file_by_id.assign_signer_to_file(signer.clone());
                    Ok(())
                }
            )?;

            Self::deposit_event(RawEvent::SignerAssigned(caller, id, signer));
        }
    }
}

impl<T: Config> Module<T> {
    /// <pre>
    /// Method: address_is_signer_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: u32, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is an signer for the given file
    /// </pre>
    pub fn address_is_signer_for_file(id: u32, address: &T::AccountId) -> bool {
        FileByID::<T>::get(id).signers.iter().any(|x| x == address)
    }

    /// <pre>
    /// Method: address_is_owner_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: u32, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is the owner for the given file
    /// </pre>
    pub fn address_is_owner_for_file(id: u32, address: &T::AccountId) -> bool {
        FileByID::<T>::get(id).owner == *address
    }

    #[cfg(test)]
    fn get_file_by_id(id: u32) -> FileStruct<<T as frame_system::Config>::AccountId> {
        FileByID::<T>::get(id)
    }
}