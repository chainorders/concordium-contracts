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
    name = "userOf",
    parameter = "ContractTokenId",
    return_value = "Option<Address>",
    error = "ContractError"
)]
fn contract_user_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> Result<Option<Address>, ContractError> {
    let token_id: ContractTokenId = ctx.parameter_cursor().get()?;
    let res = match host.state().token_users.get(&token_id) {
        Some(u) => {
            if ctx.metadata().slot_time().cmp(&u.1).is_lt() {
                Some(u.0)
            } else {
                None
            }
        }
        None => None,
    };

    Ok(res)
}
