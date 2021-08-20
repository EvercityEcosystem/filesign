# Filesign

Filesign is a substrate pallet which allows to create and store files with different metadata on blockchain
## Main features:
- file versioning
- store file hashes for each version
- assign auditors to files
- provide file signing


Add the pallet to the [Substrate node template](https://github.com/substrate-developer-hub/substrate-node-template) pallets folder. 

Add the following snippets of code to the runtime/lib.rs:

```
pub use pallet_audit;

impl pallet_audit::Config for Runtime {
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        Audit: pallet_audit::{Pallet, Call, Storage},
	}
);
```

Add the following dependencies to runtime Cargo.toml:
```
[dependencies.pallet-audit]
default-features = false
path = '../pallets/audit'
version = '3.0.0'
```

Add to root folder Cargo.toml:
```
'pallets/audit',
```

Run:
```
cargo build --release
./target/release/node-template purge-chain --dev
./target/release/node-template --dev
```

Go to [extrinsics](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/extrinsics) for the locally running node 