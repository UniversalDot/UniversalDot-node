use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{Hooks};


// <<<<<<<<<<<<<<<<<< Helper functions and constants >>>>>>>>>>>>>>>>>>


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


// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< TESTS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

#[test]
fn accounts_can_request_a_grant() {
	new_test_ext().execute_with(|| {

		// Ensure we can request grants
		assert_ok!(Grant::request_grant(Origin::signed(1), 7 ));

	});
}

#[test]
fn requests_can_be_counted() {
	new_test_ext().execute_with(|| {

		// Ensure we can request grants
		assert_ok!(Grant::request_grant(Origin::signed(1), 7 ));
		assert_ok!(Grant::request_grant(Origin::signed(1), 6 ));
		
		// Ensure we can count requests
		assert_eq!(Grant::requesters_count(), 2);

	});
}

#[test]
fn ensure_funds_can_be_transfered() {
	new_test_ext().execute_with(|| {

		// Account starts with balance of 10
		assert_eq!(Balances::free_balance(&2), 10);

		// Ensure we can transfer
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

		// Ensure we can transfer
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

		// Go to later block
		run_to_block(7);

		// Ensure a user can request a grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 5 ));

        // Find the request
        let requests = Grant::storage_requesters(5).expect("should find requests");

        // Ensure we can access the storage requests
        assert_eq!(requests.owner, 5);
        assert_eq!(requests.balance,Some(0));
		assert_eq!(requests.block_number, 7);

	});
}

#[test]
fn ensure_requests_can_be_made_by_separate_accounts() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 5 ));
        assert_ok!(Grant::request_grant(Origin::signed(1), 6 ));

        // Find the request
        let request1 = Grant::storage_requesters(5).expect("should find requests");
        let request2 = Grant::storage_requesters(6).expect("should find requests");

        // Ensure we can access the storage requests
        assert_eq!(request1.owner, 5);
        assert_eq!(request2.owner, 6);

	});
}

#[test]
fn ensure_only_users_with_no_balance_can_request_grants() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		assert_ok!(Grant::request_grant(Origin::signed(3), 7 ));
		assert_eq!(Balances::free_balance(7), 0);
        
        // Ensure only empty balance can make requests
        assert_noop!(Grant::request_grant(Origin::signed(3), 1), Error::<Test>::NonEmptyBalance);

	});
}

#[test]
fn winner_can_be_selected() {
	new_test_ext().execute_with(|| {

		// Request grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 7 ));

		// go to later block 
		run_to_block(4);

		// Ensure the winner is the only account that requested
		assert_eq!(Grant::winner().unwrap(), 7);

	});
}

#[test]
fn winner_can_be_queried_by_anyone() {
	new_test_ext().execute_with(|| {

		// Request grant
		assert_ok!(Grant::request_grant(Origin::signed(1), 7 ));

		// go to later block 
		run_to_block(2);

		// Ensure the winner is the only account that requested
		assert_eq!(Grant::winner().unwrap(), 7);
		assert_ok!(Grant::winner_is(Origin::signed(1)));

	});
}


#[test]
fn winner_can_be_selected_per_block() {
	new_test_ext().execute_with(|| {
		
		// Request grant and run to block
		assert_ok!(Grant::request_grant(Origin::signed(1), 5 ));
		run_to_block(2);

		// Ensure we have selected the correct winner
		assert_eq!(Grant::winner().unwrap(), 5);
		
		// Request additional grant for different block
		assert_ok!(Grant::request_grant(Origin::signed(1), 8 ));
		assert_ok!(Grant::request_grant(Origin::signed(1), 7 ));
		
		run_to_block(5);

		// Ensure we have the correct winner (Repeatable randomness?)
		assert_eq!(Grant::winner().unwrap(), 7);

	});
}

#[test]
fn winner_can_be_recieve_grant_reward() {
	new_test_ext().execute_with(|| {

		// Add balance to grant treasury account
		Balances::mutate_account(&Grant::account_id(), |balance| {
			balance.free = 100;
		});
		let treasury = Balances::free_balance(Grant::account_id());
		assert_eq!(treasury, 100);
		
		// Check initial account getting grant is zero.
		assert_eq!(Balances::free_balance(5), 0);
		
		// Request grant and run to block
		assert_ok!(Grant::request_grant(Origin::signed(1), 5));
		run_to_block(2);

		// Ensure we have selected the correct winner
		assert_eq!(Grant::winner().unwrap(), 5);

		//Ensure money is tranfered todo:: look for minimum balance
		assert!(Balances::free_balance(5) == 100 - 1);
	});
}
