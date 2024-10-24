#![cfg(feature = "test-sbf")]

mod common;
mod instructions;

use crate::instructions::add_transceiver::add_transceiver;
use crate::instructions::discard_admin::discard_admin;
use crate::instructions::enable_transceiver::{enable_recv_transceiver, enable_send_transceiver};
use crate::instructions::register::register;

use crate::instructions::transfer_admin::transfer_admin;
use anchor_lang::prelude::*;
use common::setup::{get_account, setup};
use router::error::RouterError;
use router::{
    state::{IntegratorChainConfig, IntegratorConfig, TransceiverInfo},
    utils::bitmap::Bitmap,
};
use solana_program_test::*;
use solana_sdk::{
    instruction::InstructionError, signature::Keypair, signer::Signer,
    transaction::TransactionError,
};

async fn initialize_test_environment(
    context: &mut ProgramTestContext,
) -> (Keypair, Pubkey, Pubkey, Pubkey, Pubkey, Pubkey, u16) {
    let payer = context.payer.insecure_clone();
    let admin = Keypair::new();
    let integrator_program_id = mock_integrator::id();
    let chain_id: u16 = 1;

    let (integrator_config_pda, _) = IntegratorConfig::pda(&mock_integrator::id());

    register(
        context,
        &payer,
        &admin,
        integrator_config_pda,
        mock_integrator::id(),
    )
    .await
    .unwrap();

    // Prepare integrator_chain_config_pda
    let (integrator_chain_config_pda, _) =
        IntegratorChainConfig::pda(&integrator_program_id, chain_id);

    // Register a transceiver
    let transceiver_program_id = Keypair::new().pubkey();
    let (registered_transceiver_pda, _) =
        TransceiverInfo::pda(&integrator_program_id, &transceiver_program_id);

    add_transceiver(
        context,
        &admin,
        &payer,
        integrator_config_pda,
        registered_transceiver_pda,
        integrator_program_id,
        transceiver_program_id,
    )
    .await
    .unwrap();

    (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver_program_id,
        chain_id,
    )
}

async fn verify_transceiver_state(
    context: &mut ProgramTestContext,
    integrator_chain_config_pda: Pubkey,
    expected_recv_bitmap: u128,
    expected_send_bitmap: u128,
    expected_chain_id: u16,
    expected_integrator_id: Pubkey,
) {
    let integrator_chain_config: IntegratorChainConfig =
        get_account(&mut context.banks_client, integrator_chain_config_pda).await;

    assert_eq!(
        integrator_chain_config.recv_transceiver_bitmap,
        Bitmap::from_value(expected_recv_bitmap)
    );
    assert_eq!(
        integrator_chain_config.send_transceiver_bitmap,
        Bitmap::from_value(expected_send_bitmap)
    );
    assert_eq!(integrator_chain_config.chain_id, expected_chain_id);
    assert_eq!(
        integrator_chain_config.integrator_program_id,
        expected_integrator_id
    );
}

#[tokio::test]
async fn test_enable_in_transceivers_success() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    let payer = context.payer.insecure_clone();

    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;
    assert!(result.is_ok());

    verify_transceiver_state(
        &mut context,
        integrator_chain_config_pda,
        1,
        0,
        chain_id,
        integrator_program_id,
    )
    .await;
}

#[tokio::test]
async fn test_enable_in_transceivers_multiple_sets_success() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    let payer = context.payer.insecure_clone();

    // Set the first transceiver
    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;
    assert!(result.is_ok());

    // Register a second transceiver
    let transceiver2_address = Pubkey::new_unique();
    let (registered_transceiver2_pda, _) =
        TransceiverInfo::pda(&integrator_program_id, &transceiver2_address);

    add_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        registered_transceiver2_pda,
        integrator_program_id,
        transceiver2_address,
    )
    .await
    .unwrap();

    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver2_pda,
        chain_id,
        transceiver2_address,
        integrator_program_id,
    )
    .await;
    assert!(result.is_ok());

    // Verify that both transceivers are set

    verify_transceiver_state(
        &mut context,
        integrator_chain_config_pda,
        3,
        0,
        chain_id,
        integrator_program_id,
    )
    .await;
}

