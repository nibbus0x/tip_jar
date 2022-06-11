use crate::{constant::*, error::ErrorCode, state::*};
use anchor_lang::prelude::*;

pub fn get_rent_checked_amount(account: &AccountInfo, rent: &Rent, amount: u64) -> Result<u64> {
    let rent_exempt_balance = rent.minimum_balance(account.data_len());
    let mut checked_amount: u64 = amount;

    if account
        .lamports()
        .checked_sub(amount)
        .ok_or(ErrorCode::NumericalOverflow)?
        < rent_exempt_balance
    {
        checked_amount = account
            .lamports()
            .checked_sub(rent_exempt_balance)
            .ok_or(ErrorCode::NumericalOverflow)?;

        msg!(
            "Notice: Amount has been changed to {}. This is due to account rent requirements",
            checked_amount
        );
    }

    Ok(checked_amount)
}

pub fn verify_tip_percentage(tip_percentage: u16) -> Result<()> {
    if tip_percentage > 10000 {
        return Err(error!(ErrorCode::InvalidTipPercentage));
    }

    Ok(())
}

pub fn verify_tippees(
    tippees: Option<Vec<Tippee>>,
) -> Result<(Option<[Option<Tippee>; MAX_TIPPEES]>, u8)> {
    let mut total_share: u16 = 0;
    let mut tippee_count: u8 = 0;

    let tippees_array: Option<[Option<Tippee>; MAX_TIPPEES]> = match tippees {
        Some(vec) => match vec.len() {
            0 => None,
            1..=MAX_TIPPEES => {
                let mut arr: [Option<Tippee>; MAX_TIPPEES] = [None; MAX_TIPPEES];

                for (i, tippee) in vec.iter().enumerate() {
                    arr[i] = Some(*tippee);
                    total_share += tippee.share;
                    tippee_count += 1;
                }

                if total_share != 10000 {
                    return Err(error!(ErrorCode::InvalidTotalTippeeShare));
                }

                Some(arr)
            }
            _ => return Err(error!(ErrorCode::TooManyTippees)),
        },
        None => None,
    };

    Ok((tippees_array, tippee_count))
}

pub fn check_receiver_jar_init(receiver_jar_state: &AccountInfo) -> bool {
    return receiver_jar_state.data_len() > 0
        && receiver_jar_state.data.borrow()[8] == AccountType::TipJarStateV1 as u8;
}
