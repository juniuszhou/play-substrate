#![cfg_attr(not(feature = "std"), no_std)]

use sp_io::offchain_index;
use sp_runtime::{
	offchain as rt_offchain,
	offchain::{
		storage::StorageValueRef,
		storage_lock::{BlockAndTime, StorageLock},
	},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
	},
	RuntimeDebug,
};

pub fn set() {
    let key = vec![];
    let data = vec![];
    offchain_index::set(&key, &data.encode());

    let oci_mem = StorageValueRef::persistent(&key);
}

