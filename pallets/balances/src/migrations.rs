use super::*;
use frame_support::{
	dispatch::GetStorageVersion, pallet_prelude::ValueQuery, storage_alias,
	traits::OnRuntimeUpgrade,
    weights::Weight,
};

pub mod v1 {
	use super::*;

	pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
			frame_support::ensure!(
				StorageVersion::<T>::get() == 0,
				"Required v0 before upgrading to v1"
			);

			Ok(Default::default())
		}

		fn on_runtime_upgrade() -> Weight {
			Weight::default()
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
			frame_support::ensure!(
				Pallet::<T>::on_chain_storage_version() == 13,
				"v13 not applied"
			);

			frame_support::ensure!(
				!StorageVersion::<T>::exists(),
				"Storage version not migrated correctly"
			);

			Ok(())
		}
	}
}

