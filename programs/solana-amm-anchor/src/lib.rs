pub mod events;

use anchor_lang::prelude::*;

pub use events::*;

declare_id!("BVEgvpd2ajpJkMAewAuVGLM7dgXCNkVjbzDCCYhmFouA");

#[program]
pub mod solana_amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed:u64, fee:u16,authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, &ctx.bumps)?;

        emit!(InitializeEvent{
            admin: ctx.accounts.admin.key(),
            mint_x: ctx.accounts.mint_x.key(),
            mint_y: ctx.accounts.mint_y.key(),
            mint_lp: ctx.accounts.mint_lp.key(),
            vault_x: ctx.accounts.vault_x.key(),
            vault_y: ctx.accounts.vault_y.key(),
            config: ctx.accounts.config.key(),
            fee: fee
        })
    }
}

#[derive(Accounts)]
pub struct Initialize {}
