use concordium_cis2::*;
use concordium_std::*;

use super::contract_types::*;
use super::error::CustomContractError;

#[derive(Debug, Serialize, Clone, SchemaType)]
pub struct TokenMetadata {
    /// The URL following the specification RFC1738.
    #[concordium(size_length = 2)]
    pub url: String,
    /// A optional hash of the content.
    #[concordium(size_length = 2)]
    pub hash: String,
}

impl TokenMetadata {
    fn get_hash_bytes(&self) -> Option<[u8; 32]> {
        match hex::decode(&self.hash) {
            Ok(v) => {
                let slice = v.as_slice();
                match convert::TryInto::try_into(slice) {
                    Ok(array) => Option::Some(array),
                    Err(_) => Option::None,
                }
            }
            Err(_) => Option::None,
        }
    }

    pub(crate) fn to_metadata_url(&self) -> MetadataUrl {
        MetadataUrl {
            url: self.url.to_string(),
            hash: self.get_hash_bytes(),
        }
    }
}

/// The state for each address.
#[derive(Serial, DeserialWithState, Deletable)]
#[concordium(state_parameter = "S")]
pub(crate) struct AddressState<S> {
    /// The tokens owned by this address.
    pub(crate) owned_tokens: StateSet<ContractTokenId, S>,
    /// The address which are currently enabled as operators for this address.
    pub(crate) operators: StateSet<Address, S>,
}

impl<S: HasStateApi> AddressState<S> {
    fn empty(state_builder: &mut StateBuilder<S>) -> Self {
        AddressState {
            owned_tokens: state_builder.new_set(),
            operators: state_builder.new_set(),
        }
    }
}

/// The contract state.
// Note: The specification does not specify how to structure the contract state
// and this could be structured in a more space efficient way depending on the use case.
#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
pub(crate) struct State<S> {
    /// The state for each address.
    pub(crate) state: StateMap<Address, AddressState<S>, S>,
    /// All of the token IDs
    pub(crate) all_tokens: StateSet<ContractTokenId, S>,
    pub(crate) metadata: StateMap<ContractTokenId, TokenMetadata, S>,
    pub(crate) token_users: StateMap<ContractTokenId, (Address, Timestamp), S>,

    /// Map with contract addresses providing implementations of additional
    /// standards.
    pub(crate) implementors: StateMap<StandardIdentifierOwned, Vec<ContractAddress>, S>,
}

// Functions for creating, updating and querying the contract state.
impl<S: HasStateApi> State<S> {
    /// Creates a new state with no tokens.
    pub(crate) fn empty(state_builder: &mut StateBuilder<S>) -> Self {
        State {
            state: state_builder.new_map(),
            all_tokens: state_builder.new_set(),
            implementors: state_builder.new_map(),
            metadata: state_builder.new_map(),
            token_users: state_builder.new_map(),
        }
    }

