use crate::{constant::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeJar<'info> {
    #[account(
      init,
      payer = wallet,
      space = TIP_JAR_ACCOUNT_SIZE,
      seeds = [
        TIP_JAR.as_bytes(),
        wallet.key().as_ref(),
        STATE.as_bytes(),
      ],
      bump,
    )]
    pub tip_jar_state: Account<'info, TipJarState>,
    #[account(
      mut,
      seeds = [
        TIP_JAR.as_bytes(),
        wallet.key().as_ref(),
        WALLET.as_bytes(),
      ],
      bump
    )]
    pub tip_jar_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateTipJarState<'info> {
    #[account(
    seeds = [
      TIP_JAR.as_bytes(),
      wallet.key().as_ref(),
      STATE.as_bytes(),
    ],
    bump,
  )]
    pub tip_jar_state: Account<'info, TipJarState>,
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TipJarTransfer<'info> {
    #[account(
      seeds = [
        TIP_JAR.as_bytes(),
        wallet.key().as_ref(),
        STATE.as_bytes(),
      ],
      bump,
    )]
    pub tip_jar_state: Account<'info, TipJarState>,
    #[account(
      mut,
      seeds = [
        TIP_JAR.as_bytes(),
        wallet.key().as_ref(),
        WALLET.as_bytes(),
      ],
      bump
    )]
    pub tip_jar_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Tip<'info> {
    // System account of receiver
    #[account(mut)]
    pub receiver_account: SystemAccount<'info>,
    /// CHECK: Not dangerous, seeds checked
    #[account(
    mut,
    seeds = [
      TIP_JAR.as_bytes(),
      receiver_account.key().as_ref(),
      STATE.as_bytes(),
    ],
    bump
  )]
    pub receiver_jar_state: UncheckedAccount<'info>,
    #[account(
    mut,
    seeds = [
      TIP_JAR.as_bytes(),
      receiver_account.key().as_ref(),
      WALLET.as_bytes(),
    ],
    bump
  )]
    pub receiver_jar_wallet: SystemAccount<'info>,
    #[account(
    seeds = [
      TIP_JAR.as_bytes(),
      wallet.key().as_ref(),
      STATE.as_bytes(),
    ],
    bump,
  )]
    pub tip_jar_state: Account<'info, TipJarState>,
    #[account(
    mut,
    seeds = [
      TIP_JAR.as_bytes(),
      wallet.key().as_ref(),
      WALLET.as_bytes(),
    ],
    bump
  )]
    pub tip_jar_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
