// use core::convert::TryInto;
// use frame_support::storage::bounded_vec::BoundedVec;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn issue_grant_to_account() {
	new_test_ext().execute_with(|| {

		// Ensure the user can create profile
		assert_ok!(Grant::issue_grant(Origin::signed(1), 2 ));
	});
}

#[test]
fn ensure_funds_are_transfered() {
	new_test_ext().execute_with(|| {

		// Ensure the user can create profile
		assert_ok!(Grant::issue_grant(Origin::signed(1), 2 ));

        // assert_eq!(Balances::balance(&2), 1000);
	});
}