    /// Mint a new token with a given address as the owner
    pub(crate) fn mint(
        &mut self,
        token: ContractTokenId,
        metadata: &TokenMetadata,
        owner: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> Result<(), ContractError> {
        ensure!(
            self.all_tokens.insert(token),
            CustomContractError::TokenIdAlreadyExists.into()
        );
        self.metadata.insert(token, metadata.clone());

        let mut owner_state = self
            .state
            .entry(*owner)
            .or_insert_with(|| AddressState::empty(state_builder));
        owner_state.owned_tokens.insert(token);
        Ok(())
    }

    /// Check that the token ID currently exists in this contract.
    #[inline(always)]
    pub(crate) fn contains_token(&self, token_id: &ContractTokenId) -> bool {
        self.all_tokens.contains(token_id)
    }

    /// Get the current balance of a given token ID for a given address.
    /// Results in an error if the token ID does not exist in the state.
    /// Since this contract only contains NFTs, the balance will always be
    /// either 1 or 0.
    pub(crate) fn balance(
        &self,
        token_id: &ContractTokenId,
        address: &Address,
    ) -> Result<ContractTokenAmount, ContractError> {
        ensure!(self.contains_token(token_id), ContractError::InvalidTokenId);
        let balance = self
            .state
            .get(address)
            .map(|address_state| {
                if address_state.owned_tokens.contains(token_id) {
                    1
                } else {
                    0
                }
            })
            .unwrap_or(0);
        Ok(balance.into())
    }

    /// Check if a given address is an operator of a given owner address.
    pub(crate) fn is_operator(&self, address: &Address, owner: &Address) -> bool {
        self.state
            .get(owner)
            .map(|address_state| address_state.operators.contains(address))
            .unwrap_or(false)
    }

    /// Update the state with a transfer of some token.
    /// Results in an error if the token ID does not exist in the state or if
    /// the from address have insufficient tokens to do the transfer.
    pub(crate) fn transfer(
        &mut self,
        token_id: &ContractTokenId,
        amount: ContractTokenAmount,
        from: &Address,
        to: &Address,
        state_builder: &mut StateBuilder<S>,
    ) -> Result<(), ContractError> {
        ensure!(self.contains_token(token_id), ContractError::InvalidTokenId);
        // A zero transfer does not modify the state.
        if amount == 0.into() {
            return Ok(());
        }
        // Since this contract only contains NFTs, no one will have an amount greater
        // than 1. And since the amount cannot be the zero at this point, the
        // address must have insufficient funds for any amount other than 1.
        ensure_eq!(amount, 1.into(), ContractError::InsufficientFunds);

        {
            let mut from_address_state = self
                .state
                .get_mut(from)
                .ok_or(ContractError::InsufficientFunds)?;
            // Find and remove the token from the owner, if nothing is removed, we know the
            // address did not own the token..
            let from_had_the_token = from_address_state.owned_tokens.remove(token_id);
            ensure!(from_had_the_token, ContractError::InsufficientFunds);
        }

        // Add the token to the new owner.
        let mut to_address_state = self
            .state
            .entry(*to)
            .or_insert_with(|| AddressState::empty(state_builder));
        to_address_state.owned_tokens.insert(*token_id);
        Ok(())
    }

    /// Update the state adding a new operator for a given address.
    /// Succeeds even if the `operator` is already an operator for the
    /// `address`.
    pub(crate) fn add_operator(
        &mut self,
        owner: &Address,
        operator: &Address,
        state_builder: &mut StateBuilder<S>,
    ) {
        let mut owner_state = self
            .state
            .entry(*owner)
            .or_insert_with(|| AddressState::empty(state_builder));
        owner_state.operators.insert(*operator);
    }

    /// Update the state removing an operator for a given address.
    /// Succeeds even if the `operator` is _not_ an operator for the `address`.
    pub(crate) fn remove_operator(&mut self, owner: &Address, operator: &Address) {
        self.state.entry(*owner).and_modify(|address_state| {
            address_state.operators.remove(operator);
        });
    }

    /// Check if state contains any implementors for a given standard.
    pub(crate) fn have_implementors(&self, std_id: &StandardIdentifierOwned) -> SupportResult {
        if let Some(addresses) = self.implementors.get(std_id) {
            SupportResult::SupportBy(addresses.to_vec())
        } else {
            SupportResult::NoSupport
        }
    }

    /// Set implementors for a given standard.
    pub(crate) fn set_implementors(
        &mut self,
        std_id: StandardIdentifierOwned,
        implementors: Vec<ContractAddress>,
    ) {
        self.implementors.insert(std_id, implementors);
    }

    pub(crate) fn set_user(
        &mut self,
        token_id: &ContractTokenId,
        user: Address,
        expires: Timestamp,
    ) {
        self.token_users.insert(*token_id, (user, expires));
    }

    pub(crate) fn reset_user(&mut self, token_id: &ContractTokenId) {
        self.token_users.remove(token_id)
    }
}
