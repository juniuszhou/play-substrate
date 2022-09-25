#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	type ClubId = u32;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Max club number
		#[pallet::constant]
		type MaxClub: Get<u32>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Members<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<ClubId, T::MaxClub>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// a member added into club successfully
		ClubMemberAdded { club: ClubId, account: T::AccountId },
		/// a member removed from club successfully
		ClubMemberRemoved { club: ClubId, account: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Club id invalid.
		InvalidClubId,
		/// Try to add the existed member to club again
		AlreadyClubMember,
		/// Try to remove not existed member from club
		NotClubMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// add a member to a club
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(T::WeightInfo::add_member())]
		pub fn add_member(
			origin: OriginFor<T>,
			club: ClubId,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(club < T::MaxClub::get(), Error::<T>::InvalidClubId);
			ensure!(!Self::members(&account).contains(&club), Error::<T>::AlreadyClubMember);

			Members::<T>::mutate(&account, |ref mut clubs| {
				clubs.force_push(club);
			});

			Self::deposit_event(Event::ClubMemberAdded { club, account });
			Ok(())
		}
		/// remove a member from a clubs
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_member(
			origin: OriginFor<T>,
			club: ClubId,
			account: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(club < T::MaxClub::get(), Error::<T>::InvalidClubId);
			ensure!(Self::members(&account).contains(&club), Error::<T>::NotClubMember);

			Members::<T>::mutate(&account, |ref mut clubs| {
				let index = clubs.iter().position(|&r| r == club).unwrap();
				clubs.remove(index);
			});

			// Emit an event.
			Self::deposit_event(Event::ClubMemberRemoved { club, account });
			Ok(())
		}
	}
}
