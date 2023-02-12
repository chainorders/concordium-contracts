use concordium_cis2::{Cis2Error, TokenAmountU8, TokenIdU32};

use super::error::CustomContractError;

/// Contract token ID type.
/// To save bytes we use a token ID type limited to a `u32`.
pub type ContractTokenId = TokenIdU32;

/// Contract token amount.
/// Since the tokens are non-fungible the total supply of any token will be at
/// most 1 and it is fine to use a small type for representing token amounts.
pub type ContractTokenAmount = TokenAmountU8;

pub type ContractError = Cis2Error<CustomContractError>;
