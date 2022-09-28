use crate::{mock::*, Error, RepInfoOf};
use frame_support::{assert_noop, assert_ok, bounded_vec};


#[test]
fn test_reputation_can_be_created() {
	new_test_ext().execute_with(|| {

        // Assert reputation can be created.
        assert_ok!(Reputation::create_reputation_record(&0));
        assert!(RepInfoOf::<Test>::get(0).is_some());
    });
}

#[test]
fn test_reputation_can_be_removed() {
	new_test_ext().execute_with(|| {

        // Assert reputation can be created.
        assert_ok!(Reputation::create_reputation_record(&0));
        assert!(RepInfoOf::<Test>::get(0).is_some());

        // Assert reputation can be removed.
        assert_ok!(Reputation::remove_reputation_record(0u64));
        assert!(RepInfoOf::<Test>::get(0).is_none());
    });
}

#[test]
fn duplicate_records_cannot_be_created() {
	new_test_ext().execute_with(|| {

        // Assert one record can be created.
        assert_ok!(Reputation::create_reputation_record(&0));
        assert!(RepInfoOf::<Test>::get(0).is_some());

        // Assert the same AccountId cannot.
        assert_noop!(Reputation::create_reputation_record(&0), Error::<Test>::ReputationAlreadyExists);
    });
}

#[test]
fn placeholder_rep_function_works() {
	new_test_ext().execute_with(|| {
        
        // Setup default state.  
        assert_ok!(Reputation::create_reputation_record(&0));

        // Assert logic follows as described in: https://github.com/UniversalDot/universal-dot-node/issues/37
        assert_ok!(Reputation::rate_account(&0, &bounded_vec![1u8, 1u8]));
        let rep_record = RepInfoOf::<Test>::get(0).unwrap();
        assert!(rep_record.reputation == (-4));

        assert_ok!(Reputation::rate_account(&0, &bounded_vec![5u8, 5u8]));
        let rep_record = RepInfoOf::<Test>::get(0).unwrap();
        assert!(rep_record.reputation == 0);

        assert_ok!(Reputation::rate_account(&0, &bounded_vec![5u8, 5u8]));
        let rep_record = RepInfoOf::<Test>::get(0).unwrap();
        assert!(rep_record.reputation == 4);
        
    });
}