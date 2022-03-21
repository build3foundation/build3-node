//! The build3 runtime built from Substrate.
//! This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

/// FRAME crates
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU32, EnsureOneOf, KeyOwnerProofSystem},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
		DispatchClass, IdentityFee, Weight,
	},
};

use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};

/// Pallets
use pallet_contracts::weights::WeightInfo;

use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};

/// Substrate primitives
use sp_api::impl_runtime_apis;

use sp_consensus_aura::sr25519::AuthorityId as AuraId;

use sp_core::{crypto::KeyTypeId, OpaqueMetadata};

use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, NumberFor},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};

use sp_std::prelude::*;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

/// -------------------------^ PRIVATE IMPORTS ^ __________________________________
///
///
/// -------------------------\/ PUBLIC IMPORTS \/ _________________________________
pub use sp_version::RuntimeVersion;

/// Constant values used within the runtime.
pub mod constants;

pub use constants::{block_time::*, currency::*};

/// Node primitives
pub use node_primitives::{AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, Signature};

/// A few imports from a few pallets
pub use pallet_balances::Call as BalancesCall;

pub use pallet_timestamp::Call as TimestampCall;

pub use pallet_collective::{
	Call as CollectiveCall, MoreThanMajorityThenPrimeDefaultVote, RawOrigin as CollectiveOrigin,
};

use pallet_transaction_payment::CurrencyAdapter;

/// Complex storage builder stuff
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

/// # Opaque Types
/// These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub aura: Aura,
			pub grandpa: Grandpa,
		}
	}
}

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// To learn more about runtime versioning and what each of the following value means:
//   https://docs.substrate.io/v3/runtime/upgrades#runtime-versioning
#[sp_version::runtime_version]
/// The runtime version
pub const VERSION: RuntimeVersion = RuntimeVersion {
	/// The identified for the different Substrate runtimes.
	spec_name: create_runtime_str!("build3-node"),
	// The name of the implementation of the spec. This is of little
	// consequence for the node and serves only to differentiate code of
	// different implementation teams.
	impl_name: create_runtime_str!("build3-node"),
	authoring_version: 1,
	// Per convention: if the runtime behavior changes, increment spec_version
	// and set impl_version to 0. If only runtime
	// implementation changes and behavior does not, then leave spec_version as
	// is and increment impl_version.

	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

// Prepare the frame_system storage
parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;

	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();

	pub const SS58Prefix: u8 = 42;
}

/// Configure FRAME pallets to include in runtime.
impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = frame_support::traits::Everything;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Version of the runtime.
	type Version = Version;
	/// Converts a module to the index of the module in `construct_runtime!`.
	///
	/// This type is being generated by `construct_runtime!`.
	type PalletInfo = PalletInfo;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The set code logic, just the default since we're not a parachain.
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

/// A pallet which provides a randomness function
impl pallet_randomness_collective_flip::Config for Runtime {}

// Aura consensus storage
parameter_types! {
	pub const MaxAuthorities: u32 = 32;
}

/// Aura consensus configuration
impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

/// Configure an Origin requirement which must be eitehr half council vote or a
/// sudo key
type EnsureRootOrHalfCouncil = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

