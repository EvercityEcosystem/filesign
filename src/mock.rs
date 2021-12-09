#![allow(clippy::from_over_into)]

use crate as pallet_filesign;

use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::H256;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
		Filesign: pallet_filesign::{Module, Call, Storage, Event<T>},
	}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl pallet_filesign::Config for TestRuntime {
	type Event = Event;
	type Randomness = RandomnessCollectiveFlip;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap().into()
}

// Build genesis storage for event testing
pub fn new_test_ext_with_event() -> frame_support::sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// get and cut last event
#[allow(clippy::result_unit_err)] 
pub fn last_event() -> Result<Event, ()> {
	match System::events().pop() {
		Some(ev) => Ok(ev.event),
		None => Err(())
	}
}