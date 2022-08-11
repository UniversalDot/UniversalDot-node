use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

/// Attributes or properties that make an identity.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Attribute<BlockNumber, Moment, BoundedName, BoundedValue> {
    pub name: BoundedName,
    pub value: BoundedValue,
    pub validity: BlockNumber,
    pub creation: Moment,
    pub nonce: u64,
}

pub type AttributedId<BlockNumber, Moment, BoundedName, BoundedValue> = (Attribute<BlockNumber, Moment, BoundedName, BoundedValue>, [u8; 32]);

/// Off-chain signed transaction.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AttributeTransaction<Signature, AccountId, BoundedName, BoundedValue> {
    pub signature: Signature,
    pub name: BoundedName,
    pub value: BoundedValue,
    pub validity: u32,
    pub signer: AccountId,
    pub identity: AccountId,
}
