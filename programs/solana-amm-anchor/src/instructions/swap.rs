use anchor_lang::prelude::*;

use anchor_lang::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::AmmError, state::Config};
use constant_product_curve::{ConstantProduct, LiquidityPair};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[b"lp", config.key().as_ref()],
        bump=config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
        has_one=mint_x,
        has_one=mint_y,
    )]
    pub config: Account<'info, Config>,
}
