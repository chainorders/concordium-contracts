use concordium_std::*;

use super::state::State;

/// Initialize contract instance with no token types initially.
#[init(contract = "CIS2-4907-NFT")]
fn contract_init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    // Construct the initial contract state.
    Ok(State::empty(state_builder))
}
