# solana-program
Solana smart contract written in rust that increments, decrements, or sets a counter based on client instructions


# Getting Started
The following dependencies are required to build and run this example, depending on your OS, they may already be installed:

Install node (v14 recommended)  
Install npm  
Install Rust v1.56.1 or later from https://rustup.rs/  
Install Solana v1.8.14 or later from https://docs.solana.com/cli/install-solana-cli-tools  

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

##Expected output
```
> solana-smart-contract@0.0.1 start
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 1122441720, 'solana-core': '1.10.8' }
Using account 7H11wsJQNyKymphQ1NzSpcvE2MPAZd2Q56w2NbKywCxx containing 499999999.18092644 SOL to pay for fees
Using program CMAzYtb3iNuwDatfV5BXc5Y7MCVPn7aoV3UKQwL2hzP6
Saying hello to HuAuKTuGWCdjmfiLAQZzQLW3AuJMYLcrmkBvwBRaGxAr
HuAuKTuGWCdjmfiLAQZzQLW3AuJMYLcrmkBvwBRaGxAr is saying  256 times for my Father!!!!
Success
```
