use crate::types::AttributedId;

use frame_support::dispatch::DispatchResult;

pub trait Did<AccountId, BlockNumber, Moment, Signature, BoundedName, BoundedValue, BoundedDelegateType> {
    fn is_owner(identity: &AccountId, actual_owner: &AccountId) -> DispatchResult;
    fn identity_owner(identity: &AccountId) -> AccountId;
    fn valid_delegate(
        identity: &AccountId,
        delegate_type: &BoundedDelegateType,
        delegate: &AccountId,
    ) -> DispatchResult;
    fn valid_listed_delegate(
        identity: &AccountId,
        delegate_type: &BoundedDelegateType,
        delegate: &AccountId,
    ) -> DispatchResult;
    fn create_delegate(
        who: &AccountId,
        identity: &AccountId,
        delegate: &AccountId,
        delegate_type: &BoundedDelegateType,
        valid_for: Option<BlockNumber>,
    ) -> DispatchResult;
    fn check_signature(signature: &Signature, msg: &[u8], signer: &AccountId) -> DispatchResult;
    fn valid_signer(
        identity: &AccountId,
        signature: &Signature,
        msg: &[u8],
        signer: &AccountId,
    ) -> DispatchResult;
    fn create_attribute(
        who: &AccountId,
        identity: &AccountId,
        name: &BoundedName,
        value: &BoundedValue,
        valid_for: Option<BlockNumber>,
    ) -> DispatchResult;
    fn reset_attribute(who: AccountId, identity: &AccountId, name: &BoundedName) -> DispatchResult;
    fn valid_attribute(identity: &AccountId, name: &BoundedName, value: &BoundedValue) -> DispatchResult;
    fn attribute_and_id(identity: &AccountId, name: &BoundedName) -> Option<AttributedId<BlockNumber, Moment, BoundedName, BoundedValue>>;
}