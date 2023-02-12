- Deploy

  ```bash
  export ACCOUNT=new
  export GRPC_IP=127.0.0.1
  export GRPC_PORT=10001

  concordium-client module deploy ./module.wasm --sender $ACCOUNT --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT
  ```

- Initialize Smart Contract

  ```bash
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract init 38e7ab9a4209a1b940b88a42ac71fc19c736bb5693eb48da5f2bead644251f7b --contract CIS2-4907-NFT --sender $ACCOUNT --energy 3000
  ```

- Mint

  ```bash
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract update 2980 --entrypoint mint --parameter-json ../sample-artifacts/cis2-4907-nft/mint.json --schema ../cis2-4907-nft/schema.bin  --sender $ACCOUNT --energy 6000
  ```

  **2095 here is the index of the contract that was initialized.**

- View Contract State

  ```
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract invoke 2980 --entrypoint view --schema ../cis2-4907-nft/schema.bin
  ```

- Set Users

  ```bash
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract update 2980 --entrypoint setUsers --parameter-json ../sample-artifacts/cis2-4907-nft/set-users.json --schema ../cis2-4907-nft/schema.bin --sender $ACCOUNT --energy 6000
  ```

- User Expires

  ```bash
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract invoke 2980 --entrypoint userExpires --parameter-json ../sample-artifacts/cis2-4907-nft/user-expires.json --schema ../cis2-4907-nft/schema.bin
  ```

- User Of

  ```bash
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract invoke 2980 --entrypoint userOf --parameter-json ../sample-artifacts/cis2-4907-nft/user-of.json --schema ../cis2-4907-nft/schema.bin
  ```

- Transfer

  ```bash
  concordium-client --grpc-ip $GRPC_IP --grpc-port $GRPC_PORT contract update 2095 --entrypoint transfer --parameter-json ../sample-artifacts/cis2-4907-nft/transfer.json --schema ../cis2-4907-nft/schema.bin --sender $ACCOUNT --energy 6000
  ```
