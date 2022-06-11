pub mod constant;
pub mod error;
pub mod processor;
pub mod state;
pub mod util;

use crate::{processor::context::*, state::*};
use anchor_lang::prelude::*;

declare_id!("78vov3JfntD46gxEAFhtu2tTfonpEfkbzMSDe9aNTHbP");

#[program]
pub mod donos {
    use super::*;

    pub fn initialize_tip_jar(
        ctx: Context<InitializeJar>,
        tip_percentage: u16,
        tippees: Option<Vec<Tippee>>,
    ) -> Result<()> {
        processor::initialize_tip_jar(ctx, tip_percentage, tippees)
    }
}
