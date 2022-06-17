use core::convert::TryInto;

use crate::TaskStatus;
use crate::{mock::*, Error};
use frame_support::traits::fungible::Inspect;
use frame_support::storage::bounded_vec::BoundedVec;
use frame_support::{assert_noop, assert_ok, traits::{UnixTime, Hooks}};

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<  Constants and Functions used in TESTS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

pub const MILLISEC_PER_BLOCK: u64 = 12000; // 12 sec for a block
pub const HOURS : u8 = 40_u8;
pub const BUDGET : u64 = 7_u64;
pub const BUDGET2 : u64 = 10_u64;

fn username() -> BoundedVec<u8, MaxUsernameLen> {
	vec![1u8, 4].try_into().unwrap()
}

fn interests() -> BoundedVec<u8, MaxInterestsLen> {
	vec![1u8, 4].try_into().unwrap()
}

fn title() -> BoundedVec<u8, MaxTitleLen> {
	vec![1u8, 2].try_into().unwrap()
}

fn title2() -> BoundedVec<u8, MaxTitleLen> {
	vec![1u8, 7].try_into().unwrap()
}

fn spec() -> BoundedVec<u8, MaxSpecificationLen> {
	vec![1u8, 3].try_into().unwrap()
}

fn spec2() -> BoundedVec<u8, MaxSpecificationLen> {
	vec![1u8, 4].try_into().unwrap()
}

fn attachments() -> BoundedVec<u8, MaxAttachmentsLen> {
	vec![1u8, 4].try_into().unwrap()
}

fn attachments2() -> BoundedVec<u8, MaxAttachmentsLen> {
	vec![1u8, 3].try_into().unwrap()
}

fn keywords() -> BoundedVec<u8, MaxKeywordsLen> {
	vec![1u8, 5].try_into().unwrap()
}

fn keywords2() -> BoundedVec<u8, MaxKeywordsLen> {
	vec![1u8, 5].try_into().unwrap()
}

fn feedback() -> BoundedVec<u8, MaxFeedbackLen> {
	vec![1u8, 4].try_into().unwrap()
}


fn get_deadline() -> u64 {
		// deadline is current time + 1 hour
		let deadline = <Time as UnixTime>::now() + std::time::Duration::from_millis(3600 * 1000_u64);
		let deadline_u64 = deadline.as_secs() * 1000_u64;
		assert_eq!(deadline.as_millis(), deadline_u64 as u128);
		deadline_u64
}

fn run_to_block(n: u64) {
	Task::on_finalize(System::block_number());
	for b in (System::block_number() + 1)..=n {
		next_block(b);
		if b != n {
			Task::on_finalize(System::block_number());
		}
	}
}

fn next_block(n: u64) {
	Time::set_timestamp(MILLISEC_PER_BLOCK * n);
	System::set_block_number(n);
	Task::on_initialize(n);
}


// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<  TESTS  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

#[test]
fn create_new_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));
	});
}

#[test]
fn fund_transfer_on_create_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		assert_eq!(Balances::balance(&1), 1000);
		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec() , BUDGET, get_deadline(), attachments(), keywords()));
		assert_eq!(Balances::balance(&1), 993);
		let task_id = Task::tasks_owned(&1)[0];
		assert_eq!(Balances::balance(&Task::account_id(&task_id)), BUDGET);
	});
}

#[test]
fn increase_task_count_when_creating_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec() , BUDGET, get_deadline(), attachments(), keywords()));

		// Assert that count is incremented by 1 after task creation
		assert_eq!(Task::task_count(), 1);
	});
}

#[test]
fn increase_task_count_when_creating_two_tasks(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec2(), BUDGET2, get_deadline(), attachments(), keywords()));

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 2);
	});
}

#[test]
fn cant_own_more_tax_than_max_tasks(){
	new_test_ext().execute_with( || {

		// TODO: use MaxTasksOwned instead of hardcoded values;

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Create 77 tasks  ExceedMaxTasksOwned
		for _n in 0..77 {
			// Ensure new task can be created.
			assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));
		}

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 77);

		// Assert that when creating the 77 Task, Error is thrown
		assert_noop!(Task::create_task(Origin::signed(1), title(), spec2(), BUDGET, get_deadline(), attachments(), keywords()), Error::<Test>::ExceedMaxTasksOwned);

	});

}

#[test]
fn assign_task_to_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), username(), interests(), HOURS));

		assert_ok!(Task::create_task(Origin::signed(10), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		assert_eq!(task.current_owner, 10);
	});
}

#[test]
fn verify_inputs_outputs_to_tasks(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), username(), interests(), HOURS));

		assert_ok!(Task::create_task(Origin::signed(10), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// Ensure that task properties are assigned correctly
		assert_eq!(task.current_owner, 10);
		assert_eq!(task.specification, vec![1u8, 3]);
		assert_eq!(task.budget, BUDGET);
		assert_eq!(task.title, title());
		assert_eq!(task.attachments, attachments());
		assert_eq!(task.keywords, keywords());
	});
}

