use anchor_lang::prelude::*;
use crate::wallet_whitelist::*;
use crate::whitelist_config::*;
use crate::MagicHatError;
use crate::constants::MAGIC_HAT_CREATOR_WALLET;
use std::str::FromStr;


#[derive(Accounts)]
pub struct CreateWhitelistAccount<'info> {
    #[account(init, 
        payer = magic_hat_creator, 
        space = 8 + std::mem::size_of::<WalletWhitelist>(),
        seeds = [b"wallet-whitelist".as_ref(), whitelisted_address.key().as_ref(), magic_hat_creator.key().as_ref()], 
        bump
    )]
    wallet_whitelist: Account<'info, WalletWhitelist>,
    #[account(has_one = magic_hat_creator)]
    whitelist_config: Account<'info, WhitelistConfig>,
    /// CHECK:
    whitelisted_address: AccountInfo<'info>,
    #[account(mut, address = Pubkey::from_str(MAGIC_HAT_CREATOR_WALLET).unwrap())]
    magic_hat_creator: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler_create_whitelist_account(ctx: Context<CreateWhitelistAccount>, whitelist_type: String) -> Result<()>{
    let wallet_whitelist = &mut ctx.accounts.wallet_whitelist;
    let whitelist_config = &ctx.accounts.whitelist_config;
    wallet_whitelist.magic_hat_creator = ctx.accounts.magic_hat_creator.key();
    wallet_whitelist.whitelisted_address = ctx.accounts.whitelisted_address.key();
    match whitelist_type.as_str() {
        "One" => {
            wallet_whitelist.whitelist_type = WLType::One;
            wallet_whitelist.number_of_whitelist_spots_per_user = 1;
            wallet_whitelist.discounted_mint_price = whitelist_config.whitelist_schedule.wl_start_time_1.discounted_mint_price;
            wallet_whitelist.whitelist_mint_start_time = whitelist_config.whitelist_schedule.wl_start_time_1.whitelist_mint_start_time;
        },
        "Two" => {
            wallet_whitelist.whitelist_type = WLType::Two;
            wallet_whitelist.number_of_whitelist_spots_per_user = 2;
            wallet_whitelist.discounted_mint_price = whitelist_config.whitelist_schedule.wl_start_time_2.discounted_mint_price;
            wallet_whitelist.whitelist_mint_start_time = whitelist_config.whitelist_schedule.wl_start_time_2.whitelist_mint_start_time;
        },
        "Three" => {
            wallet_whitelist.whitelist_type = WLType::Three;
            wallet_whitelist.number_of_whitelist_spots_per_user = 4;
            wallet_whitelist.discounted_mint_price = whitelist_config.whitelist_schedule.wl_start_time_3.discounted_mint_price;
            wallet_whitelist.whitelist_mint_start_time = whitelist_config.whitelist_schedule.wl_start_time_3.whitelist_mint_start_time;
        },
        "Four" => {
            wallet_whitelist.whitelist_type = WLType::Four;
            wallet_whitelist.number_of_whitelist_spots_per_user = 5;
            wallet_whitelist.discounted_mint_price = whitelist_config.whitelist_schedule.wl_start_time_4.discounted_mint_price;
            wallet_whitelist.whitelist_mint_start_time = whitelist_config.whitelist_schedule.wl_start_time_4.whitelist_mint_start_time;
        },
        _ => {
            wallet_whitelist.whitelist_type = WLType::Null;
            wallet_whitelist.number_of_whitelist_spots_per_user = 0;
            wallet_whitelist.discounted_mint_price = u64::MAX;
            wallet_whitelist.whitelist_mint_start_time = u64::MAX;
            return Err(error!(MagicHatError::InvalidWLType))
        },
    }
    Ok(())
}
