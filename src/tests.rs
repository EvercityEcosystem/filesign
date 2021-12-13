use crate::mock::*;
use crate::H256;
use frame_support::{assert_ok, assert_noop, dispatch::{
		DispatchResult, 
		Vec,
}};

type RuntimeError = crate::Error<TestRuntime>;

fn generate_file_id() -> crate::file::FileId {
	[6; 16]
}

#[test]
fn it_works_for_create_new_file() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let owner = 3;
		let file_id = generate_file_id();

		let create_file_result = Filesign::create_new_file(Origin::signed(owner), tag, filehash, Some(file_id));
		let file_option = Filesign::get_file_by_id(file_id);
		
		assert!(file_option.is_some());
		let file = file_option.unwrap();

		assert_ok!(create_file_result, ());
		assert_eq!(owner, file.owner);
		assert_eq!(file_id, file.id);
		assert_eq!(filehash, file.versions[0].filehash);
		assert_eq!(1, file.versions.len());
		assert_eq!(0, file.signers.len());
	});
}

#[test]
fn it_works_for_create_new_file_id_already_exists() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let owner = 3;
		let second_owner = 4;
		let file_id = generate_file_id();

		let create_file_result = Filesign::create_new_file(Origin::signed(owner), tag.clone(), filehash, Some(file_id));
		let create_second_file_result = Filesign::create_new_file(Origin::signed(second_owner), tag, filehash, Some(file_id));
		let file_option = Filesign::get_file_by_id(file_id);
		
		assert!(file_option.is_some());
		let file = file_option.unwrap();

		assert_noop!(create_second_file_result, RuntimeError::IdAlreadyExists);
		assert_ok!(create_file_result, ());
		assert_eq!(owner, file.owner);
		assert_eq!(file_id, file.id);
		assert_eq!(filehash, file.versions[0].filehash);
		assert_eq!(1, file.versions.len());
		assert_eq!(0, file.signers.len());
	});
}

#[test]
fn it_works_for_create_new_file_no_file_id() {
	new_test_ext_with_event().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let owner = 3;

		let create_file_result = Filesign::create_new_file(Origin::signed(owner), tag, filehash, None);

		let event = last_event().unwrap();

		// get file id from event
		let file_id = match event {
			Event::pallet_filesign(e) => {
				match e {
					crate::RawEvent::FileCreated(_, id) => {
						id
					},
					_ => panic!("event not right")
				}
			},
			_ => panic!("event not right")
		};


		let file_option = Filesign::get_file_by_id(file_id);
		assert!(file_option.is_some());
		let file = file_option.unwrap();

		assert_ne!([0; 16], file_id);
		assert_ok!(create_file_result, ());
		assert_eq!(owner, file.owner);
		assert_eq!(file_id, file.id);
		assert_eq!(filehash, file.versions[0].filehash);
		assert_eq!(1, file.versions.len());
		assert_eq!(0, file.signers.len());
	});
}

#[test]
fn it_fails_for_create_new_file_incorrect_file_input() {
	new_test_ext().execute_with(|| {
		let tag = Vec::new();
		let filehash = H256::from([0x66; 32]);
		let owner = 3;
		let file_id = generate_file_id();

		let create_file_result = Filesign::create_new_file(Origin::signed(owner), tag, filehash, Some(file_id));		
		let file_opt = Filesign::get_file_by_id(file_id);

		assert!(file_opt.is_none());
		assert_ne!(create_file_result, DispatchResult::Ok(()));
	});
}

#[test]
fn it_works_assign_signer() {
	new_test_ext().execute_with(|| {
		let tag = vec![40, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
		let filehash = H256::from([0x66; 32]);
		let account_id = 1;
		let file_id = generate_file_id();

		let create_file_result = Filesign::create_new_file(Origin::signed(1), tag, filehash, Some(file_id));
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), file_id, account_id);
		let file_opt = Filesign::get_file_by_id(file_id);

		assert!(file_opt.is_some());
		let file = file_opt.unwrap();

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
		let file_id = generate_file_id();

		let create_file_result = Filesign::create_new_file(Origin::signed(1), tag, filehash, Some(file_id));
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), file_id, account_id);

		// Try Dublicate:
		let _ = Filesign::assign_signer(Origin::signed(1), file_id, account_id);

		let file_opt = Filesign::get_file_by_id(file_id);

		assert!(file_opt.is_some());
		let file = file_opt.unwrap();

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
		let file_id = generate_file_id();

		let create_file_result = Filesign::create_new_file(Origin::signed(1), tag, filehash, Some(file_id));
		let assign_signer_result = Filesign::assign_signer(Origin::signed(1), file_id, account_id);

		// Check file state before delete
		let file_with_signer_opt = Filesign::get_file_by_id(file_id);
		let delete_signer_result = Filesign::delete_signer(Origin::signed(1), file_id, account_id);

		// Check file state after delete
		let file_without_signer_opt = Filesign::get_file_by_id(file_id);

		assert!(file_with_signer_opt.is_some());
		let file_with_signer = file_with_signer_opt.unwrap();
		assert!(file_without_signer_opt.is_some());
		let file_without_signer = file_without_signer_opt.unwrap();

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
		let file_id = generate_file_id();

		let account_id = 1;
		let _ = Filesign::create_new_file(Origin::signed(1), tag, filehash, Some(file_id));

		// First - try to delete unexisting signer 
		let delete_signer_result_no_signers = Filesign::delete_signer(Origin::signed(1), file_id, account_id);

		// Second - try to delete unexisting signer after delete:
		let _ = Filesign::assign_signer(Origin::signed(1), file_id, account_id);
		let _ = Filesign::delete_signer(Origin::signed(1), file_id, account_id);
		let delete_signer_result_after_delete = Filesign::delete_signer(Origin::signed(1), file_id, account_id);

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
		let file_id = generate_file_id();

		let _ = Filesign::create_new_file(Origin::signed(account_id), tag, filehash, Some(file_id));
		let assign_signer_result = Filesign::assign_signer(Origin::signed(account_id), file_id, account_id);
		let sign_latest_version_result = Filesign::sign_latest_version(Origin::signed(account_id), file_id);
		let _ = Filesign::sign_latest_version(Origin::signed(account_id), file_id);
		let file_opt = Filesign::get_file_by_id(file_id);

		assert!(file_opt.is_some());
		let file = file_opt.unwrap();

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
		let file_id = generate_file_id();
		let account_id = 1;

		let _ = Filesign::create_new_file(Origin::signed(account_id), tag, filehash, Some(file_id));
		let sign_latest_version_result = Filesign::sign_latest_version(Origin::signed(account_id), file_id);
		let file_opt = Filesign::get_file_by_id(file_id);

		assert!(file_opt.is_some());
		let file = file_opt.unwrap();

		assert_ne!(sign_latest_version_result, DispatchResult::Ok(()));
		// Assert that no sign has been added
		assert_eq!(0, file.versions.last().unwrap().signatures.len());
	});
}