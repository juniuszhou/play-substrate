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
}
