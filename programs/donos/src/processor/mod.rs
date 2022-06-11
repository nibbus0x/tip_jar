pub mod context;

use crate::{constant::*, error::ErrorCode, state::*, util::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use context::*;

pub fn initialize_tip_jar(
    ctx: Context<InitializeJar>,
    tip_percentage: u16,
    tippees: Option<Vec<Tippee>>,
) -> Result<()> {
    let tip_jar_state = &mut ctx.accounts.tip_jar_state;
    let tip_jar_wallet = &mut ctx.accounts.tip_jar_wallet;
    let wallet = &mut ctx.accounts.wallet;
    let rent = &mut ctx.accounts.rent;

    verify_tip_percentage(tip_percentage)?;
    let tippees_res = verify_tippees(tippees)?;

    tip_jar_state.owner = wallet.key();
    tip_jar_state.tip_percentage = tip_percentage;
    tip_jar_state.tippees = tippees_res.0;
    tip_jar_state.num_tippees = tippees_res.1;

    let tj_wallet_balance = tip_jar_wallet.lamports();
    let rent_exempt_bal = rent.minimum_balance(tip_jar_wallet.data_len());

    if tj_wallet_balance < rent_exempt_bal {
        invoke(
            &system_instruction::transfer(
                wallet.key,
                tip_jar_wallet.key,
                rent_exempt_bal
                    .checked_sub(tj_wallet_balance)
                    .ok_or(ErrorCode::NumericalOverflow)?,
            ),
            &[wallet.to_account_info(), tip_jar_wallet.to_account_info()],
        )?;
    }

    Ok(())
}

pub fn update_tip_percentage(
    ctx: Context<UpdateTipJarState>,
    new_tip_percentage: u16,
) -> Result<()> {
    let tip_jar_state = &mut ctx.accounts.tip_jar_state;

    verify_tip_percentage(new_tip_percentage)?;

    tip_jar_state.tip_percentage = new_tip_percentage;

    Ok(())
}

pub fn update_tippees(ctx: Context<UpdateTipJarState>, tippees: Option<Vec<Tippee>>) -> Result<()> {
    let tip_jar_state = &mut ctx.accounts.tip_jar_state;

    let tippees_res = verify_tippees(tippees)?;

    tip_jar_state.tippees = tippees_res.0;
    tip_jar_state.num_tippees = tippees_res.1;

    Ok(())
}

pub fn deposit_to_jar(ctx: Context<TipJarTransfer>, amount: u64) -> Result<()> {
    let tip_jar_wallet = &mut ctx.accounts.tip_jar_wallet;
    let wallet = &mut ctx.accounts.wallet;

    invoke(
        &system_instruction::transfer(wallet.key, tip_jar_wallet.key, amount),
        &[wallet.to_account_info(), tip_jar_wallet.to_account_info()],
    )?;

    Ok(())
}

pub fn withdraw_from_jar(ctx: Context<TipJarTransfer>, amount: u64) -> Result<()> {
    let tip_jar_wallet = &mut ctx.accounts.tip_jar_wallet;
    let wallet = &mut ctx.accounts.wallet;
    let rent = &ctx.accounts.rent;
    let tip_jar_wallet_bump = *ctx
        .bumps
        .get("tip_jar_wallet")
        .ok_or(ErrorCode::MissingBump)?;

    let withdraw_amount = get_rent_checked_amount(&tip_jar_wallet, rent, amount)?;

    invoke_signed(
        &system_instruction::transfer(tip_jar_wallet.key, wallet.key, withdraw_amount),
        &[wallet.to_account_info(), tip_jar_wallet.to_account_info()],
        &[&[
            TIP_JAR.as_bytes(),
            wallet.key().as_ref(),
            WALLET.as_bytes(),
            &[tip_jar_wallet_bump],
        ]],
    )?;

    Ok(())
}

pub fn tip_from_jar(ctx: Context<Tip>, amount: u64) -> Result<()> {
    let tip_jar_wallet = &mut ctx.accounts.tip_jar_wallet;
    let receiver_account = &mut ctx.accounts.receiver_account;
    let receiver_jar_state = &mut ctx.accounts.receiver_jar_state;
    let receiver_jar_wallet = &mut ctx.accounts.receiver_jar_wallet;
    let wallet = &mut ctx.accounts.wallet;
    let rent = &ctx.accounts.rent;
    let tip_jar_wallet_bump = *ctx
        .bumps
        .get("tip_jar_wallet")
        .ok_or(ErrorCode::MissingBump)?;

    let is_receiver_jar_init = check_receiver_jar_init(&receiver_jar_state);

    let (to_account_key, to_account_info): (Pubkey, AccountInfo) = match is_receiver_jar_init {
        true => (
            receiver_jar_wallet.key(),
            receiver_jar_wallet.to_account_info(),
        ),
        false => (receiver_account.key(), receiver_account.to_account_info()),
    };

    let checked_amount = get_rent_checked_amount(&tip_jar_wallet, rent, amount)?;

    invoke_signed(
        &system_instruction::transfer(tip_jar_wallet.key, &to_account_key, checked_amount),
        &[tip_jar_wallet.to_account_info(), to_account_info],
        &[&[
            TIP_JAR.as_bytes(),
            wallet.key().as_ref(),
            WALLET.as_bytes(),
            &[tip_jar_wallet_bump],
        ]],
    )?;

    Ok(())
}

pub fn tip_from_wallet(ctx: Context<Tip>, amount: u64) -> Result<()> {
    let tip_jar_wallet = &mut ctx.accounts.tip_jar_wallet;
    let receiver_account = &mut ctx.accounts.receiver_account;
    let receiver_jar_state = &mut ctx.accounts.receiver_jar_state;
    let receiver_jar_wallet = &mut ctx.accounts.receiver_jar_wallet;
    let wallet = &mut ctx.accounts.wallet;

    let is_receiver_jar_init = check_receiver_jar_init(&receiver_jar_state);

    let (to_account_key, to_account_info): (Pubkey, AccountInfo) = match is_receiver_jar_init {
        true => (
            receiver_jar_wallet.key(),
            receiver_jar_wallet.to_account_info(),
        ),
        false => (receiver_account.key(), receiver_account.to_account_info()),
    };

    invoke(
        &system_instruction::transfer(wallet.key, &to_account_key, amount),
        &[tip_jar_wallet.to_account_info(), to_account_info],
    )?;

    Ok(())
}
