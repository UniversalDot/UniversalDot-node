// use core::convert::TryInto;
// use frame_support::storage::bounded_vec::BoundedVec;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


// #[test]
// fn issue_grant_to_account() {
// 	new_test_ext().execute_with(|| {

// 		// Ensure the user can create profile
// 		assert_ok!(Grant::issue_grant(Origin::signed(1), 2 ));
// 	});
// }

// #[test]
// fn ensure_funds_are_transfered() {
// 	new_test_ext().execute_with(|| {

// 		// Ensure the user can create profile
// 		assert_ok!(Grant::issue_grant(Origin::signed(1), 2 ));

//         assert_eq!(Balances::free_balance(&2), 11);
// 	});
// }

#[test]
fn accounts_can_request_a_grant() {
	new_test_ext().execute_with(|| {

		// Ensure the user can create profile
		assert_ok!(Grant::request_grant(Origin::signed(1), 2 ));

        
	});
}

#[test]
fn throw_error_when_granting_to_self() {
	new_test_ext().execute_with(|| {

		// Ensure the user can create profile
		// assert_noop!(Grant::request_grant(Origin::signed(1), 1 , 3 ), Error::<Test>::CantGrantToSelf);
	});
}

#[test]
fn ensure_request_is_stored() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 1 ));

        // Find the request
        let requests = Grant::storage_requesters(1).expect("should find requests");

        // Ensure we can access the storage requests
        assert_eq!(requests.owner, 1);
        assert_eq!(requests.balance,Some(10));

	});
}

#[test]
fn ensure_requests_can_be_made_by_separate_accounts() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 1 ));
        assert_ok!(Grant::request_grant(Origin::signed(1), 2 ));

        // Find the request
        let request1 = Grant::storage_requesters(1).expect("should find requests");
        let request2 = Grant::storage_requesters(2).expect("should find requests");

        // Ensure we can access the storage requests
        assert_eq!(request1.owner, 1);
        assert_eq!(request2.owner, 2);

	});
}

#[test]
fn ensure_only_users_with_no_balance_can_request_grants() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		// assert_ok!(Grant::request_grant(Origin::signed(3), 3 ));
        
        // Ensure only empty balance can make requests
        // assert_noop!(Grant::request_grant(Origin::signed(1), 3 ), Error::<Test>::NonEmptyBalance);

	});
}

#[test]
fn winner_is_selected() {
	new_test_ext().execute_with(|| {

		assert_ok!(Grant::winner_is(Origin::signed(3)));

	});
}