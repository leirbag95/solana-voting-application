# solana-voting-application
A simple voting or governance application powered by solana

# MacOS & Linux

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

'''bash
solana airdrop 2 <YOURPUBKEY>
#This command only in you don't have lamports on the account
#By the way, you have to change the Program ID from Anchor.toml and lib.rs to make it works!
#Also, you have to turn off the solana validator when testing in next step
'''
'''bash
anchor test
'''

