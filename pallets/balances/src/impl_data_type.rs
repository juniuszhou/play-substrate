use super::*;

impl<T: Config> Pallet<T> {
    fn dummy_account(who: T::AccountId) {}

    fn map_operation() {
        // insert data
        <SomeMap<T>>::insert(1_u8, 1_u32);

        // update 
        <SomeMap<T>>::mutate(1_u8, |mut data| *data = 
            match data {
            Some(value) => Some(*value + 1_u32),
            None => Some(1_u32)
            }
        );
    }

    fn double_map_ops() {
        let key = 0_u32;
        let value = 1_u32;

        let remove_limit = 100_u32;

        // insert
        <SomeDoubleMap<T>>::insert(key, key, value);

        // remove 
        <SomeDoubleMap<T>>::remove(key, key);

        // clear with prefix. 
        <SomeDoubleMap<T>>::clear_prefix(key, remove_limit, None);

        // get all item according to first key
        let _all: Vec<u32> = <SomeDoubleMap<T>>::iter_key_prefix(key).collect();
    }
}
