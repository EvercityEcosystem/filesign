#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    decl_error, 
    decl_event, 
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
    pallet_prelude::*
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

pub type FileHash = Vec<u8>;

pub struct AccountId([u8; 32]);
impl PartialEq for AccountId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct SigStruct<AccountId> {
    pub address: AccountId,
    pub signed: bool,
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct VersionStruct<AccountId> {
    pub tag: Vec<u8>,
    pub filehash: u64,
    pub signatures: Vec<SigStruct<AccountId>>,
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct FileStruct<AccountId> {
    pub owner: AccountId,
    pub id: u32,
    pub versions: Vec<VersionStruct<AccountId>>,
    pub auditors: Vec<AccountId>,
}



pub type FileStructOf<T> = FileStruct<
    <T as frame_system::Config>::AccountId,
>;

impl<AccountId> FileStruct<AccountId> {
    // Assigns a new auditor to a file
    fn assign_auditor_to_file (
        mut file: FileStruct<AccountId>, 
        new_auditor: AccountId
    ) -> FileStruct<AccountId> {
        file.auditors.push(new_auditor);
        file
    }

    // Removes auditor from file
    fn delete_auditor_from_file (
        mut file: FileStruct<AccountId>, 
        auditor: AccountId
    ) -> FileStruct<AccountId> where AccountId: PartialEq {
        let index = file.auditors.iter().position(|x| x == &auditor).unwrap();
        file.auditors.remove(index);
        file
    }

    // Asserts that the latest version of file has no missing signatures from auditors
    fn check_sig_status(
        file: FileStruct<AccountId>
    ) -> bool {
        let latest_version: &VersionStruct<AccountId> = file.versions.last().unwrap();
        let iter = latest_version.signatures.iter().filter(|x| x.signed == false);
        iter.count() == 0
    }

}

pub trait Config: frame_system::Config {
    
}


decl_storage! {
    trait Store for Module<T: Config> as Audit {
        /// Storage map for file IDs
        FileByID
            get(fn file_by_id):
            map hasher(blake2_128_concat) u32 => FileStruct<T::AccountId>;

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
                    let file = file_by_id.clone();
                    let latest_version = file.versions.last().unwrap();
                    let index = latest_version.signatures.iter().position(|sig| sig.address == caller).unwrap();
                    let mut updated_latest_version = latest_version.clone();
                    updated_latest_version.signatures[index].signed = true;
                    file_by_id.versions.pop();
                    file_by_id.versions.push(updated_latest_version);
                Ok(())
                })?;
		}

        #[weight = 10_000]
        pub fn create_new_file(origin, tag: Vec<u8>, filehash: u64) {
            let caller = ensure_signed(origin)?;
            
            let empty_vec: Vec<SigStruct<AccountId>> = Vec::new();
            let latest_version = VersionStruct {
                tag,
                filehash,
                signatures: empty_vec,
            };

            // Update last created file ID
            let last_id = LastID::get();
            let new_id = last_id + 1;

            let new_file = FileStruct {
                owner: caller,
                id: last_id,
                versions: Vec::new(),
                auditors: Vec::new(),
            };

            <FileByID<T>>::insert(new_id, new_file);
            LastID::mutate(|x| *x += 1);

        }

        #[weight = 10_000]
        pub fn get_info_by_tag(origin, id: u32, tag: Vec<u8>) // -> VersionStruct<<T as frame_system::Config>::AccountId> 
        {
            let file = FileByID::<T>::get(id);
            let index = file.versions.iter().position(|v| v.tag == tag).unwrap();
            // TODO: return file.versions[index]
        }
        
        #[weight = 10_000]
        pub fn delete_auditor(origin, id: u32, auditor: T::AccountId) {
            let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_owner_for_file(id, caller), Error::<T>::AddressNotOwner);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    let mut file = file_by_id.clone();
                    let mut current_auditors = file.auditors;
                    let index = current_auditors.iter().position(|a| a == &auditor).unwrap();
                    current_auditors.remove(index);
                    let mut updated_auditors = current_auditors;
                    file.auditors = updated_auditors;
                    *file_by_id = file;
                Ok(())
                })?;
        }

        #[weight = 10_000]
        pub fn assign_auditor(origin, id: u32, auditor: T::AccountId) {
            let caller = ensure_signed(origin)?;
            ensure!(Self::address_is_owner_for_file(id, caller), Error::<T>::AddressNotOwner);

            FileByID::<T>::try_mutate(
                id, |file_by_id| -> DispatchResult {
                    let mut file = file_by_id.clone(); // TODO assert that the auditor is not already an auditor
                    file.auditors.push(auditor);
                    *file_by_id = file;
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
    pub fn address_is_owner_for_file(id: u32, address: T::AccountId) -> bool {
        FileByID::<T>::get(id).owner == address
    }
}