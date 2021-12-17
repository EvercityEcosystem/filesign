# Filesign

Filesign is a substrate pallet which allows to create and store files with different metadata on blockchain
## Main features:
- file versioning
- store file hashes for each version
- assign signers to files
- provide file signing


Add the pallet to the [Substrate node template](https://github.com/substrate-developer-hub/substrate-node-template) pallets folder. 

Add the following snippets of code to the runtime/lib.rs:

```
use pallet_evercity_filesign;

impl pallet_evercity_filesign::Config for Runtime {
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        EvercityFilesign: pallet_evercity_filesign::{ Module, Call, Storage, Event<T> },
	}
);
```

Add the following dependencies to runtime Cargo.toml:
```
[dependenciest]
pallet-evercity-filesign = { default-features = false, version = '0.1.3', git = 'https://github.com/EvercityEcosystem/filesign'}
```

Run:
```
cargo build --release
./target/release/node-template purge-chain --dev
./target/release/node-template --dev
```

Go to [extrinsics](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/extrinsics) for the locally running node 

works with JUL template

git clone -b v3.0.0+monthly-2021-07 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template