use anchor_lang::prelude::*;

declare_id!("BVEgvpd2ajpJkMAewAuVGLM7dgXCNkVjbzDCCYhmFouA");

#[program]
pub mod solana_amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
