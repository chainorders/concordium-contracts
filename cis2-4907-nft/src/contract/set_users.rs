use concordium_std::*;

use super::{
    contract_types::{ContractError, ContractTokenAmount, ContractTokenId},
    state::State,
};

#[derive(Serial, Deserial, SchemaType)]
struct SetUserParams {
    token_id: ContractTokenId,
    expires: Timestamp,
    user: Address,
}

#[derive(Serial, Deserial, SchemaType)]
struct SetUsersParams(Vec<SetUserParams>);

#[receive(
    contract = "CIS2-4907-NFT",
    name = "setUsers",
    parameter = "SetUsersParams",
    mutable,
    error = "ContractError"
)]
fn contract_set_users<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), ContractError> {
    let params: SetUsersParams = ctx.parameter_cursor().get()?;

    for SetUserParams {
        token_id,
        expires,
        user,
    } in params.0.iter()
    {
        ensure!(
            host.state().contains_token(token_id),
            ContractError::InvalidTokenId
        );
        match host.state().balance(token_id, &ctx.sender()) {
            Err(e) => bail!(e),
            Ok(balance) => ensure!(
                balance.cmp(&ContractTokenAmount::from(1)).is_ge(),
                ContractError::InsufficientFunds
            ),
        }

        host.state_mut().set_user(token_id, *user, *expires);
    }

    Ok(())
}
