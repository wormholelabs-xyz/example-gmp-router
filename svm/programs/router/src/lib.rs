use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("7qtLhNMdb9dNAWwFvNBMok64EJrS1toY9TQoedVhU1xp");

/// The main program module for the GMP Router
#[program]
pub mod router {
    use super::*;

    /// Registers an integrator and initializes their configuration
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `RegisterArgs` struct containing:
    ///     * `integrator_program_id` - The program ID of the integrator
    ///     * `integrator_program_pda_bump` - The bump for the integrator_program_pda derivation
    pub fn register(ctx: Context<Register>, args: RegisterArgs) -> Result<()> {
        instructions::register::register(ctx, args)
    }

    /// Registers a new transceiver for an integrator
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `EnableTransceiverArgs` struct containing:
    ///     * `integrator_program` - The program id of the integrator_program
    ///     * `transceiver_program_id` - The address of the transceiver to register
    pub fn add_transceiver(ctx: Context<AddTransceiver>, args: AddTransceiverArgs) -> Result<()> {
        instructions::add_transceiver::add_transceiver(ctx, args)
    }

    /// Sets a transceiver as a receive transceiver for a specific chain
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `EnableTransceiverArgs` struct containing:
    ///     * `chain_id` - The ID of the chain for which the transceiver is being set
    ///     * `transceiver` - The Pubkey of the transceiver to be set
    ///     * `integrator_program` - The Pubkey of the integrator program
    pub fn enable_recv_transceiver(
        ctx: Context<EnableTransceiver>,
        args: TransceiverInfoArgs,
    ) -> Result<()> {
        instructions::enable_transceiver::enable_recv_transceiver(ctx, args)
    }

    /// Sets a transceiver as a send transceiver for a specific chain
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `EnableTransceiverArgs` struct containing:
    ///     * `chain_id` - The ID of the chain for which the transceiver is being set
    ///     * `transceiver` - The Pubkey of the transceiver to be set
    ///     * `integrator_program` - The Pubkey of the integrator program
    pub fn enable_send_transceiver(
        ctx: Context<EnableTransceiver>,
        args: TransceiverInfoArgs,
    ) -> Result<()> {
        instructions::enable_transceiver::enable_send_transceiver(ctx, args)
    }

    /// Disables a receive transceiver for a specific chain
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `DisableTransceiverArgs` struct containing:
    ///     * `chain_id` - The ID of the chain for which the transceiver is being disabled
    ///     * `transceiver` - The Pubkey of the transceiver to be disabled
    ///     * `integrator_program` - The Pubkey of the integrator program
    pub fn disable_recv_transceiver(
        ctx: Context<DisableTransceiver>,
        args: TransceiverInfoArgs,
    ) -> Result<()> {
        instructions::disable_transceiver::disable_recv_transceiver(ctx, args)
    }

    /// Disables a send transceiver for a specific chain
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `DisableTransceiverArgs` struct containing:
    ///     * `chain_id` - The ID of the chain for which the transceiver is being disabled
    ///     * `transceiver` - The Pubkey of the transceiver to be disabled
    ///     * `integrator_program` - The Pubkey of the integrator program
    pub fn disable_send_transceiver(
        ctx: Context<DisableTransceiver>,
        args: TransceiverInfoArgs,
    ) -> Result<()> {
        instructions::disable_transceiver::disable_send_transceiver(ctx, args)
    }

    /// Updates the admin of an IntegratorConfig account
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction, containing:
    ///     * `admin` - The current admin (signer)
    ///     * `integrator_config` - The IntegratorConfig account to update
    /// * `args` - The `UpdateAdminArgs` struct containing:
    ///     * `new_admin` - The public key of the new admin
    ///     * `integrator_program_id` - The program ID of the integrator
    pub fn update_admin(ctx: Context<UpdateAdmin>, args: UpdateAdminArgs) -> Result<()> {
        instructions::update_admin::update_admin(ctx, args)
    }

    /// Initiates the transfer of admin rights for an IntegratorConfig account
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    /// * `args` - The `TransferAdminArgs` struct containing:
    ///     * `new_admin` - The public key of the new admin
    ///     * `integrator_program_id` - The program ID of the integrator
    pub fn transfer_admin(ctx: Context<TransferAdmin>, args: TransferAdminArgs) -> Result<()> {
        instructions::transfer_admin::transfer_admin(ctx, args)
    }

    /// Claims the admin rights for an IntegratorConfig account
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    pub fn claim_admin(ctx: Context<ClaimAdmin>) -> Result<()> {
        instructions::transfer_admin::claim_admin(ctx)
    }

    /// Discards the admin role for an IntegratorConfig account, making it immutable
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the instruction
    pub fn discard_admin(ctx: Context<DiscardAdmin>) -> Result<()> {
        instructions::discard_admin::discard_admin(ctx)
    }
}
