use anchor_lang::{InstructionData, ToAccountMetas};
use router::accounts::{ClaimAdmin, TransferAdmin};
use solana_program_test::*;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
};

use crate::common::execute_transaction::execute_transaction;

pub async fn transfer_admin(
    context: &mut ProgramTestContext,
    admin: &Keypair,
    new_admin: &Pubkey,
    payer: &Keypair,
    integrator_config: Pubkey,
    integrator_program_id: Pubkey,
) -> Result<(), BanksClientError> {
    let accounts = TransferAdmin {
        admin: admin.pubkey(),
        integrator_config,
    };

    let args = router::instructions::TransferAdminArgs {
        integrator_program_id,
        new_admin: *new_admin,
    };

    let ix = Instruction {
        program_id: router::id(),
        accounts: accounts.to_account_metas(None),
        data: router::instruction::TransferAdmin { args }.data(),
    };

    execute_transaction(context, ix, &[admin, payer], payer).await
}

pub async fn claim_admin(
    context: &mut ProgramTestContext,
    new_admin: &Keypair,
    payer: &Keypair,
    integrator_config: Pubkey,
) -> Result<(), BanksClientError> {
    let accounts = ClaimAdmin {
        new_admin: new_admin.pubkey(),
        integrator_config,
    };

    let ix = Instruction {
        program_id: router::id(),
        accounts: accounts.to_account_metas(None),
        data: router::instruction::ClaimAdmin {}.data(),
    };

    execute_transaction(context, ix, &[new_admin, payer], payer).await
}
