use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::{PalletInfoAccess, GetStorageVersion, StorageVersion, OnGenesis}};
use codec::{Encode, Decode};

use crate as play_balances;

use sp_std::collections::{btree_map::BTreeMap, btree_set::BTreeSet};

#[test]
fn get_pallet_info() {
	new_test_ext().execute_with(|| {
        // must call on_genesis, then we can get correct storage version.
        play_balances::Pallet::<Test>::on_genesis();

		println!("Pallet name is {:?}", <PlayBalances as PalletInfoAccess>::name());

        println!("Pallet storage version is {:?}", <PlayBalances as GetStorageVersion>::on_chain_storage_version());
	});
}
