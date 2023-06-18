use super::*;
// use alloc::sync::Arc;
use sp_core::sr25519::Public;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::{AccountId32, MultiSignature, MultiSigner};

impl<T: Config> Pallet<T> {
    fn account_id_to(who: T::AccountId, public: Public, signature: MultiSigner) {
        // let len = who.0.len();
        let sig = MultiSigner::from(public);

        let sender: AccountId32 = signature.into_account();

        // let account_id: AccountId32 = signature.into_account();

        // let ms: MultiSignature = who.into();
        // let account: T::AccountId = T::AccountId::from(sender);
    }

    // fn do_verify(v: dyn Verify) {

    // }
}

// account_id_to_sr25519_public
