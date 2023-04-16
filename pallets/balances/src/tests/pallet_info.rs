use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::PalletInfoAccess};
use codec::{Encode, Decode};

use crate as play_balances;

use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

#[test]
fn get_pallet_info() {
	new_test_ext().execute_with(|| {
		// Read pallet storage and assert an expected result.
		assert_eq!(<PlayBalances as PalletInfoAccess>::name(), "PlayBalances");
	});
}
