use concordium_std::*;

use super::contract_types::*;
use super::state::{State, TokenMetadata};

#[derive(Serialize, SchemaType)]
struct ViewAddressState {
    owned_tokens: Vec<ContractTokenId>,
    operators: Vec<Address>,
}

#[derive(Serialize, SchemaType)]
struct TokenUser {
    user: Address,
    expires: Timestamp,
}

#[derive(Serialize, SchemaType)]
struct ViewState {
    state: Vec<(Address, ViewAddressState)>,
    all_tokens: Vec<ContractTokenId>,
    metadata: Vec<(ContractTokenId, TokenMetadata)>,
    users: Vec<(ContractTokenId, TokenUser)>,
}

/// View function that returns the entire contents of the state. Meant for
/// testing.
#[receive(contract = "CIS2-4907-NFT", name = "view", return_value = "ViewState")]
fn contract_view<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ViewState> {
    let state = host.state();

    let mut inner_state = Vec::new();
    for (k, a_state) in state.state.iter() {
        let owned_tokens = a_state.owned_tokens.iter().map(|x| *x).collect();
        let operators = a_state.operators.iter().map(|x| *x).collect();
        inner_state.push((
            *k,
            ViewAddressState {
                owned_tokens,
                operators,
            },
        ));
    }
    let all_tokens = state.all_tokens.iter().map(|x| *x).collect();
    let all_tokens_metadata = state
        .metadata
        .iter()
        .map(|(k, v)| (*k, v.clone()))
        .collect();

    let users = state
        .token_users
        .iter()
        .map(|(k, v)| {
            (
                *k,
                TokenUser {
                    user: v.0,
                    expires: v.1,
                },
            )
        })
        .collect();

    Ok(ViewState {
        state: inner_state,
        all_tokens,
        metadata: all_tokens_metadata,
        users,
    })
}
