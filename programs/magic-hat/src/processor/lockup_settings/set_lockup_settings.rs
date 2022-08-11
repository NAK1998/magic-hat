use anchor_lang::prelude::*;

use crate::{
    constants::LOCKUP_SETTINGS_FEATURE_INDEX,
    set_feature_flag,
    state::{LOCKUP_SETTINGS_SEED, LOCKUP_SETTINGS_SIZE},
    LockupSettings, LockupType, MagicHat, MagicHatError,
};

/// Set the collection PDA for the magic hat
#[derive(Accounts)]
pub struct SetLockupSettings<'info> {
    #[account(mut, has_one = authority)]
    magic_hat: Account<'info, MagicHat>,
    authority: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [LOCKUP_SETTINGS_SEED.as_bytes(), magic_hat.to_account_info().key.as_ref()],
        space = LOCKUP_SETTINGS_SIZE,
        payer = payer,
        bump
    )]
    lockup_settings: Box<Account<'info, LockupSettings>>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handle_set_lockup_settings(
    ctx: Context<SetLockupSettings>,
    lockup_type: u8,
    number: i64,
) -> Result<()> {
    let magic_hat = &mut ctx.accounts.magic_hat;
    let lockup_settings = &mut ctx.accounts.lockup_settings;
    if lockup_type != LockupType::DurationSeconds as u8
        && lockup_type != LockupType::ExpirationUnixTimstamp as u8
    {
        return err!(MagicHatError::InvalidLockupType);
    }
    lockup_settings.magic_hat = magic_hat.key();
    lockup_settings.lockup_type = lockup_type;
    lockup_settings.number = number;
    set_feature_flag(&mut magic_hat.data.uuid, LOCKUP_SETTINGS_FEATURE_INDEX);
    Ok(())
}
