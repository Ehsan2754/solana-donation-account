# Solana Donation Account


## Description
This project is a Solana based program in Rust to accept donations in the form of native currency (in lamports).

## Features 
* Program accepts donations in lamports.
* Program withdraws donations to the wallet of the creator of the program.(Only the creator of the program can perform this action.)
* Program stores the addresses of all users who have made donations in PDA.
* Store the donation amounts of each user.
* Unit tests.
* Deploy the contract to devnet.
* A minimal client to interact with the contract in devnet.

## Code

#### **Pre-requesties**


1. Rust
``` 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add rustfmt
```
2. Solana
``` 
sh -c "$(curl -sSfL https://release.solana.com/v1.9.1/install)" 
```
3. Nodejs v>11.0.0 and npm
4. Yarn
``` 
npm install -g yarn
```
5. Anchor 
```
 npm i -g @project-serum/anchor-cli
```
> You may run ```doctor.sh``` to check if the all dependencies are installed or install all dependencies on debian based linux systems.

### **Usage**

1. Install package dependencies ```yarn install```
2. Create a new key ```solana-keygen new```
2. Set your deployment server ```solana config set -u <server>```
3. Build the program ```anchor build```
4. Test the program ```anchor test```


### **Static analyzers (Lint)** and CI/CD
* The code is evaluated using the [workflow](https://github.com/marketplace/actions/solana-anchor-test) for running to developed tests on the program after each commit to main branch. 
* The continuce Integration is evaluated using the [workflow](https://github.com/mrgnlabs/anchor-build-action) for the program after each commit to main branch. 





### ***Technology Stack***
![Ubuntu](https://img.shields.io/badge/Ubuntu-E95420?style=for-the-badge&logo=ubuntu&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-FFD43B?style=for-the-badge&logo=rust&logoColor=red)
![YARN](https://img.shields.io/badge/YARN-27338e?style=for-the-badge&logo=yarn&logoColor=white)
![Anchor](	https://img.shields.io/badge/Anchor-000000?style=for-the-badge&logo=anchor&logoColor=black)
![Solana](https://img.shields.io/badge/Solana-316192?style=for-the-badge&logo=Solana&logoColor=blue)
![Nodejs](https://img.shields.io/badge/NodeJS-CC0000.svg?&style=for-the-badge&logo=nodejs&logoColor=green)
![npm](https://img.shields.io/badge/NPM-FF6C37?style=for-the-badge&logo=npm&logoColor=green)
![Actions](https://img.shields.io/badge/Actions-FF6C37?style=for-the-badge&logo=github&logoColor=black)



### Contributors

[![Ehsans's GitHub stats](https://github-readme-stats.vercel.app/api?username=ehsan2754)](https://github.com/ehsan2754/github-readme-stats)


Feel free to contact me for contribution. The project is open source and I would like to invite anybody interested in feild of unmanned control to contribute.



![Telegram](	https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)

[Ehsan Shaghaei](https://t.me/ethanshagaei)



### License

[MIT](LICENSE) 
