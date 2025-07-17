use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

}