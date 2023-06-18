use super::*;
use sp_runtime::traits::SaturatedConversion;

impl<T: Config> Pallet<T> {
    pub fn call_block_number() -> T::BlockNumber {
        // get current block number
        <frame_system::Pallet<T>>::block_number()
    }

    pub fn block_number_ops() {
        // get block number from u32
        let _b: T::BlockNumber = T::BlockNumber::from(10_u32);
        let _u: u32 = <frame_system::Pallet<T>>::block_number().saturated_into::<u32>();
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
