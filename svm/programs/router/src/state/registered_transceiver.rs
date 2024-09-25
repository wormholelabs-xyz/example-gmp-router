use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RegisteredTransceiver {
    pub bump: u8,
    pub integrator_id: u16,
    pub id: u16,
    pub address: Pubkey,
}

impl RegisteredTransceiver {
    pub const SEED_PREFIX: &'static [u8] = b"registered_transceiver";
}