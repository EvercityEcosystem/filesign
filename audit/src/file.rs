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


/// Main File Domain
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct FileStruct<AccountId> where AccountId: PartialEq {
    pub owner: AccountId,
    pub id: u32,
    pub versions: Vec<VersionStruct<AccountId>>,
    pub auditors: Vec<AccountId>,
}

impl<AccountId> FileStruct<AccountId> where AccountId: PartialEq {
    // Constructor
    pub fn new(owner: AccountId, id: u32, tag: Vec<u8>, filehash: &H256) -> Self {
        let empty_vec = Vec::new();
        let latest_version = VersionStruct {
            tag,
            filehash: *filehash,
            signatures: empty_vec,
        };

        let mut versions = Vec::with_capacity(1);
        versions.push(latest_version);

        FileStruct {
            owner,
            id,
            versions,
            auditors: Vec::new(),
        }
    }

    // Assigns a new auditor to a file
    pub fn assign_auditor_to_file (&mut self, auditor: AccountId, caller: AccountId) {
        if !self.auditors.iter().any(|x| *x == caller){
            self.auditors.push(auditor);
        }    
    }

    // Removes auditor from file
    pub fn delete_auditor_from_file (&mut self, auditor: AccountId) -> Result<(), ()> {
        let index = match self.auditors.iter().position(|a| a == &auditor) {
            Some(i) => i,
            None => return Err(())
        };
        self.auditors.remove(index);
        Ok(())
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

    pub fn get_auditors(&self) -> &Vec<AccountId> {
        &self.auditors
    }
}