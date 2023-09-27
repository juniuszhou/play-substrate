use crate::{mock::*, Error};
use codec::{Decode, Encode};
use frame_support::{assert_noop, assert_ok};

use sp_io::{
    hashing::{blake2_128, keccak_256},
    offchain::timestamp,
    storage::{get, set},
    TestExternalities,
};

use sp_core::offchain::{testing, OffchainWorkerExt};

#[test]
fn test_get_set() {
    TestExternalities::default().execute_with(|| {
        let key = b"hello";
        let value = b"world";
        let value_in_storage = get(key);
        assert_eq!(None, value_in_storage);

        set(key, value);
        let value_in_storage = get(key).unwrap();
        assert_eq!(value, &value_in_storage[..]);
    })
}

#[test]
fn test_hashing() {
    let data = b"hello world";
    let hash = blake2_128(data);
    let hash2 = keccak_256(data);
    println!("hash: {:?}, hash2: {:?}", hash, hash2);
}

#[test]
fn test_get_timestamp() {
    let mut ext = TestExternalities::default();
    let (offchain, _state) = testing::TestOffchainExt::new();
    ext.register_extension(OffchainWorkerExt::new(offchain));
    ext.execute_with(|| {
        let now = timestamp();
        println!("timestamp: {:?}", now);
    })
}
