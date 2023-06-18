use crate::{mock::*, Error};
use codec::{Decode, Encode};
use frame_support::{assert_noop, assert_ok};

use crate as play_balances;

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
fn test_basic_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(PlayBalances::do_something(RuntimeOrigin::signed(1), 42));

        match PlayBalances::something() {
            Some(ref mut value) => {
                // can't update it even use ref and mut
                // let mut a = 100_u32;
                *value = 100_u32;
            }
            None => {}
        }
        // Read pallet storage and assert an expected result.
        assert_eq!(PlayBalances::something(), Some(42));
    });
}

#[test]
fn test_basic_value_two() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(PlayBalances::do_something(RuntimeOrigin::signed(1), 42));

        // how to get storage and update via mutate
        play_balances::Something::<Test>::mutate(|mut data| *data = Some(100));
        // Read pallet storage and assert an expected result.
        assert_eq!(PlayBalances::something(), Some(100));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            PlayBalances::cause_error(RuntimeOrigin::signed(1)),
            Error::<Test>::NoneValue
        );
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

#[test]
fn test_codec() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum EnumType {
        #[codec(index = 15)]
        A,
        B(u32, u64),
        C {
            a: u32,
            b: u64,
        },
    }

    let a = EnumType::A;
    let b = EnumType::B(1, 2);
    let c = EnumType::C { a: 1, b: 2 };

    a.using_encoded(|ref slice| {
        assert_eq!(slice, &b"\x0f");
    });

    b.using_encoded(|ref slice| {
        assert_eq!(slice, &b"\x01\x01\0\0\0\x02\0\0\0\0\0\0\0");
    });

    c.using_encoded(|ref slice| {
        assert_eq!(slice, &b"\x02\x01\0\0\0\x02\0\0\0\0\0\0\0");
    });

    let mut da: &[u8] = b"\x0f";
    assert_eq!(EnumType::decode(&mut da).ok(), Some(a));

    let mut db: &[u8] = b"\x01\x01\0\0\0\x02\0\0\0\0\0\0\0";
    assert_eq!(EnumType::decode(&mut db).ok(), Some(b));

    let mut dc: &[u8] = b"\x02\x01\0\0\0\x02\0\0\0\0\0\0\0";
    assert_eq!(EnumType::decode(&mut dc).ok(), Some(c));

    let mut dz: &[u8] = &[0];
    assert_eq!(EnumType::decode(&mut dz).ok(), None);
}
