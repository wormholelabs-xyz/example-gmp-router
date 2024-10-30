use anchor_lang::prelude::*;
use router::program::Router;
use router::{self};
use router::{
    cpi::accounts::{RecvMessage, Register, SendMessage},
    instructions::{RegisterArgs, SendMessageArgs},
};
use universal_address::UniversalAddress;

declare_id!("B86KSKnHBRiJeDcP7vwaXuxfkqfChZmYKBqh4dkLYEpj");

/// This module serves as a mock integrator to demonstrate how to call the register function
/// in the router program. It's designed to simulate the process of registering an integrator,
/// which requires a Cross-Program Invocation (CPI) call with a Program Derived Address (PDA) signer.

#[program]
pub mod mock_integrator {
    use super::*;

    /// Invokes the register function in the router program via a CPI call.
    /// This function demonstrates how to properly set up the accounts and sign the transaction
    /// using a PDA, which is required for the registration process.
    pub fn invoke_register(ctx: Context<InvokeRegister>, args: InvokeRegisterArgs) -> Result<()> {
        let bump_seed = &[ctx.bumps.integrator_program_pda][..];
        let signer_seeds: &[&[&[u8]]] = &[&[b"router_integrator", bump_seed]];

        router::cpi::register(
            ctx.accounts.invoke_register().with_signer(signer_seeds),
            RegisterArgs {
                integrator_program_pda_bump: ctx.bumps.integrator_program_pda,
                integrator_program_id: crate::ID,
                admin: args.admin,
            },
        )?;
        Ok(())
    }

    pub fn invoke_send_message(
        ctx: Context<InvokeSendMessage>,
        args: InvokeSendMessageArgs,
    ) -> Result<()> {
        let bump_seed = &[ctx.bumps.integrator_program_pda][..];
        let signer_seeds: &[&[&[u8]]] = &[&[b"router_integrator", bump_seed]];

        router::cpi::send_message(
            ctx.accounts.invoke_send_message().with_signer(signer_seeds),
            SendMessageArgs {
                integrator_program_id: crate::ID,
                integrator_program_pda_bump: ctx.bumps.integrator_program_pda,
                dst_chain: args.dst_chain,
                dst_addr: args.dst_addr,
                payload_hash: args.payload_hash,
            },
        )?;
        Ok(())
    }

    /// Invokes the recv_message instruction on the router program via CPI
    pub fn invoke_recv_message(
        ctx: Context<InvokeRecvMessage>,
        args: router::instructions::RecvMessageArgs,
    ) -> Result<(u128, u128)> {
        // Prepare the seeds for PDA signing
        let bump_seed = &[ctx.bumps.integrator_program_pda][..];
        let signer_seeds: &[&[&[u8]]] = &[&[b"router_integrator", bump_seed]];

        // Perform the CPI call to the router program's recv_message instruction
        let result = router::cpi::recv_message(
            ctx.accounts.invoke_recv_message().with_signer(signer_seeds),
            args,
        )?;

        Ok(result.get())
    }

    /// Invokes the exec_message instruction on the router program via CPI
    pub fn invoke_exec_message(
        ctx: Context<InvokeExecMessage>,
        args: router::instructions::ExecMessageArgs,
    ) -> Result<()> {
        // Prepare the seeds for PDA signing
        let bump_seed = &[ctx.bumps.integrator_program_pda][..];
        let signer_seeds: &[&[&[u8]]] = &[&[b"router_integrator", bump_seed]];

        // Perform the CPI call to the router program's exec_message instruction
        router::cpi::exec_message(
            ctx.accounts.invoke_exec_message().with_signer(signer_seeds),
            args,
        )?;

        Ok(())
    }
}
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InvokeRegisterArgs {
    pub admin: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: InvokeRegisterArgs)]
pub struct InvokeRegister<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: This account is to be checked and initialized by the router program
    pub integrator_config: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account is to be checked and initialized by the router program
    pub sequence_tracker: UncheckedAccount<'info>,

    /// The integrator program's PDA
    #[account(
        seeds = [b"router_integrator"],
        bump,
    )]
    pub integrator_program_pda: SystemAccount<'info>,

    /// The System Program
    pub system_program: Program<'info, System>,

    pub router_program: Program<'info, Router>,
}

