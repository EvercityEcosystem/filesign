#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::Vec,
    sp_runtime::{
        traits::{
            Hash,
        }
    },
    sp_std::cmp::{Eq, PartialEq},
    sp_std::result::Result,
    traits::Get,
};
use frame_system::{ensure_signed, pallet_prelude::*};

pub use file::{FileStruct, VersionStruct, SigStruct, File};
pub mod file;

pub trait Config: frame_system::Config {
    
}


decl_storage! {
    trait Store for Module<T: Config> as Audit {
        /// Storage map for file IDs
        FileByID
            get(fn file_by_id):
            map hasher(blake2_128_concat) u32 => FileStruct<T::AccountId>;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        
    }
}