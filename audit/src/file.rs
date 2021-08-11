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

type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

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
    fn create_file() {
    }
}