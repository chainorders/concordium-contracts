# Sample CIS2 Contracts

Concordium is a science-based proof-of-stake blockchain created for all, with in particular business applications in mind. [Read more about concordium](https://www.concordium.com/about)

This repository provides sample implementations of following ways in which a developer can interact with an on chain contract on Concordium.

- Using [Concordium Client](<(https://developer.concordium.software/en/mainnet/smart-contracts/guides/on-chain-index.html)>)

## Contents Of Repository

- **Contracts**

  - [CIS2 Auctions](./cis2-auctions/README.md)

    - Allows only a Single Auction per instance of the contract.
    - Allows auctioning of any CIS2 Token. By transferring the ownership of the token to `auction` Smart Contract Instance.
    - Participants need to also hold and then transfer Participation token to the `auction` Smart Contract Instance to be able to participate in a particular Auction.

  - [CIS2 Fractionalizer](./cis2-fractionalizer/README.md)

    - Allows the user to Transfer any Quantity of a CIS2 token to Mint other CIS2 token of Any Quantity.

  - [CIS2 Market](./cis2-market/README.md)

    - Allows user to Sell a CIS2 token for a particular specified price in CCD.

  - [CIS2 Multi](./cis2-multi/README.md)

    - Allows a user to mint any amount of a CIS2 token and perform various interactions specified by CIS2 Standard.

  - [CIS2 Nft](./cis2-nft/README.md)
    - Allows a user to mint A single CIS2 token (NFT) and perform various interactions specified by CIS2 Standard.

  - [CIS2 4907 Nft](./cis2-4907-nft/README.md)
    - [EIP-4907](https://eips.ethereum.org/EIPS/eip-4907) implementation over CIS2-NFT

- [**Concordium Client**](./concordium-client/README.md) :
  Sample commands to interact with various provided Contracts.

- [**Sample Artifacts**](./sample-artifacts/) :
  Sample JSON requests used to interact with the Smart Contracts. These should be modified according to the user and their deployed instances of smart contracts.

## Get Started

Throughout this repository [Concordium Testnet](https://testnet.ccdscan.io/) is being used to demo the functionality.

- Perquisites

  - Download and [Install Docker Compose](https://docs.docker.com/compose/install/)
  - [Install tools for Smart Contract development](https://developer.concordium.software/en/mainnet/smart-contracts/guides/setup-tools.html#setup-tools)
  - Clone this Repository
  - Create Concordium Account & Wallet
    - Download concordium testnet wallet
      - [For IOS, IPhone](https://developer.concordium.software/en/mainnet/net/installation/downloads-testnet.html#ios)
      - [For Android](https://developer.concordium.software/en/mainnet/net/installation/downloads-testnet.html#android)
    - [Create Testnet Account](https://developer.concordium.software/en/mainnet/net/guides/create-account.html)
    - [Export wallet](https://developer.concordium.software/en/mainnet/net/guides/export-import.html#export-import) and then copy the file in root named [concordium-backup.concordiumwallet](./concordium-backup.concordiumwallet)

- Build Contracts

  ```bash
  cd cis2-multi ## Or cd cis2-market
  cargo concordium build --out module.wasm --schema-out schema.bin
  ```

- Interact with Contracts
  - Using [Concordium Client](./concordium-client/README.md)
