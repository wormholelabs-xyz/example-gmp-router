use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub bump: u8,
    pub owner: Pubkey,
    pub paused: bool,
    pub next_integrator_id: u16,
}
