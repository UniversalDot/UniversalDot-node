use crate as pallet_grant;
use frame_system as system;
use sp_core::{
	H256,
	sr25519
};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
	traits::ConstU32
};
use frame_support_test::TestRandomness;
use frame_support::{
	PalletId,
	once_cell::sync::Lazy,
	parameter_types,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = sr25519::Public;
pub type Balance = u64;

const EXISTENTIAL_DEPOSIT: u64 = 0;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Grant: pallet_grant::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const LotteryPalletId: PalletId = PalletId(*b"py/lotto");
}

impl system::Config for Test {
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = AccountId;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub GrantAmount: Balance = 10;
	pub static TreasuryAccount: AccountId = *Lazy::new(||{sr25519::Public::from_raw([100u8; 32])});
	pub const MaxGenerateRandom: u32 = 5;
}

impl pallet_grant::Config for Test {
	type Event = Event;
	type Currency =  Balances;
	type WeightInfo = ();
	type PalletId = LotteryPalletId;
	type Randomness = TestRandomness<Self>;
	type TreasuryAccount = TreasuryAccount;
	type GrantAmount = GrantAmount;
	type MaxGenerateRandom = MaxGenerateRandom;
	type ExistentialDeposit = ExistentialDeposit;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {

}

pub static ALICE : Lazy<sr25519::Public> = Lazy::new(||{sr25519::Public::from_raw([1u8; 32])});
pub static BOB : Lazy<sr25519::Public> = Lazy::new(||{sr25519::Public::from_raw([2u8; 32])});
pub static TED : Lazy<sr25519::Public> = Lazy::new(||{sr25519::Public::from_raw([10u8; 32])});
pub static JOHN_EX : Lazy<sr25519::Public> = Lazy::new(||{sr25519::Public::from_raw([11u8; 32])});
pub static PETE_EX : Lazy<sr25519::Public> = Lazy::new(||{sr25519::Public::from_raw([12u8; 32])});
pub static SIMON_EX : Lazy<sr25519::Public> = Lazy::new(||{sr25519::Public::from_raw([13u8; 32])});


// Build genesis storage according to the mock runtime.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	GenesisConfig {
		balances: BalancesConfig {
			balances: vec![(*ALICE,  10), (*BOB,  10), (*TED, 10), (*JOHN_EX, EXISTENTIAL_DEPOSIT), (*PETE_EX, EXISTENTIAL_DEPOSIT), (*SIMON_EX, EXISTENTIAL_DEPOSIT) ]
		},
		..Default::default()
	}
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
