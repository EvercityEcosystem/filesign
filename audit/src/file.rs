use frame_support::{
    codec::{Decode, Encode},
    dispatch::{Vec},
    sp_runtime::{
        traits::{Verify, IdentifyAccount,},
        RuntimeDebug,
        MultiSignature,
    },
    sp_std::cmp::{Eq, PartialEq},
};

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
    pub id: u32,
    pub versions: Vec<VersionStruct<AccountId>>,
    pub auditors: Vec<AccountId>,
}

pub type File<T> = FileStruct<
    <T as frame_system::Config>::AccountId,
>;

impl<AccountId> FileStruct<AccountId> {
    fn create_file() -> FileStruct<AccountId> {
        FileStruct {
            id: 0,
            versions: Vec::new(),
            auditors: Vec::new(),
        }
    }

    fn assign_auditor(
        mut file: FileStruct<AccountId>, 
        new_auditor: AccountId
    ) -> FileStruct<AccountId> {
        file.auditors.push(new_auditor);
        file
    }

    fn delete_auditor(
        mut file: FileStruct<AccountId>, 
        auditor: AccountId
    ) -> FileStruct<AccountId> where AccountId: PartialEq {
        let index = file.auditors.iter().position(|x| x == &auditor).unwrap();
        file.auditors.remove(index);
        file
    }

    fn check_sig_status(
        file: FileStruct<AccountId>
    ) -> bool {
        let latest_version: &VersionStruct<AccountId> = file.versions.last().unwrap();
        let mut iter = latest_version.signatures.iter().filter(|x| x.signed == false);
        iter.count() == 0
    }

    fn get_info_by_tag() {

    }

    fn sign_latest_version() {

    }
}