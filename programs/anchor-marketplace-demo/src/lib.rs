pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D41dZgASdrgZHRyBQEn4qBN1aeJKEmuAnurLw8PxMKZM");

#[program]
pub mod anchor_marketplace_demo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