#[test]
fn task_can_be_updated_after_it_is_created(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), username(), interests(), HOURS));

		assert_ok!(Task::create_task(Origin::signed(10), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// assert the budget is correct
		assert_eq!(task.budget, BUDGET);

		assert_ok!(Task::update_task(Origin::signed(10), hash, title2(), spec2(), BUDGET2, get_deadline(), attachments2(), keywords2()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// Ensure that task properties are assigned correctly
		assert_eq!(task.current_owner, 10);
		assert_eq!(task.budget, BUDGET2);
		assert_eq!(task.title, title2());
		assert_eq!(task.attachments, attachments2());
		assert_eq!(task.keywords, keywords2());
	});
}

#[test]
fn check_balance_after_update_task(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(10), username(), interests(), HOURS));

		assert_ok!(Task::create_task(Origin::signed(10), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		let hash = Task::tasks_owned(10)[0];
		assert_ok!(Task::update_task(Origin::signed(10), hash, title2(), spec2(), BUDGET2, get_deadline(), attachments2(), keywords2()));

		let hash = Task::tasks_owned(10)[0];
		let task_account = Task::account_id(&hash);
		assert_eq!(Balances::balance(&task_account), BUDGET2);

		assert_ok!(Task::update_task(Origin::signed(10), hash, title2(), spec2(), BUDGET, get_deadline(), attachments2(), keywords2()));
		let hash = Task::tasks_owned(10)[0];
		let task_account = Task::account_id(&hash);
		assert_eq!(Balances::balance(&task_account), BUDGET);

	});
}

#[test]
fn task_can_be_updated_only_by_one_who_created_it(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), username(), interests(), HOURS));

		assert_ok!(Task::create_task(Origin::signed(10), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];

		// Throw error when someone other than creator tries to update task
		assert_noop!(Task::update_task(Origin::signed(7), hash, title(), spec(), BUDGET2, get_deadline(), attachments2(), keywords2()), Error::<Test>::OnlyInitiatorUpdatesTask);

	});
}

#[test]
fn task_can_be_updated_only_after_it_has_been_created(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), username(), interests(), HOURS));

		// Ensure task can be created
		assert_ok!(Task::create_task(Origin::signed(10), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Throw error when someone other than creator tries to update task
		assert_noop!(Task::update_task(Origin::signed(10), hash, title(), spec(), BUDGET2, get_deadline(), attachments2(), keywords2()), Error::<Test>::NoPermissionToUpdate);

	});
}

#[test]
fn start_tasks_assigns_new_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

	});
}

#[test]
fn start_tasks_assigns_task_to_volunteer(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started it is assigned to volunteer (user 2)
		assert_eq!(task.volunteer, 1);
		assert_eq!(Task::tasks_owned(2).len(), 1);
		assert_eq!(Task::tasks_owned(1).len(), 0);
	});
}

#[test]
fn completing_tasks_assigns_new_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);
	});
}

#[test]
fn the_volunteer_is_different_from_task_creator(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		let hash = Task::tasks_owned(1)[0];
		assert_noop!(Task::start_task(Origin::signed(1), hash), Error::<Test>::NoPermissionToStart);

	});
}


#[test]
fn task_can_only_be_started_once(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		assert_noop!(Task::start_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToStart);

	});
}

#[test]
fn task_can_only_be_finished_by_the_user_who_started_it(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure that a user who didn't start the task has no permission to complete it
		assert_noop!(Task::complete_task(Origin::signed(1), hash), Error::<Test>::NoPermissionToComplete);

	});
}

#[test]
fn task_can_be_removed_by_owner(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];

		// Ensure another user can't remove the task
		assert_noop!(Task::remove_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToRemove);

		// Ensure the task can be removed
		assert_ok!(Task::remove_task(Origin::signed(1), hash));
		assert_eq!(Task::task_count(), 0);

	});
}

#[test]
fn task_can_be_removed_only_when_status_is_created(){
	new_test_ext().execute_with( || {

		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), 7, get_deadline(), attachments(), keywords()));

		// Ensure that task can't be started once its started
		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure another user can't remove the task
		assert_noop!(Task::remove_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToRemove);

	});
}


#[test]
fn only_creator_accepts_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);

		// Ensure task is accepted by task creator (user 1)
		assert_noop!(Task::accept_task(Origin::signed(2), hash), Error::<Test>::OnlyInitiatorAcceptsTask);
		assert_ok!(Task::accept_task(Origin::signed(1), hash));
	});
}

