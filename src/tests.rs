use crate::mock::*;
use crate::H256;
use frame_support::{assert_ok, dispatch::{
		DispatchResult, 
		Vec,
}};


#[test]
fn it_works_for_create_new_file() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let owner = 3;

		let create_file_result = Filesign::create_new_file(Origin::signed(owner), tag.clone(), filehash);
		let file = Filesign::get_file_by_id(1);

		assert_ok!(create_file_result, ());
		assert_eq!(owner, file.owner);
		assert_eq!(1, file.id);
		assert_eq!(filehash, file.versions[0].filehash);
		assert_eq!(1, file.versions.len());
		assert_eq!(0, file.signers.len());
	});
}

#[test]
fn it_works_for_create_new_file_increment_version() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let owner1 = 1;
		let owner2 = 1;

		let _ = Filesign::create_new_file(Origin::signed(owner1), tag.clone(), filehash);
		let _ = Filesign::create_new_file(Origin::signed(owner2), tag.clone(), filehash);
		let file1 = Filesign::get_file_by_id(1);
		let file2 = Filesign::get_file_by_id(2);

		assert_eq!(owner1, file1.owner);
		assert_eq!(1, file1.id);
		assert_eq!(owner2, file2.owner);
		assert_eq!(2, file2.id);
	});
}

#[test]
fn it_fails_for_create_new_file_incorrect_file_input() {
	new_test_ext().execute_with(|| {
		let tag = Vec::new();
		let filehash = H256::from([0x66; 32]);
		let owner = 3;

		let create_file_result = Filesign::create_new_file(Origin::signed(owner), tag.clone(), filehash);		
		let file = Filesign::get_file_by_id(1);

		assert_ne!(create_file_result, DispatchResult::Ok(()));
		assert_eq!(0, file.owner);
	});
}

#[test]
fn it_works_assign_signer() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let account_id = 1;

		let create_file_result = Filesign::create_new_file(Origin::signed(1), tag, filehash);
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), 1, account_id);
		let file = Filesign::get_file_by_id(1);

		assert_ok!(create_file_result, ());
		assert_ok!(assign_signer_result, ());
		assert_eq!(1, file.signers.len());
		assert_eq!(account_id, file.signers[0]);
	});
}

#[test]
fn it_works_assign_signer_do_no_dublicates() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let account_id = 2;

		let create_file_result = Filesign::create_new_file(Origin::signed(1), tag, filehash);
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), 1, account_id);

		// Try Dublicate:
		let _ = Filesign::assign_signer(Origin::signed(1), 1, account_id);

		let file = Filesign::get_file_by_id(1);

		assert_ok!(create_file_result, ());
		assert_ok!(assign_signer_result, ());
		assert_eq!(1, file.signers.len());
		assert_eq!(account_id, file.signers[0]);
	});
}


#[test]
fn it_works_delete_signer() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let account_id = 2;

		let create_file_result = Filesign::create_new_file(Origin::signed(1), tag.clone(), filehash);
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), 1, account_id);

		// Check file state before delete
		let file_with_signer = Filesign::get_file_by_id(1);
		let delete_signer_result = Filesign::delete_signer(Origin::signed(1), 1, account_id);

		// Check file state after delete
		let file_without_signer = Filesign::get_file_by_id(1);

		assert_ok!(create_file_result, ());
		assert_ok!(assign_signer_result, ());
		assert_ok!(delete_signer_result, ());
		assert_eq!(1, file_with_signer.signers.len());
		assert_eq!(account_id, file_with_signer.signers[0]);
		assert_eq!(0, file_without_signer.signers.len());
	});
}

#[test]
fn it_fails_delete_signer_no_signers() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);

		let account_id = 1;
		let _ = Filesign::create_new_file(Origin::signed(1), tag.clone(), filehash);

		// First - try to delete unexisting signer 
		let delete_signer_result_no_signers = Filesign::delete_signer(Origin::signed(1), 1, account_id);

		// Second - try to delete unexisting signer after delete:
		let _ = Filesign::assign_signer(Origin::signed(1), 1, account_id);
		let _ = Filesign::delete_signer(Origin::signed(1), 1, account_id);
		let delete_signer_result_after_delete = Filesign::delete_signer(Origin::signed(1), 1, account_id);

		assert_ne!(delete_signer_result_no_signers, DispatchResult::Ok(()));
		assert_ne!(delete_signer_result_after_delete, DispatchResult::Ok(()));
	});
}


#[test]
fn it_works_sign_latest_version() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let account_id = 1;

		let _ = Filesign::create_new_file(Origin::signed(1), tag, filehash);
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), 1, account_id);
		let sign_latest_version_result = Filesign::sign_latest_version(Origin::signed(1), 1);
		let _ = Filesign::sign_latest_version(Origin::signed(1), 1);
		let file = Filesign::get_file_by_id(1);

		assert_ok!(assign_signer_result, ());
		assert_ok!(sign_latest_version_result, ());
		assert_eq!(1, file.versions.last().unwrap().signatures.len());
	});
}

#[test]
fn it_fail_sign_latest_version_not_an_signer() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);

		let _ = Filesign::create_new_file(Origin::signed(1), tag, filehash);
		let sign_latest_version_result = Filesign::sign_latest_version(Origin::signed(1), 1);
		let file = Filesign::get_file_by_id(1);

		assert_ne!(sign_latest_version_result, DispatchResult::Ok(()));
		// Assert that no sign has been added
		assert_eq!(0, file.versions.last().unwrap().signatures.len());
	});
}