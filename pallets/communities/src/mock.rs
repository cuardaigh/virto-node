use crate as communities;
use frame_support::{
	parameter_types,
	traits::{ConstU32, Contains, Everything},
};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Percent,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type Balance = u128;

pub type AccountId = u8;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
		Payments: orml_payments::{Pallet, Call, Storage, Event<T>},
		Communities: communities::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: u32| -> Balance {
		0u128
	};
}
parameter_types! {
	pub const MaxLocks: u32 = 50;
}
pub type ReserveIdentifier = [u8; 8];

pub struct MockDustRemovalWhitelist;
impl Contains<AccountId> for MockDustRemovalWhitelist {
	fn contains(_a: &AccountId) -> bool {
		false
	}
}

impl orml_tokens::Config for Test {
	type Amount = i64;
	type Balance = Balance;
	type CurrencyId = u32;
	type Event = Event;
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = MockDustRemovalWhitelist;
	type MaxReserves = ConstU32<2>;
	type ReserveIdentifier = ReserveIdentifier;
}

pub struct VirtoDisputeResolver;
impl orml_payments::DisputeResolver<AccountId> for VirtoDisputeResolver {
	fn get_resolver_account() -> AccountId {
		1
	}
}

parameter_types! {
	pub const IncentivePercentage: Percent = Percent::from_percent(10);
	pub const MaxRemarkLength: u32 = 50;
	pub const FeeRecipientLimit : u32 = 5;
	// 1hr buffer period (60*60)/6
	pub const CancelBufferBlockLength: u64 = 600;
	pub const MaxScheduledTaskListLength : u32 = 20;
}

impl orml_payments::Config for Test {
	type Event = Event;
	type Asset = Tokens;
	type DisputeResolver = VirtoDisputeResolver;
	type IncentivePercentage = IncentivePercentage;
	type FeeHandler = communities::CommunityFeeHandler;
	type MaxRemarkLength = MaxRemarkLength;
	type FeeRecipientLimit = FeeRecipientLimit;
	type CancelBufferBlockLength = CancelBufferBlockLength;
	type MaxScheduledTaskListLength = MaxScheduledTaskListLength;
	type WeightInfo = ();
}

impl communities::Config for Test {
	type Event = Event;
	type Asset = Tokens;
	type MaxDomainNameSize = ConstU32<10>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut ext: sp_io::TestExternalities = t.into();
	// need to set block number to 1 to test events
	ext.execute_with(|| System::set_block_number(1));
	ext
}