#[test]
fn accepted_task_is_added_to_completed_task_for_volunteer(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		let hash = Task::tasks_owned(1)[0];
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		// An accepted task is added as completed task on volunteer's profile.
		let completed_tasks = Profile::completed_tasks(2);
		assert!(completed_tasks.is_some());
		assert_eq!(completed_tasks.unwrap().into_inner(), vec![hash]);
	});
}
#[test]
fn volunteer_gets_paid_on_task_completion(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		let hash = Task::tasks_owned(1)[0];
		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));


		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure task is accepted by task creator (user 1)
		// User 2 gets fund for completing task after it is accepted by user 1
		assert_eq!(Balances::balance(&2), 1000);
		assert_ok!(Task::accept_task(Origin::signed(1), hash));
		assert_eq!(Balances::balance(&2), 1007);

	});
}

#[test]
fn only_started_task_can_be_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		assert_noop!(Task::complete_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToComplete);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));
	});
}

#[test]
fn when_task_is_accepted_ownership_is_cleared(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);

		// Ensure task is accepted by task creator (user 1)
		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		// Ensure ownership of task is cleared
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 0);
	});
}


#[test]
fn decrease_task_count_when_accepting_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Accepting task decreases count
		assert_ok!(Task::accept_task(Origin::signed(1), hash));
		assert_eq!(Task::task_count(), 0);
	});
}

#[test]
fn task_can_be_rejected_by_creator(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Task is rejected by creator
		assert_ok!(Task::reject_task(Origin::signed(1), hash, feedback()));

		// Assert that the status is back in progress and, owner is the volunteer
		let hash = Task::tasks_owned(2)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 2);
		assert_eq!(task.status, TaskStatus::InProgress);

	});
}


#[test]
fn feedback_is_given_when_task_is_rejected(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Task is rejected by creator
		assert_ok!(Task::reject_task(Origin::signed(1), hash, feedback()));

		// Assert that the status is back in progress and, owner is the volunteer
		let hash = Task::tasks_owned(2)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.feedback, Some(feedback()));

	});
}

// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<  Integration tests  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


#[test]
fn increase_profile_reputation_when_task_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		assert_ok!(Profile::create_profile(Origin::signed(2), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure task is accepted by task creator (user 1)
		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		let profile1 = Profile::profiles(1).expect("should find the profile");
		let profile2 = Profile::profiles(2).expect("should find the profile");

		// Ensure that the reputation has been added to both profiles
		assert_eq!(profile1.reputation, 1);
		assert_eq!(profile2.reputation, 1);

	});
}

#[test]
fn only_add_reputation_when_task_has_been_accepted(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Ensure task can be accepted
		assert_ok!(Task::accept_task(Origin::signed(1), hash));

		// Reputation should remain 0 since the task was removed without being completed
		// TODO: Make sure that user creating a task is not the same as the one completing it
		let profile = Profile::profiles(1).expect("should find the profile");
		assert_eq!(profile.reputation, 2);
	});
}

#[test]
fn delete_task_after_deadline() {
	new_test_ext().execute_with( || {
		run_to_block(1);
		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash);
		assert!(task.is_some());
		// deadline is 1 hour => 3600 sec => 300 blocks as 12 secs per block
		run_to_block(302);
		let task = Task::tasks(hash);
		assert!(task.is_none());
	});
}

#[test]
fn balance_check_after_delete_task() {
	new_test_ext().execute_with( || {
		run_to_block(1);
		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));
		let signer_balance = Balances::balance(&1);

		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));
		let signer_balance_after_task_creation = Balances::balance(&1);
		let hash = Task::tasks_owned(1)[0];
		let task_account = Task::account_id(&hash);
		let task_balance = Balances::balance(&task_account);
		assert_eq!(task_balance, BUDGET);
		assert_eq!(signer_balance, signer_balance_after_task_creation + task_balance);
		assert_ok!(Task::remove_task(Origin::signed(1), hash));
		let signer_balance_after_task_deletion = Balances::balance(&1);
		let task_balance = Balances::balance(&task_account);
		assert_eq!(signer_balance, signer_balance_after_task_deletion);
		assert_eq!(task_balance, 0);
	});
}

#[test]
fn block_time_is_added_when_task_is_updated() {
	new_test_ext().execute_with( || {
		System::set_block_number(1);

		assert_ok!(Profile::create_profile(Origin::signed(1), username(), interests(), HOURS));

		// Ensure new task can be created.
		assert_ok!(Task::create_task(Origin::signed(1), title(), spec(), BUDGET, get_deadline(), attachments(), keywords()));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// Ensure block time of task creation is correct
		assert_eq!(task.created_at, 1);

		System::set_block_number(3);
		assert_ok!(Task::update_task(Origin::signed(1), hash, title2(), spec2(), BUDGET2, get_deadline(), attachments2(), keywords2()));

		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.updated_at, 3);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		System::set_block_number(100);
		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.completed_at, 100);

	})
}
