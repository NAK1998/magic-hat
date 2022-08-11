use anchor_lang::prelude::*;

use crate::{
    constants::LOCKUP_SETTINGS_FEATURE_INDEX, remove_feature_flag, state::LOCKUP_SETTINGS_SEED,
    LockupSettings, MagicHat, MagicHatError,
};

/// Set the collection PDA for the magic hat
#[derive(Accounts)]
pub struct CloseLockupSettings<'info> {
    /// CHECK: account may be empty
    #[account(mut)]
    magic_hat: UncheckedAccount<'info>,
    authority: Signer<'info>,
    #[account(
        mut,
        close = authority,
        seeds = [LOCKUP_SETTINGS_SEED.as_bytes(), magic_hat.to_account_info().key.as_ref()],
        bump
    )]
    lockup_settings: Box<Account<'info, LockupSettings>>,
    system_program: Program<'info, System>,
}

pub fn handle_close_lockup_settings(ctx: Context<CloseLockupSettings>) -> Result<()> {
    let magic_hat_info = &ctx.accounts.magic_hat;
    if !magic_hat_info.data_is_empty() {
        let magic_hat = &mut Account::<MagicHat>::try_from(magic_hat_info)?;
        if magic_hat.authority != ctx.accounts.authority.key() {
            return err!(MagicHatError::InvalidMagicHatAuthority);
        }
        remove_feature_flag(&mut magic_hat.data.uuid, LOCKUP_SETTINGS_FEATURE_INDEX);
    }
    Ok(())
}
