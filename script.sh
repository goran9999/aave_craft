#!/bin/bash

npm i

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup default 1.70.0

sh -c "$(curl -sSfL https://release.solana.com/v1.14.18/install)"

cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

avm install 0.28.0

avm use 0.28.0

anchor test