// Identity pallet storage
parameter_types! {
	pub const BasicDeposit: Balance = 10 * DOLLARS;       // 258 bytes on-chain
	pub const FieldDeposit: Balance = 250 * CENTS;        // 66 bytes on-chain
	pub const SubAccountDeposit: Balance = 2 * DOLLARS;   // 53 bytes on-chain
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

/// Identity pallet configuration.
/// The identity pallet is used to model engineering licensure. Eventually this
///will be made into a different instance (or pallet) with more customized
///features for engineering.
impl pallet_identity::Config for Runtime {
	type Event = Event;
	type Currency = Balances;

	/// The amount held on deposit for a registered identity.
	type BasicDeposit = BasicDeposit;

	/// The amount held on deposit per additional field for a registered identity.
	type FieldDeposit = FieldDeposit;

	/// The amount held on deposit for a registered subaccount. This should account for the fact
	/// that one storage item's value will increase by the size of an account ID, and there will be
	/// another trie item whose value is the size of an account ID plus 32 bytes.
	type SubAccountDeposit = SubAccountDeposit;

	/// The maximum number of sub-accounts allowed per identified account.
	type MaxSubAccounts = MaxSubAccounts;

	/// Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O
	/// required to access an identity, but can be pretty high.
	type MaxAdditionalFields = MaxAdditionalFields;

	/// Maxmimum number of registrars allowed in the system. Needed to bound the complexity
	/// of, e.g., updating judgements.
	type MaxRegistrars = MaxRegistrars;

	/// What to do with slashed funds.
	type Slashed = ();

	/// The origin which may forcibly set or remove a name. Root can always do this.
	/// Right now this is only the root. This will eventually be assigned to
	/// the council.
	type ForceOrigin = EnsureRootOrHalfCouncil;

	/// The origin which may add or remove registrars. Right now only set to
	/// root but can eventually be at the vote of a council.
	type RegistrarOrigin = EnsureRootOrHalfCouncil;

	/// Weight information for extrinsics in this pallet. Not yet configured.
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}

/// GRANDPA finality configuration.
impl pallet_grandpa::Config for Runtime {
	type Event = Event;
	type Call = Call;

	type KeyOwnerProofSystem = ();

	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;

	type HandleEquivocation = ();

	type WeightInfo = ();
	type MaxAuthorities = MaxAuthorities;
}

// Timestamp storage setting minimum period between blocks.
parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

/// Timestamp configuration.
impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

// Balances storage
parameter_types! {
	pub const ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
	pub const MaxLocks: u32 = 50;
}

/// Balances configuration
impl pallet_balances::Config for Runtime {
	/// The maximum number of locks that should exist on an account.
	/// Not strictly enforced, but used for weight estimation.
	type MaxLocks = MaxLocks;

	/// The maximum number of named reserves that can exist on an account.
	type MaxReserves = ();

	/// The id type for named reserves.
	type ReserveIdentifier = [u8; 8];

	/// The type for recording an account's balance.
	type Balance = Balance;

	/// The ubiquitous event type.
	type Event = Event;

	/// Handler for the unbalanced reduction when removing a dust account.
	type DustRemoval = ();

	/// The minimum amount required to keep an account open.
	type ExistentialDeposit = ExistentialDeposit;

	/// The means of storing the balances of an account.
	type AccountStore = System;

	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

// Council constant configurations
parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

// Council configuration
type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}

// Board of VA constant configurations
parameter_types! {
	pub const BoardVirginiaMotionDuration: BlockNumber = 5 * DAYS;
	pub const BoardVirginiaMaxProposals: u32 = 100;
	pub const BoardVirginiaMaxMembers: u32 = 100;
}

// Board of VA configuration
type BoardVirginiaCollective = pallet_collective::Instance2;
impl pallet_collective::Config<BoardVirginiaCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = BoardVirginiaMotionDuration;
	type MaxProposals = BoardVirginiaMaxProposals;
	type MaxMembers = BoardVirginiaMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}

// Transaction storage
parameter_types! {
	pub const TransactionByteFee: Balance = 1;
	pub OperationalFeeMultiplier: u8 = 5;
}

/// Transaction configuration
impl pallet_transaction_payment::Config for Runtime {
	/// Handler for withdrawing, refunding and depositing the transaction fee.
	/// Transaction fees are withdrawn before the transaction is executed.
	/// After the transaction was executed the transaction weight can be
	/// adjusted, depending on the used resources by the transaction. If the
	/// transaction weight is lower than expected, parts of the transaction fee
	/// might be refunded. In the end the fees can be deposited.
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;

	/// The fee to be paid for making a transaction; the per-byte portion.
	type TransactionByteFee = TransactionByteFee;

	/// A fee mulitplier for `Operational` extrinsics to compute "virtual tip" to boost their
	/// `priority`
	///
	/// This value is multipled by the `final_fee` to obtain a "virtual tip" that is later
	/// added to a tip component in regular `priority` calculations.
	/// It means that a `Normal` transaction can front-run a similarly-sized `Operational`
	/// extrinsic (with no tip), by including a tip value greater than the virtual tip.
	///
	/// ```rust,ignore
	/// // For `Normal`
	/// let priority = priority_calc(tip);
	///
	/// // For `Operational`
	/// let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
	/// let priority = priority_calc(tip + virtual_tip);
	/// ```
	///
	/// Note that since we use `final_fee` the multiplier applies also to the regular `tip`
	/// sent with the transaction. So, not only does the transaction get a priority bump based
	/// on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`
	/// transactions.
	type OperationalFeeMultiplier = OperationalFeeMultiplier;

