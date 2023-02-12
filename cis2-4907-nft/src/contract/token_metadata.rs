use concordium_cis2::*;
use concordium_std::*;

use super::contract_types::*;
use super::state::State;

/// Parameter type for the CIS-2 function `tokenMetadata` specialized to the
/// subset of TokenIDs used by this contract.
type ContractTokenMetadataQueryParams = TokenMetadataQueryParams<ContractTokenId>;

/// Get the token metadata URLs and checksums given a list of token IDs.
///
/// It rejects if:
/// - It fails to parse the parameter.
/// - Any of the queried `token_id` does not exist.
#[receive(
    contract = "CIS2-4907-NFT",
    name = "tokenMetadata",
    parameter = "ContractTokenMetadataQueryParams",
    return_value = "TokenMetadataQueryResponse"
)]
fn contract_token_metadata<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> Result<TokenMetadataQueryResponse, ContractError> {
    // Parse the parameter.
    let params: ContractTokenMetadataQueryParams = ctx.parameter_cursor().get()?;
    // Build the response.
    let mut response = Vec::with_capacity(params.queries.len());
    for token_id in params.queries {
        // Check the token exists.
        ensure!(
            host.state().contains_token(&token_id),
            ContractError::InvalidTokenId
        );

        let metadata_url: MetadataUrl = host
            .state()
            .metadata
            .get(&token_id)
            .map(|metadata| metadata.to_metadata_url())
            .ok_or(ContractError::InvalidTokenId)?;

        response.push(metadata_url);
    }

    Ok(TokenMetadataQueryResponse::from(response))
}