impl<'info> InvokeRegister<'info> {
    pub fn invoke_register(&self) -> CpiContext<'_, '_, '_, 'info, Register<'info>> {
        let cpi_program = self.router_program.to_account_info();
        let cpi_accounts = Register {
            payer: self.payer.to_account_info(),
            integrator_config: self.integrator_config.to_account_info(),
            integrator_program_pda: self.integrator_program_pda.to_account_info(),
            sequence_tracker: self.sequence_tracker.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InvokeSendMessageArgs {
    pub dst_chain: u16,
    pub dst_addr: UniversalAddress,
    pub payload_hash: [u8; 32],
}

#[derive(Accounts)]
pub struct InvokeSendMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"router_integrator"],
        bump,
    )]
    pub integrator_program_pda: SystemAccount<'info>,

    #[account(mut)]
    /// CHECK: This account is checked by the router program
    pub integrator_chain_config: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account is initialized by the router program
    pub outbox_message: Signer<'info>,

    #[account(mut)]
    /// CHECK: This account is checked by the router program
    pub sequence_tracker: UncheckedAccount<'info>,

    pub router_program: Program<'info, Router>,

    pub system_program: Program<'info, System>,
}

impl<'info> InvokeSendMessage<'info> {
    pub fn invoke_send_message(&self) -> CpiContext<'_, '_, '_, 'info, SendMessage<'info>> {
        let cpi_program = self.router_program.to_account_info();
        let cpi_accounts = SendMessage {
            integrator_program_pda: self.integrator_program_pda.to_account_info(),
            payer: self.payer.to_account_info(),
            integrator_chain_config: self.integrator_chain_config.to_account_info(),
            outbox_message: self.outbox_message.to_account_info(),
            sequence_tracker: self.sequence_tracker.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

/// Accounts struct for the invoke_recv_message instruction
#[derive(Accounts)]
pub struct InvokeRecvMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"router_integrator"],
        bump,
    )]
    pub integrator_program_pda: SystemAccount<'info>,

    /// The integrator chain config account
    /// CHECK: This account is checked by the router program
    pub integrator_chain_config: UncheckedAccount<'info>,

    /// The attestation info account
    /// CHECK: This account is checked by the router program
    #[account(mut)]
    pub attestation_info: UncheckedAccount<'info>,

    /// The system program
    pub system_program: Program<'info, System>,

    /// The router program
    pub router_program: Program<'info, Router>,
}

impl<'info> InvokeRecvMessage<'info> {
    /// Helper function to create the CpiContext for the recv_message instruction
    pub fn invoke_recv_message(&self) -> CpiContext<'_, '_, '_, 'info, RecvMessage<'info>> {
        let cpi_program = self.router_program.to_account_info();
        let cpi_accounts = RecvMessage {
            integrator_program_pda: self.integrator_program_pda.to_account_info(),
            payer: self.payer.to_account_info(),
            integrator_chain_config: self.integrator_chain_config.to_account_info(),
            attestation_info: self.attestation_info.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct InvokeExecMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"router_integrator"],
        bump,
    )]
    pub integrator_program_pda: SystemAccount<'info>,

    /// The attestation info account
    /// CHECK: This account is checked by the router program
    #[account(mut)]
    pub attestation_info: UncheckedAccount<'info>,

    /// The system program
    pub system_program: Program<'info, System>,

    /// The router program
    pub router_program: Program<'info, Router>,
}

impl<'info> InvokeExecMessage<'info> {
    /// Helper function to create the CpiContext for the exec_message instruction
    pub fn invoke_exec_message(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, router::cpi::accounts::ExecMessage<'info>> {
        let cpi_program = self.router_program.to_account_info();
        let cpi_accounts = router::cpi::accounts::ExecMessage {
            payer: self.payer.to_account_info(),
            integrator_program_pda: self.integrator_program_pda.to_account_info(),
            attestation_info: self.attestation_info.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
