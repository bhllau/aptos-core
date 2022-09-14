// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{assert_success, MoveHarness};
use aptos_crypto::SigningKey;
use aptos_types::account_config::AccountResource;
use aptos_types::{account_address::AccountAddress, account_config::CORE_CODE_ADDRESS};
use cached_packages::aptos_stdlib;
use move_deps::move_core_types::parser::parse_struct_tag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SignerCapabilityOfferProofChallenge {
    account_address: AccountAddress,
    module_name: String,
    struct_name: String,
    sequence_number: u64,
    recipient_address: AccountAddress,
}

#[derive(Serialize, Deserialize)]
struct RotationCapabilityOfferProofChallenge {
    account_address: AccountAddress,
    module_name: String,
    struct_name: String,
    sequence_number: u64,
    source_address: AccountAddress,
    recipient_address: AccountAddress,
}

#[test]
fn offer_signer_capability() {
    let mut harness = MoveHarness::new();

    let account1 = harness.new_account_at(AccountAddress::from_hex_literal("0x123").unwrap());
    let account2 = harness.new_account_at(AccountAddress::from_hex_literal("0x345").unwrap());

    let signer_capability_proof = SignerCapabilityOfferProofChallenge {
        account_address: CORE_CODE_ADDRESS,
        module_name: String::from("account"),
        struct_name: String::from("SignerCapabilityOfferProofChallenge"),
        sequence_number: 10,
        recipient_address: *account2.address(),
    };

    let signer_capability_proof_msg = bcs::to_bytes(&signer_capability_proof);
    let signer_proof_signed = account1
        .privkey
        .sign_arbitrary_message(&signer_capability_proof_msg.unwrap());

    assert_success!(harness.run_transaction_payload(
        &account1,
        aptos_stdlib::account_offer_signer_capability(
            signer_proof_signed.to_bytes().to_vec(),
            0,
            account1.pubkey.to_bytes().to_vec(),
            *account2.address(),
        )
    ));

    let account_resource = parse_struct_tag("0x1::account::Account").unwrap();
    assert_eq!(
        harness
            .read_resource::<AccountResource>(account1.address(), account_resource)
            .unwrap()
            .signer_capability_offer()
            .unwrap(),
        *account2.address()
    );
}

#[test]
fn offer_rotation_capability() {
    let mut harness = MoveHarness::new();

    let account1 = harness.new_account_at(AccountAddress::from_hex_literal("0x123").unwrap());
    let account2 = harness.new_account_at(AccountAddress::from_hex_literal("0x345").unwrap());

    let rotation_capability_proof = RotationCapabilityOfferProofChallenge {
        account_address: CORE_CODE_ADDRESS,
        module_name: String::from("account"),
        struct_name: String::from("RotationCapabilityOfferProofChallengeV2"),
        sequence_number: 10,
        source_address: *account1.address(),
        recipient_address: *account2.address(),
    };

    let rotation_capability_proof_msg = bcs::to_bytes(&rotation_capability_proof);
    let rotation_proof_signed = account1
        .privkey
        .sign_arbitrary_message(&rotation_capability_proof_msg.unwrap());

    assert_success!(harness.run_transaction_payload(
        &account1,
        aptos_stdlib::account_offer_rotation_capability(
            rotation_proof_signed.to_bytes().to_vec(),
            0,
            account1.pubkey.to_bytes().to_vec(),
            *account2.address(),
        )
    ));

    let account_resource = parse_struct_tag("0x1::account::Account").unwrap();
    assert_eq!(
        harness
            .read_resource::<AccountResource>(account1.address(), account_resource)
            .unwrap()
            .rotation_capability_offer()
            .unwrap(),
        *account2.address()
    );
}
