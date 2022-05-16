# solana-voting-application
A simple voting or governance application powered by solana

# MacOS & Linux
Welcome to our Voting System App made at Solana Coding Bootcamp ^_^ . 
You would probably need to install Solana, Anchor and Rust to make it all run.
You can install that dependencies following this tutorial : https://project-serum.github.io/anchor/getting-started/installation.html

When you are set up, you can run the App doing the following commands:
```bash
solana config set --url localhost
```
```bash
cd solana-voting-application
npm i
yarn add ts-mocha
```
```bash
solana-test-validator
# Open in other console de test validator!
```

```bash
anchor build
```
```bash
anchor deploy
```

```bash
solana airdrop 2 <YOURPUBKEY>
#This command only in you don't have lamports on the account
#By the way, you have to change the Program ID from Anchor.toml and lib.rs to make it works!
#Also, you have to turn off the solana validator when testing in next step
```
```bash
anchor test
```

