use crate as pallet_template;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, storage::bounded_vec::BoundedVec};
use sp_runtime::traits::BadOrigin;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let club_id = 1;
		let account_id = 2;

		let mut expected: BoundedVec<u32, <Test as pallet_template::Config>::MaxClub> =
			Default::default();
		expected.force_push(1);

		assert_ok!(TemplateModule::add_member(Origin::root(), club_id, account_id));
		assert_eq!(TemplateModule::members(account_id), expected);
		assert_ok!(TemplateModule::remove_member(Origin::root(), club_id, account_id));
		expected.pop();
		assert_eq!(TemplateModule::members(account_id), expected);

		assert_eq!(
			events(),
			[
				Event::TemplateModule(crate::Event::ClubMemberAdded {
					club: club_id,
					account: account_id
				}),
				Event::TemplateModule(crate::Event::ClubMemberRemoved {
					club: club_id,
					account: account_id
				}),
			]
		);
	});
}

#[test]
fn not_root() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 2;

		assert_noop!(
			TemplateModule::add_member(Origin::signed(account_id), club_id, account_id),
			BadOrigin
		);
		assert_noop!(
			TemplateModule::remove_member(Origin::signed(account_id), club_id, account_id),
			BadOrigin
		);
	});
}

#[test]
fn invalid_club_id() {
	new_test_ext().execute_with(|| {
		let club_id = <Test as pallet_template::Config>::MaxClub::get();
		let account_id = 2;

		assert_noop!(
			TemplateModule::add_member(Origin::root(), club_id, account_id),
			Error::<Test>::InvalidClubId
		);

		assert_noop!(
			TemplateModule::remove_member(Origin::root(), club_id, account_id),
			Error::<Test>::InvalidClubId
		);
	});
}

#[test]
fn exists_error() {
	new_test_ext().execute_with(|| {
		let club_id = 1;
		let account_id = 2;
		assert_ok!(TemplateModule::add_member(Origin::root(), club_id, account_id));

		assert_noop!(
			TemplateModule::add_member(Origin::root(), club_id, account_id),
			Error::<Test>::AlreadyClubMember
		);

		assert_ok!(TemplateModule::remove_member(Origin::root(), club_id, account_id));
		assert_noop!(
			TemplateModule::remove_member(Origin::root(), club_id, account_id),
			Error::<Test>::NotClubMember
		);
	});
}
