use anchor_lang::prelude::*;

#[event]
pub struct InitializeEvent{
    pub admin: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub mint_lp: Pubkey,
    pub vault_x: Pubkey,
    pub vault_y: Pubkey,
    pub config: Pubkey,
    pub fee: u16,
}