	/// Convert a weight value into a deductible fee based on the currency type.
	type WeightToFee = IdentityFee<Balance>;

	/// Update the multiplier of the next block, based on the previous block's weight.
	type FeeMultiplierUpdate = ();
}

/// Adding root account to the runtime. This is only required if we decide to
/// keep root access with build3 which is not likely required.
impl pallet_sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}

// Contract storages
parameter_types! {
	pub const DepositPerItem: Balance = deposit(1, 0);
	pub const DepositPerByte: Balance = deposit(0, 1);
	// The lazy deletion runs inside on_initialize.
	pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
		RuntimeBlockWeights::get().max_block;
	// The weight needed for decoding the queue should be less or equal than a fifth
	// of the overall weight dedicated to the lazy deletion.
	pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
		)) / 5) as u32;
	pub Schedule: pallet_contracts::Schedule<Runtime> = {
		let mut schedule = pallet_contracts::Schedule::<Runtime>::default();
		// We decided to **temporarily* increase the default allowed contract size here
		// (the default is `128 * 1024`).
		//
		// Our reasoning is that a number of people ran into `CodeTooLarge` when trying
		// to deploy their contracts. We are currently introducing a number of optimizations
		// into ink! which should bring the contract sizes lower. In the meantime we don't
		// want to pose additional friction on developers.
		schedule.limits.code_len = 256 * 1024;
		schedule
	};
}

/// Contracts pallet configuration
impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type Event = Event;
	type Call = Call;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `Call` structure itself
	/// is not allowed to change the indices of existing pallets, too.
	type CallFilter = frame_support::traits::Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type DeletionQueueDepth = DeletionQueueDepth;
	type DeletionWeightLimit = DeletionWeightLimit;
	type Schedule = Schedule;
	type CallStack = [pallet_contracts::Frame<Self>; 31];
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		Contracts: pallet_contracts,
		Identity: pallet_identity,
		Council: pallet_collective::<Instance1>,
		BoardVirginia: pallet_collective::<Instance2>,
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// The SignedExtension to the basic transaction logic.
pub type Extra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
/// TODO: convert to checked extrinsics prior to production.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, Extra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	/// Implement Aura consensus by setting slot durationg and storing the
	/// authorities.
	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}


	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	/// GRANPA finality primitives
	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			_authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	/// Runtime API definitions giving nodes System access methods
	/// Returning the account nonce index (aka the account transaction counter)
	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	/// I'm honestly not sure what is happening here....KMS
	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	// Configure benchmark
	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmark!(list, extra, frame_benchmarking, BaselineBench::<Runtime>);
			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_balances, Balances);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			list_benchmark!(list, extra, pallet_identity,  Identity);
			list_benchmark!(list, extra, pallet_collective,  Council);
			list_benchmark!(list, extra, pallet_collective,  BoardVirginia);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			add_benchmark!(params, batches, frame_benchmarking, BaselineBench::<Runtime>);
			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pallet_identity, Identity);
			add_benchmark!(params, batches, pallet_collective, Council);
			add_benchmark!(params, bathces, pallet_collective, BoardVirginia);

			Ok(batches)
		}
	}

	// Giving System access to the contracts so they can be called,
	// instantiated, updloaded, and have their storage retrieved.
	impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
		for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: u64,
			storage_deposit_limit: Option<Balance>,
			input_data: Vec<u8>,
		) -> pallet_contracts_primitives::ContractExecResult<Balance> {
			Contracts::bare_call(origin, dest, value, gas_limit, storage_deposit_limit, input_data, CONTRACTS_DEBUG_OUTPUT)
		}

		fn instantiate(
			origin: AccountId,
			value: Balance,
			gas_limit: u64,
			storage_deposit_limit: Option<Balance>,
			code: pallet_contracts_primitives::Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
		{
			Contracts::bare_instantiate(origin, value, gas_limit, storage_deposit_limit, code, data, salt, CONTRACTS_DEBUG_OUTPUT)
		}

		fn upload_code(
			origin: AccountId,
			code: Vec<u8>,
			storage_deposit_limit: Option<Balance>,
		) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance>
		{
			Contracts::bare_upload_code(origin, code, storage_deposit_limit)
		}

		fn get_storage(
			address: AccountId,
			key: [u8; 32],
		) -> pallet_contracts_primitives::GetStorageResult {
			Contracts::get_storage(address, key)
		}
	}
}
