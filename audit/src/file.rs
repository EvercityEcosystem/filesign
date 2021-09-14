use fixed_hash::construct_fixed_hash;
use frame_support::{
    codec::{
        Decode, 
        Encode
    },
    dispatch::{
        Vec,
    }
};

use frame_support::sp_runtime::RuntimeDebug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct SigStruct<AccountId> {
    pub address: AccountId,
    pub signed: bool,
}


#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct VersionStruct<AccountId> {
    pub tag: Vec<u8>,
    pub filehash: H256,
    pub signatures: Vec<SigStruct<AccountId>>,
}


#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct FileStruct<AccountId> {
    pub owner: AccountId,
    pub id: u32,
    pub versions: Vec<VersionStruct<AccountId>>,
    pub auditors: Vec<AccountId>,
}

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
    fn check_sig_status(&self) -> bool where AccountId: PartialEq {
        let latest_version: &VersionStruct<AccountId> = self.versions.last().unwrap();   

        // !self.auditors.iter().any(|aud| latest_version.signatures.iter().any(|x| x.address == *aud))
        for aud in &self.auditors {
            if !latest_version.signatures.iter().any(|x| x.address == *aud){
                return false;
            }
        }
        true
    }
}