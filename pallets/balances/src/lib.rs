#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod migrations;
// pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod impl_account;
mod impl_data_type;
mod impl_system_usage;

use frame_support::{
    print,
    traits::{
        Currency, Get, Imbalance, OnUnbalanced, ReservableCurrency, StorageVersion, WithdrawReasons,
    },
};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::PositiveImbalance;
pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::PalletInfo;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    // pub use types::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// both constants are from trait Get.
        /// const for type not known need get from config
        #[pallet::constant]
        type BlockHashCount: Get<Self::BlockNumber>;

        /// type just know it is u16
        #[pallet::constant]
        type SS58Prefix: Get<u16>;

        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    /// how to define origin
    #[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, RuntimeDebug, MaxEncodedLen)]
    #[pallet::origin]
    pub enum Origin {
        /// Origin one
        One,
        /// Origin two
        Two,
    }

    impl From<u32> for Origin {
        fn from(id: u32) -> Origin {
            Origin::One
        }
    }

    /// if a pallet without any storage in runtime.
    /* 	#[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);
    */

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn some_map)]
    /// Twox64Concat map hash function
    /// u8 map key
    /// u32 map value
    pub type SomeMap<T: Config> = StorageMap<_, Twox64Concat, u8, u32, OptionQuery, GetDefault>;

    #[pallet::storage]
    #[pallet::getter(fn some_value_map)]
    /// Twox64Concat map hash function
    /// u8 map key
    /// u32 map value
    pub type QueryMap<T: Config> = StorageMap<_, Twox64Concat, u8, u32, ValueQuery, GetDefault>;

    #[pallet::storage]
    #[pallet::getter(fn some_double_map)]
    pub(super) type SomeDoubleMap<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, u32>;

    // used to check some parameters of pallet, it is executed when unit test for runtime
    // which integrate the pallet, then you can set the parameters correctly when construct runtime
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn integrity_test() {
            assert!(
                <T as Config>::BlockHashCount::get() > T::BlockNumber::from(0_u32),
                "`BlockHashCount` must ge greater than 0"
            );
        }
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000)]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            // let id: u32 = who.into();
            // let local_origin: Origin = id.into();

            // Update storage.
            <Something<T>>::put(something);

            <SomeMap<T>>::insert(1_u8, 1_u32);

            if Self::some_map(1_u8) == Some(1_u32) {
                return Ok(());
            }

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000)]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T>>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T>>::put(new);
                    Ok(())
                }
            }
        }

        #[pallet::weight(10_000)]
        pub fn dummy(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let _info = T::PalletInfo::index::<Pallet<T>>().unwrap();

            // assert_eq!(PalletInfo::index::<System>().unwrap(), 30);
            // call print here

            frame_support::print(
                "Inconsistent state - couldn't settle imbalance for funds spent by treasury",
            );

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_to_origin(id: T::AccountId) -> Origin {
            match Some(id) {
                Some(_) => Origin::One,
                None => Origin::Two,
            }

            // Origin::Two
        }
    }
}
