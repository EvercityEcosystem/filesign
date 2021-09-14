#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    decl_error, 
    decl_module, 
    decl_storage,
    codec::{
        Decode, 
        Encode
    },
    dispatch::{
        DispatchResult, 
        DispatchError, 
        Vec,
    },
    traits::{
        Get
    },
};
use frame_system::{
    ensure_signed,
};
use frame_support::sp_runtime::{
    RuntimeDebug,
    traits::{
        Hash,
    }
};
use frame_support::sp_std::{
    cmp::{
        Eq, 
        PartialEq}, 
    result::{
        Result
    },
};

use file::{VersionStruct, SigStruct, FileStruct, H256};

#[cfg(test)]
mod mock;

#[cfg(test)]    
mod tests;
mod file;



pub trait Config: frame_system::Config {}

decl_storage! {
    trait Store for Module<T: Config> as Audit {
        /// Storage map for file IDs
        FileByID
            get(fn file_by_id):
            map hasher(blake2_128_concat) u32 => FileStruct<T::AccountId>;   

        /// Last Id of created file
        LastID: u32;
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        AddressNotAuditor,
        AddressNotOwner,
    }
}


decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = 10_000]
		pub fn sign_latest_version(origin, id: u32) {
			let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_auditor_for_file(id, &caller), Error::<T>::AddressNotAuditor);
            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    let latest_version = file_by_id.versions.last_mut().unwrap();

                    // here check if has already signed
                    match latest_version.signatures.iter().position(|sig| sig.address == caller) {
                        Some(_) => {/*new logic can be made in future here*/},
                        None => {
                            latest_version.signatures.push(SigStruct{address: caller, signed: true});         
                        }
                    }
                    Ok(())
                })?;
		}

        #[weight = 10_000]
        pub fn create_new_file(origin, tag: Vec<u8>, filehash: H256) -> DispatchResult {
            if tag.len() == 0 {
                return Err(DispatchError::Other("empty file error"))
            }

            let caller = ensure_signed(origin)?;
            
            let empty_vec: Vec<SigStruct<<T as frame_system::Config>::AccountId>> = Vec::new();
            let latest_version = VersionStruct {
                tag,
                filehash,
                signatures: empty_vec,
            };

            let mut versions = Vec::with_capacity(1);
            versions.push(latest_version);

            // Update last created file ID
            let last_id = LastID::get();
            let new_id = last_id + 1;

            let new_file = FileStruct {
                owner: caller,
                id: new_id,
                versions: versions,
                auditors: Vec::new(),
            };

            <FileByID<T>>::insert(new_id, new_file);
            LastID::mutate(|x| *x += 1);

            Ok(())
        }

        #[weight = 10_000]
        pub fn get_info_by_tag(origin, id: u32, tag: Vec<u8>) // -> VersionStruct<<T as frame_system::Config>::AccountId> 
        {
            let file = FileByID::<T>::get(id);
            let index = file.versions.iter().position(|v| v.tag == tag).unwrap();
            // TODO: return file.versions[index]
        }
        
        #[weight = 10_000]
        pub fn delete_auditor(origin, id: u32, auditor: T::AccountId)  {
            let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_owner_for_file(id, &caller), Error::<T>::AddressNotOwner);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    let index = match file_by_id.auditors.iter().position(|a| a == &auditor) {
                        Some(i) => i,
                        None => return Err(DispatchError::Other("no auditor"))
                    };
                    file_by_id.auditors.remove(index);
                    Ok(())
                })?;
        }

        #[weight = 10_000]
        pub fn assign_auditor(origin, id: u32, auditor: T::AccountId) {
            let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_owner_for_file(id, &caller), Error::<T>::AddressNotOwner);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    if !file_by_id.auditors.iter().any(|x| *x == caller) {
                        file_by_id.auditors.push(auditor);
                    }          
                    Ok(())
                })?;
        }
    }
}

impl<T: Config> Module<T> {
    /// <pre>
    /// Method: address_is_auditor_for_file(id: u32, address: &T::AccountId) -> bool
    /// Arguments: id: u32, address: &T::AccountId - file ID, address
    ///
    /// Checks if the address is an auditor for the given file
    /// </pre>
    pub fn address_is_auditor_for_file(id: u32, address: &T::AccountId) -> bool {
        FileByID::<T>::get(id).auditors.iter().any(|x| x == address)
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