#[tokio::test]
async fn test_enable_out_transceivers_success() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    let payer = context.payer.insecure_clone();

    let result = enable_send_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;

    assert!(result.is_ok());

    verify_transceiver_state(
        &mut context,
        integrator_chain_config_pda,
        0,
        1,
        chain_id,
        integrator_program_id,
    )
    .await;
}

#[tokio::test]
async fn test_enable_transceiver_invalid_admin() {
    let mut context = setup().await;
    let (
        _admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    // Create a new keypair to act as an invalid admin
    let invalid_admin = Keypair::new();
    let payer = context.payer.insecure_clone();

    let result = enable_recv_transceiver(
        &mut context,
        &invalid_admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;

    // The transaction should fail due to invalid admin
    let err = result.unwrap_err();

    assert_eq!(
        err.unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(RouterError::CallerNotAuthorized.into())
        )
    );
}

#[tokio::test]
async fn test_enable_transceiver_invalid_transceiver_id() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        _transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    // Use an invalid transceiver pubkey
    let invalid_transceiver = Keypair::new().pubkey();
    let payer = context.payer.insecure_clone();

    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        invalid_transceiver,
        integrator_program_id,
    )
    .await;

    // The transaction should fail due to invalid transceiver id
    // It will return AccountNotInitialized because the transceiver is not registered
    let err = result.unwrap_err();

    assert_eq!(
        err.unwrap(),
        TransactionError::InstructionError(0, InstructionError::Custom(2006))
    );
}

#[tokio::test]
async fn test_enable_already_enabled_transceiver() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    let payer = context.payer.insecure_clone();

    // First attempt: should succeed
    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;
    assert!(result.is_ok());

    verify_transceiver_state(
        &mut context,
        integrator_chain_config_pda,
        1,
        0,
        chain_id,
        integrator_program_id,
    )
    .await;

    // Second attempt: should fail with TransceiverAlreadyEnabled
    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(
        err.unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(RouterError::TransceiverAlreadyEnabled.into())
        )
    );

    // Verify that the state hasn't changed
    verify_transceiver_state(
        &mut context,
        integrator_chain_config_pda,
        1,
        0,
        chain_id,
        integrator_program_id,
    )
    .await;
}

#[tokio::test]
async fn test_enable_transceiver_with_transfer_in_progress() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    let payer = context.payer.insecure_clone();
    let pending_admin = Keypair::new();

    // Initiate an admin transfer
    transfer_admin(
        &mut context,
        &admin,
        &pending_admin.pubkey(),
        &payer,
        integrator_config_pda,
        integrator_program_id,
    )
    .await
    .unwrap();

    // Now try to enable the receive transceiver
    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(RouterError::AdminTransferInProgress.into())
        )
    );

    // Verify that the IntegratorChainConfig account doesn't exist
    let chain_config_account = context
        .banks_client
        .get_account(integrator_chain_config_pda)
        .await
        .expect("Failed to get account");
    assert!(
        chain_config_account.is_none(),
        "IntegratorChainConfig account should not exist"
    );

    // Try to enable the send transceiver
    let result = enable_send_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(RouterError::AdminTransferInProgress.into())
        )
    );

    // Verify that the IntegratorChainConfig account still doesn't exist
    let chain_config_account = context
        .banks_client
        .get_account(integrator_chain_config_pda)
        .await
        .expect("Failed to get account");
    assert!(
        chain_config_account.is_none(),
        "IntegratorChainConfig account should not exist"
    );
}

#[tokio::test]
async fn test_enable_transceiver_with_immutable_config() {
    let mut context = setup().await;
    let (
        admin,
        integrator_program_id,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        transceiver,
        chain_id,
    ) = initialize_test_environment(&mut context).await;

    let payer = context.payer.insecure_clone();

    // Discard the admin to make the config immutable
    discard_admin(&mut context, &admin, &payer, integrator_config_pda)
        .await
        .unwrap();

    // Now try to enable the receive transceiver
    let result = enable_recv_transceiver(
        &mut context,
        &admin,
        &payer,
        integrator_config_pda,
        integrator_chain_config_pda,
        registered_transceiver_pda,
        chain_id,
        transceiver,
        integrator_program_id,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(RouterError::CallerNotAuthorized.into())
        )
    );

    // Verify that the integrator config is immutable
    let integrator_config: IntegratorConfig =
        get_account(&mut context.banks_client, integrator_config_pda).await;
    assert_eq!(integrator_config.admin, None);
}
