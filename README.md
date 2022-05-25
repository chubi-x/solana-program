# solana-program
Solana smart contract written in rust that increments, decrements, or sets a counter based on client instructions


# Getting Started
The following dependencies are required to build and run this example, depending on your OS, they may already be installed:

Install node (v14 recommended)
Install npm
Install Rust v1.56.1 or later from https://rustup.rs/
Install Solana v1.8.14 or later from https://docs.solana.com/cli/install-solana-cli-tools
If this is your first time using Rust, these Installation Notes might be helpful.

## Configure CLI
If you're on Windows, it is recommended to use WSL to run these commands

## Set CLI config url to localhost cluster
` solana config set --url localhost `
## Create CLI Keypair
If this is your first time using the Solana CLI, you will need to generate a new keypair:

`solana-keygen new` 
## Start local Solana cluster
This example connects to a local Solana cluster by default.

Start a local Solana cluster:

` solana-test-validator` 
Note: You may need to do some system tuning (and restart your computer) to get the validator to run

## Listen to transaction logs:

` solana logs` 
## Install npm dependencies
` npm install `
## Build the on-chain program
` npm run build:program-rust` 
## Deploy the on-chain program
` solana program deploy dist/program/helloworld.so` 
## Run the JavaScript client
`npm run start `
