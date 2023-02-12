use concordium_std::*;

use super::contract_types::*;
use super::state::State;

/// Takes a list of queries. Each query is an owner address and some address to
/// check as an operator of the owner address.
///
/// It rejects if:
/// - It fails to parse the parameter.
#[receive(
    contract = "CIS2-4907-NFT",
    name = "userExpires",
    parameter = "ContractTokenId",
    return_value = "Option<Timestamp>",
    error = "ContractError"
)]
fn contract_user_expires<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> Result<Option<Timestamp>, ContractError> {
    let token_id: ContractTokenId = ctx.parameter_cursor().get()?;
    let res = host
        .state()
        .token_users
        .get(&token_id).map(|u| u.1);

    Ok(res)
}
