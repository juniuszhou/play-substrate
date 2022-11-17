use super::*;

impl<T: Config> Pallet<T> {
    pub fn call_block_number() -> T::BlockNumber {
        <frame_system::Pallet<T>>::block_number()
    }
}

#[test]
fn test_call_block_number() {
    use crate::{mock::*, Error};

    new_test_ext().execute_with(|| {

        System::set_block_number(100);
        let now = PlayBalances::call_block_number();
        println!("{}", now);
        assert_eq!(now, 100);
    });
}

