//! Pallet Did pallet benchmarking
//!

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_runtime::traits::One;
use crate::Pallet as Did;

macro_rules! make_caller {
    ($T: ident) => {{
        let caller:$T::AccountId = whitelisted_caller();
        // let _ = $T::Currency::make_free_balance_be(&caller, BalanceOf::<$T>::max_value());
        caller
    }}
}

benchmarks! {
    add_delegate {
        let caller = make_caller!(T);
        let delegate:T::AccountId = account("delegate", 0, 0);
    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), delegate, BoundedVec::default(), Some(T::BlockNumber::one()))

    change_owner {
        let caller = make_caller!(T);
        let new_owner:T::AccountId = account("new_owner", 0, 0);

    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), new_owner)

    revoke_delegate {
        let caller = make_caller!(T);
        let delegate:T::AccountId = account("delegate", 0, 0);
        let _ = Did::<T>::add_delegate(RawOrigin::Signed(caller.clone()).into(), caller.clone(), delegate.clone(), BoundedVec::default(), None);
    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), BoundedVec::default(), delegate)

    add_attribute {
        let caller = make_caller!(T);
        let name : BoundedNameOf<T> = b"name1".to_vec().try_into().unwrap();
        let value : BoundedValueOf<T> = b"value1".to_vec().try_into().unwrap();
    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), name, value, Some(T::BlockNumber::one()))

    revoke_attribute {
        let caller = make_caller!(T);
        let name : BoundedNameOf<T> = b"name1".to_vec().try_into().unwrap();
        let value : BoundedValueOf<T> = b"value1".to_vec().try_into().unwrap();
        let _ = Did::<T>::add_attribute(RawOrigin::Signed(caller.clone()).into(), caller.clone(), name.clone(), value, None);
    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), name)

    delete_attribute {
        let caller = make_caller!(T);
        let name : BoundedNameOf<T> = b"name1".to_vec().try_into().unwrap();
        let value : BoundedValueOf<T> = b"value1".to_vec().try_into().unwrap();
        let _ = Did::<T>::add_attribute(RawOrigin::Signed(caller.clone()).into(), caller.clone(), name.clone(), value, None);
    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), name)
}
