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

construct_fixed_hash! {
    /// 256 bit hash type for signing files
    #[derive(Encode, Decode)]
    pub struct H256(32);
}

pub type FileId = [u8; 16];

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct SigStruct<AccountId> {
    pub address: AccountId,
    pub signed: bool,
}

#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct VersionStruct<AccountId> {
    pub tag: Vec<u8>,
    pub filehash: H256,
    pub signatures: Vec<SigStruct<AccountId>>,
}

/// Main File Domain
#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
pub struct FileStruct<AccountId> where AccountId: PartialEq {
    pub owner: AccountId,
    pub id: FileId,
    pub versions: Vec<VersionStruct<AccountId>>,
    pub signers: Vec<AccountId>,
}

#[allow(clippy::vec_init_then_push)]
impl<AccountId> FileStruct<AccountId> where AccountId: PartialEq {
    // Constructor for file
    pub fn new(owner: AccountId, id: FileId, tag: Vec<u8>, filehash: &H256) -> Self {
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
            signers: Vec::new(),
        }
    }

    // Add a sign to last version of file
    pub fn sign_latest_version(&mut self, caller: AccountId) {
        let latest_version = self.versions.last_mut().unwrap();

        // here check if has already signed
        match latest_version.signatures.iter().position(|sig| sig.address == caller) {
            Some(_) => {/*new logic can be made in future here*/},
            None => {
                latest_version.signatures.push(SigStruct{address: caller, signed: true});         
            }
        }
    }

    // Assigns a new signer to a file
    pub fn assign_signer_to_file (&mut self, signer: AccountId) {
        if !self.signers.iter().any(|x| *x == signer){
            self.signers.push(signer);
        }    
    }

    // Removes signer from file
    #[allow(clippy::result_unit_err)]
    pub fn delete_signer_from_file (&mut self, signer: AccountId) -> Result<(), ()> {
        let index = match self.signers.iter().position(|a| a == &signer) {
            Some(i) => i,
            None => return Err(())
        };
        self.signers.remove(index);
        Ok(())
    }
}

// pub fn generate_file_id() -> FileId {
//     // *uuid::Uuid::new_v4().as_bytes()
//     use rand::{
//         self, Rng,
//         distributions::{Distribution, Uniform},
//     };

//     let mut rng = rand::thread_rng().gen::<[u8; 16]>();
//     rng
// }