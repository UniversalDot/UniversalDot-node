// use core::convert::TryInto;
// use frame_support::storage::bounded_vec::BoundedVec;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{Hooks};

fn run_to_block(n: u64) {
	Grant::on_finalize(System::block_number());
	for b in (System::block_number() + 1)..=n {
		next_block(b);
		if b != n {
			Grant::on_finalize(System::block_number());
		}
	}
}

fn next_block(n: u64) {
	System::set_block_number(n);
	Grant::on_initialize(n);
}

#[test]
fn accounts_can_request_a_grant() {
	new_test_ext().execute_with(|| {

		// Ensure the user can create profile
		assert_ok!(Grant::request_grant(Origin::signed(1), 2 ));

	});
}

#[test]
fn ensure_funds_can_be_transfered() {
	new_test_ext().execute_with(|| {

		// Account starts with balance of 10
		assert_eq!(Balances::free_balance(&2), 10);

		// Ensure the user can create profile
		assert_ok!(Grant::transfer_funds(Origin::signed(1), 2 , 1));

		// User balance has increased by ammount
        assert_eq!(Balances::free_balance(&2), 11);
	});
}

#[test]
fn ensure_exact_amount_is_transfered() {
	new_test_ext().execute_with(|| {

		// Account starts with balance of 10
		assert_eq!(Balances::free_balance(&2), 10);

		// Ensure the user can create profile
		assert_ok!(Grant::transfer_funds(Origin::signed(1), 2 , 2));

		// Ensure user balance is not equal to 11 since we increased 10 + 2
        assert_ne!(Balances::free_balance(&2), 11);

	});
}

#[test]
fn throw_error_when_granting_to_self() {
	new_test_ext().execute_with(|| {

		// Ensure treasury can't issue funds to self
		assert_noop!(Grant::transfer_funds(Origin::signed(1), 1 , 3 ), Error::<Test>::CantGrantToSelf);
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
fn winner_can_be_selected() {
	new_test_ext().execute_with(|| {

		// Request grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 2 ));

		// go to later block 
		run_to_block(4);

		// Ensure the winner is the only account that requested
		assert_eq!(Grant::winner(), 2);

	});
}

#[test]
fn winner_can_be_selected_per_block() {
	new_test_ext().execute_with(|| {
		
		// Request grant and run to block
		assert_ok!(Grant::request_grant(Origin::signed(1), 2 ));
		run_to_block(2);

		// Ensure we have selected the correct winner
		assert_eq!(Grant::winner(), 2);

		// Request additional grant for different block
		assert_ok!(Grant::request_grant(Origin::signed(1), 3 ));
		run_to_block(5);

		// Ensure we have the coorect winner
		assert_eq!(Grant::winner(), 3);

	});
}