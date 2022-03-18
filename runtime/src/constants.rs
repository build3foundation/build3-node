//! A set of constant values used in substrate runtime.
use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
pub use node_primitives::{AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, Signature};
pub use sp_runtime::{create_runtime_str, Perbill};

/// Money matters.
pub mod currency {
	use node_primitives::Balance;
	// CHOY = the base number of indivisible units for balances (previously UNIT)
	pub const CHOY: Balance = 1_000_000_000_000;
	pub const MILLICENTS: Balance = 1_000_000_000;
	pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
	pub const DOLLARS: Balance = 100 * CENTS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
	}
	pub const EXISTENTIAL_DEPOSIT: Balance = MILLICENTS;
}

/// All things related to block time constants and configuration
pub mod block_time {
	pub use super::{BlockNumber, Perbill};
	use super::{Weight, WEIGHT_PER_SECOND};
	/// This determines the average expected block time that we are targeting.
	/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
	/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
	/// up by `pallet_aura` to implement `fn slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u64 = 6000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	//       Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

	/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
	/// This is used to limit the maximal weight of a single extrinsic.
	pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

	// Prints debug output of the `contracts` pallet to stdout if the node is
	// started with `-lruntime::contracts=debug`.
	pub const CONTRACTS_DEBUG_OUTPUT: bool = true;
}
