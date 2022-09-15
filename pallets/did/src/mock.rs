use crate::{self as pallet_did, Config, Pallet};
use frame_support::{parameter_types, weights::Weight, traits::Everything};
use frame_system as system;
use pallet_timestamp as timestamp;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{testing::Header, traits::{BlakeTwo256, IdentityLookup}, Perbill, traits::ConstU32, BoundedVec};
use scale_info::TypeInfo;
use codec::{Encode, MaxEncodedLen};
use sp_runtime::traits::Get;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Did: pallet_did::{Pallet, Call, Storage, Event<T>},
    }
);

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const MaximumBlockWeight: Weight = 1024;
  pub const MaximumBlockLength: u32 = 2 * 1024;
  pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Call = Call;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

parameter_types! {
    #[derive(TypeInfo, MaxEncodedLen, Encode)]
	pub const MaxDelegateTypeLen: u32 = 64;
	#[derive(TypeInfo, MaxEncodedLen, Encode)]
	pub const MaxNameLen: u32 = 64;
	#[derive(TypeInfo, MaxEncodedLen, Encode)]
	pub const MaxValueLen: u32 = 64;
}

impl Config for Test {
    type Event = Event;
    type DelegateType = DelegateTypeProvider;
    type MaxDelegateTypeLen = MaxDelegateTypeLen;
    type MaxNameLen = MaxNameLen;
    type MaxValueLen = MaxValueLen;
    type Public = sr25519::Public;
    type Signature = sr25519::Signature;
    type Time = Timestamp;
    type WeightInfo = pallet_did::weights::SubstrateWeight<Self>;
}

pub struct DelegateTypeProvider;
impl Get<BoundedVec<u8, MaxDelegateTypeLen>> for DelegateTypeProvider {
    // Provide the default delegate type as a BoundedVec
    fn get() -> BoundedVec<u8, MaxDelegateTypeLen> {
        b"x25519VerificationKey2018".to_vec().try_into().expect("could not convert delegate type into boundedvec")
    }
}

pub type DID = Pallet<Test>;
// pub type System = system::Pallet<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn account_pair(s: &str) -> sr25519::Pair {
    sr25519::Pair::from_string(&format!("//{}", s), None).expect("static values are valid; qed")
}

pub fn account_key(s: &str) -> sr25519::Public {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}