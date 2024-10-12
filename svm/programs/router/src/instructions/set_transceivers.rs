use crate::error::RouterError;
use crate::state::{IntegratorChainTransceivers, IntegratorConfig, RegisteredTransceiver};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetTransceiverArgs {
    pub chain_id: u16,
}

#[derive(Accounts)]
#[instruction(args: SetTransceiverArgs)]
pub struct SetTransceiver<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub owner: Signer<'info>,

    #[account(
        seeds = [IntegratorConfig::SEED_PREFIX, integrator_program.key().as_ref()],
        bump = integrator_config.bump,
        has_one = owner @ RouterError::InvalidIntegratorAuthority,
    )]
    pub integrator_config: Account<'info, IntegratorConfig>,

    #[account(
        mut,
        seeds = [
            IntegratorChainTransceivers::SEED_PREFIX,
            integrator_program.key().as_ref(),
            args.chain_id.to_le_bytes().as_ref(),
        ],
        bump = integrator_chain_transceivers.bump,
    )]
    pub integrator_chain_transceivers: Account<'info, IntegratorChainTransceivers>,

    #[account(
        seeds = [
            RegisteredTransceiver::SEED_PREFIX,
            integrator_program.key().as_ref(),
            transceiver.key().as_ref(),
        ],
        bump = registered_transceiver.bump,
    )]
    pub registered_transceiver: Account<'info, RegisteredTransceiver>,

    /// CHECK: This account is not read or written in this instruction
    pub integrator_program: UncheckedAccount<'info>,

    /// The transceiver account being set
    pub transceiver: AccountInfo<'info>,
}

pub fn set_recv_transceiver(ctx: Context<SetTransceiver>, args: SetTransceiverArgs) -> Result<()> {
    let registered_transceiver = &ctx.accounts.registered_transceiver;
    let integrator_chain_transceivers = &mut ctx.accounts.integrator_chain_transceivers;

    // Convert usize to u8, panicking if the value doesn't fit
    let transceiver_id = registered_transceiver.id.try_into().unwrap();

    // Set the bit corresponding to the registered_transceiver id
    integrator_chain_transceivers
        .recv_transceiver_bitmap
        .set(transceiver_id, true)?;

    Ok(())
}

pub fn disable_recv_transceiver(
    ctx: Context<SetTransceiver>,
    args: SetTransceiverArgs,
) -> Result<()> {
    let registered_transceiver = &ctx.accounts.registered_transceiver;
    let integrator_chain_transceivers = &mut ctx.accounts.integrator_chain_transceivers;

    // Convert usize to u8, panicking if the value doesn't fit
    let transceiver_id = registered_transceiver.id.try_into().unwrap();

    // Clear the bit corresponding to the registered_transceiver id
    integrator_chain_transceivers
        .recv_transceiver_bitmap
        .set(transceiver_id, false)?;

    Ok(())
}

pub fn set_send_transceiver(ctx: Context<SetTransceiver>, args: SetTransceiverArgs) -> Result<()> {
    let registered_transceiver = &ctx.accounts.registered_transceiver;
    let integrator_chain_transceivers = &mut ctx.accounts.integrator_chain_transceivers;

    // Convert usize to u8, panicking if the value doesn't fit
    let transceiver_id = registered_transceiver.id.try_into().unwrap();

    // Set the bit corresponding to the registered_transceiver id
    integrator_chain_transceivers
        .send_transceiver_bitmap
        .set(transceiver_id, true)?;

    Ok(())
}

pub fn disable_send_transceiver(
    ctx: Context<SetTransceiver>,
    args: SetTransceiverArgs,
) -> Result<()> {
    let registered_transceiver = &ctx.accounts.registered_transceiver;
    let integrator_chain_transceivers = &mut ctx.accounts.integrator_chain_transceivers;

    // Convert usize to u8, panicking if the value doesn't fit
    let transceiver_id = registered_transceiver.id.try_into().unwrap();

    // Clear the bit corresponding to the registered_transceiver id
    integrator_chain_transceivers
        .send_transceiver_bitmap
        .set(transceiver_id, false)?;

    Ok(())
}