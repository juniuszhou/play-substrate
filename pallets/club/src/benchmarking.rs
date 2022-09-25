use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::{storage::bounded_vec::BoundedVec, traits::Get};
use frame_system::RawOrigin;

benchmarks! {

	add_member {
		let max_club = T::MaxClub::get();
		let caller: T::AccountId = whitelisted_caller();


		let mut clubs: BoundedVec<u32, T::MaxClub> = Default::default();
		// worst case, almost full
		for club in 0..max_club-1 {
			clubs.force_push(club);
		}
		Members::<T>::insert(&caller, clubs);

	}: _(RawOrigin::Root, max_club-1, caller)

	remove_member {
		let max_club = T::MaxClub::get();
		let caller: T::AccountId = whitelisted_caller();


		let mut clubs: BoundedVec<u32, T::MaxClub> = Default::default();
		// worst case, bounded vec is full
		for club in 0..max_club {
			clubs.force_push(club);
		}
		Members::<T>::insert(&caller, clubs);

	}: _(RawOrigin::Root, max_club-1, caller)
}
