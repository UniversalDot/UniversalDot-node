use crate::{mock::*, Error, RepInfoOf};
use frame_support::{assert_noop, assert_ok};




#[test]
fn test_reputation_can_be_created() {
	new_test_ext().execute_with(|| {
        assert_ok!(Reputation::create_reputation_record(&0));
        assert!(RepInfoOf::<Test>::get(0).is_some());
    });
}

#[test]
fn test_reputation_can_be_removed() {
	new_test_ext().execute_with(|| {
        assert_ok!(Reputation::create_reputation_record(&0));
        assert!(RepInfoOf::<Test>::get(0).is_some());

        assert_ok!(Reputation::remove_reputation_record(0u64));

        assert!(RepInfoOf::<Test>::get(0).is_none());
    });
}

#[test]
fn duplicate_records_cannot_be_created() {
	new_test_ext().execute_with(|| {
        assert_ok!(Reputation::create_reputation_record(&0));
        assert!(RepInfoOf::<Test>::get(0).is_some());
        assert_noop!(Reputation::create_reputation_record(&0), Error::<Test>::ReputationAlreadyExists);
    });
}

#[test]
fn placeholder_rep_function_works() {
	new_test_ext().execute_with(|| {
        assert_ok!(Reputation::create_reputation_record(&0));
        assert_ok!(Reputation::rate_account(&0, &vec![1u8, 1u8]));

        let rep_record = RepInfoOf::<Test>::get(0).unwrap();
        assert!(rep_record.reputation == (-4));

        assert_ok!(Reputation::rate_account(&0, &vec![5u8, 5u8]));
        let rep_record = RepInfoOf::<Test>::get(0).unwrap();
        assert!(rep_record.reputation == 0);


        assert_ok!(Reputation::rate_account(&0, &vec![5u8, 5u8]));
        let rep_record = RepInfoOf::<Test>::get(0).unwrap();
        assert!(rep_record.reputation == 4);
        
    });
}