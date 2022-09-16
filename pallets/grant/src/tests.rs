use crate::{mock::*, Error, Config, StorageRequesters, RequestersCount};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::{Hooks};
use crate::Pallet as PalletGrant;


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

fn treasury_account() -> AccountId {
	<Test as Config>::TreasuryAccount::get()
}

fn fund_treasury(u: u64) {
	let _ = Balances::mutate_account(&treasury_account() ,|amount| {
		amount.free += u
	});	
}

fn grant_amount() -> Balance {
	<Test as Config>::GrantAmount::get()
}

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< TESTS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

#[test]
fn accounts_can_request_a_grant() {
	new_test_ext().execute_with(|| {

		// Ensure we can request grants with empty balance.
		dbg!(Balances::free_balance(*JOHN_EX));
		dbg!(<Test as Config>::ExistentialDeposit::get());

		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));

	});
}

#[test]
fn requests_can_be_counted() {
	new_test_ext().execute_with(|| {

		// Ensure we can request grants with empty balance
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));
		assert_ok!(Grant::request_grant(Origin::signed(*PETE_EX)));
		
		// Ensure we can count requests
		assert_eq!(Grant::requesters_count(), 2);

	});
}

#[test]
fn ensure_funds_can_be_transfered() {
	new_test_ext().execute_with(|| {

		// Account starts with balance of 10
		fund_treasury(100_000);

		let init_user_balance = Balances::free_balance(*ALICE);
		let init_treasry_balance = Balances::free_balance(&treasury_account());
		let amount = 1;
		// Ensure we can transfer
		assert_ok!(Grant::transfer_to_treasury(Origin::signed(*ALICE), amount));

		// User balance has decreaed by amount.
        assert_eq!(Balances::free_balance(&*ALICE), init_user_balance - amount);

		// Treasury balance has increased by amount.
		assert!(Balances::free_balance(treasury_account()) == init_treasry_balance + amount);
	});
}

#[test]
fn throw_error_when_granting_to_self() {
	new_test_ext().execute_with(|| {

		fund_treasury(100_000u64);
		// Ensure treasury can't issue funds to self
		assert_noop!(Grant::transfer_to_treasury(Origin::signed(treasury_account()), 100), Error::<Test>::CantGrantToSelf);
	});
}

#[test]
fn ensure_request_is_stored() {
	new_test_ext().execute_with(|| {

		// Go to later block
		run_to_block(7);

		// Ensure a user can request a grant with no balance;
		assert_ok!(Grant::request_grant(Origin::signed(*PETE_EX)));

        // Find the request
        let requests = Grant::storage_requesters(*PETE_EX).expect("should find requests");

        // Ensure we can access the storage requests
        assert_eq!(requests.owner, *PETE_EX);
		assert_eq!(requests.block_number, 7);

	});
}

#[test]
fn ensure_requests_can_be_made_by_separate_accounts() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		assert_ok!(Grant::request_grant(Origin::signed(*PETE_EX)));
        assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));

        // Find the request
        let request1 = Grant::storage_requesters(*PETE_EX).expect("should find requests");
        let request2 = Grant::storage_requesters(*JOHN_EX).expect("should find requests");

        // Ensure we can access the storage requests
        assert_eq!(request1.owner, *PETE_EX);
        assert_eq!(request2.owner, *JOHN_EX);

	});
}

#[test]
fn ensure_only_users_with_no_balance_can_request_grants() {
	new_test_ext().execute_with(|| {

		// Ensure a user can request a grant
		assert_eq!(Balances::free_balance(*JOHN_EX), <Test as Config>::ExistentialDeposit::get());
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));
        
        // Ensure only empty balance can make requests
        assert_noop!(Grant::request_grant(Origin::signed(*ALICE)), Error::<Test>::NonEmptyBalance);

	});
}

#[test]
fn winner_can_be_selected() {
	new_test_ext().execute_with(|| {

		// Request grant
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));

		// go to later block 
		run_to_block(4);

		// Ensure the winner is the only account that requested
		assert_eq!(Grant::winner().unwrap(), *JOHN_EX);

	});
}

#[test]
fn winner_can_be_queried_by_anyone() {
	new_test_ext().execute_with(|| {

		// Request grant
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));

		// go to later block 
		run_to_block(2);

		// Ensure the winner is the only account that requested
		assert_eq!(Grant::winner().unwrap(), *JOHN_EX);
		assert_ok!(Grant::winner_is(Origin::signed(*JOHN_EX)));

	});
}


#[test]
fn winner_can_be_selected_per_block() {
	new_test_ext().execute_with(|| {
		
		// Request grant and run to block
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));
		run_to_block(2);

		// Ensure we have selected the correct winner
		assert_eq!(Grant::winner().unwrap(), *JOHN_EX);
		
		// Request additional grant for different block
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));
		
		run_to_block(5);

		assert_eq!(Grant::winner().unwrap(), *JOHN_EX);

	});
}

#[test]
fn winner_can_be_recieve_grant_reward() {
	new_test_ext().execute_with(|| {

		// Add balance to grant treasury account
		fund_treasury(100_000u64);		
		let treasury = Balances::free_balance(&treasury_account());
		assert_eq!(treasury, 100_000u64);
		
		// Check initial account getting grant is zero.
		assert_eq!(Balances::free_balance(*JOHN_EX), <Test as Config>::ExistentialDeposit::get());
		
		// Request grant and run to block
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));
		run_to_block(2);

		// Ensure we have selected the correct winner
		assert_eq!(Grant::winner().unwrap(), *JOHN_EX);

		//Ensure money is tranfered todo:: look for minimum balance
		assert!(Balances::free_balance(*JOHN_EX) == grant_amount() + <Test as Config>::ExistentialDeposit::get());
	});
}

#[test]
fn test_reciever_can_only_request_once_per_block() {
	new_test_ext().execute_with(|| {
		fund_treasury(100_000u64);
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));

		// Assert it is not possible to request from the same account twice in the same block.
		assert_noop!(Grant::request_grant(Origin::signed(*JOHN_EX)), Error::<Test>::RequestAlreadyMade);

		// Assert that the requesterscount is only one.
		assert!(Grant::requesters_count() == 1);

	});
}

#[test]
fn test_requestors_is_cleared_each_block() {
	new_test_ext().execute_with(|| {

		//Setup state.
		run_to_block(9);
		fund_treasury(100_000u64);
		assert_ok!(Grant::request_grant(Origin::signed(*JOHN_EX)));
		assert_ok!(Grant::request_grant(Origin::signed(*PETE_EX)));
		assert_ok!(Grant::request_grant(Origin::signed(*SIMON_EX)));

		// Assert that the storage has the right amount of elements.
		assert!(Grant::requesters_count() == 3);
		assert!(StorageRequesters::<Test>::iter_keys().count() == 3);

		run_to_block(10);

		// Assert they have been clear on exactly the next block. 
		assert!(Grant::requesters_count() == 0);
		assert!(StorageRequesters::<Test>::iter_keys().count() == 0);
	});
}

#[test]
fn test_max_requestors_errs_gracefully() {
	new_test_ext().execute_with(|| {
		// Get to limit of requestors

		RequestersCount::<Test>::set(u16::MAX);

		assert_noop!(Grant::request_grant(Origin::signed(*SIMON_EX)), Error::<Test>::TooManyRequesters);
	});
}

