use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(PlayBalances::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(PlayBalances::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(PlayBalances::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
	});
}

// BoundedVec usage
#[test]
fn bound_try_into_vec() {
    use frame_support::pallet_prelude::*;

    // We start with an unbounded vector with 10 elements.
    let unbounded: Vec<u8> = vec![1; 10];
    // No we try create a bounded vec with only 5 elements from the bounded vec.
    let bounded = BoundedVec::<u8, ConstU32<5>>::try_from(unbounded.clone());
    assert!(bounded.is_err(), "Does not fit");
    
    // This panics.
    // let _bounded: BoundedVec::<u8, ConstU32<5>> = unbounded.try_into().unwrap();
}

// BoundedVec usage
#[test]
fn bound_truncate_vec() {
    use frame_support::pallet_prelude::*;

    // We start with an unbounded vector with 10 elements.
    let unbounded: Vec<u8> = vec![1; 10];
    // No we create a bounded vec with the first 5 elements.
    let bounded = BoundedVec::<u8, ConstU32<5>>::truncate_from(unbounded.clone());
    assert_eq!(bounded.len(), 5);
}