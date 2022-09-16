// Ripped from polkadot/common/src/impls.rs
use frame_support::traits::{Imbalance, OnUnbalanced};
use pallet_balances::NegativeImbalance;

/// Logic for the author to get a portion of fees.
//pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);
//impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
//where
//	R: pallet_balances::Config + pallet_authorship::Config,
//{
//	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
//		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
//			<pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
//		}
//	}
//}

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_treasury::Config ,
	pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 50% to treasury, 50% is dropped.
			let mut split = fees.ration(50, 50);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% is dropped.
				tips.merge_into(&mut split.1);
			}
			use pallet_treasury::Pallet as Treasury;
			
			// Send to treasury.
			<Treasury<R> as OnUnbalanced<_>>::on_unbalanced(split.0);

			// Drop the rest.
			drop(split.1);
		}
	}
}