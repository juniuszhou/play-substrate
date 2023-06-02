use super::*;
use crate::{mock::*, Error, *};
use frame_support::{assert_noop, assert_ok};
use codec::{Encode, Decode};

use crate as play_balances;

use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};



#[test]
fn test_map() {
	new_test_ext().execute_with(|| {
		// to get value via pallet instance and the method
		assert_eq!(PlayBalances::some_map(0), None);

		// to get value via data type instance with generic parameter
		assert_eq!(SomeMap::<Test>::take(0), None);

		assert_eq!(SomeMap::<Test>::get(0), None);

		assert_eq!(SomeMap::<Test>::try_get(0), Err(()));

		SomeMap::<Test>::mutate(0, |value| *value = Some(123));

		assert_eq!(SomeMap::<Test>::get(0).unwrap(), 123);

		SomeMap::<Test>::try_mutate(0, |value| {
			if value.unwrap() > 100 {
				*value = Some(99);
				Ok(())
			} else {
				Err(())
			}
		});

		assert_eq!(SomeMap::<Test>::get(0).unwrap(), 99);
	});
}

#[test]
fn test_double_map() {
	new_test_ext().execute_with(|| {
		// to get value via pallet instance and the method
		assert_eq!(PlayBalances::some_double_map(0, 0), None);

		// to get value via data type instance with generic parameter
		assert_eq!(SomeDoubleMap::<Test>::take(0, 0), None);

		assert_eq!(SomeDoubleMap::<Test>::get(0, 0), None);

		SomeDoubleMap::<Test>::mutate(0, 0, |value| *value = Some(123));

		assert_eq!(SomeDoubleMap::<Test>::get(0, 0).unwrap(), 123);

		
	});
}
