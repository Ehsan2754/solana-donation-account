# solana-donation-account
## Main features
1) Makes donations in lamports.
2) Withdraws donations to the wallet of the creator of the program.
(Only the creator of the program can perform this action.)
3) Stores the addresses of all users who have made donations (in a PDA).
4) Stores the donation amounts of each user.
5) unit tests.
6) Deploy the contract to devnet.
7) Write a minimal client to interact with the contract in devnet.
The program  in Rust, but it is recommended to use the Anchor framework.
It is recommended to write tests/client in typescript.

## Check list
-Install the sollet extension as a wallet. Load generated private key information into sollet.
-The program has the function of making any amount of lamports as a donation.
-The program has a function to withdraw all donations to the account of the owner of the program, which can only be called by the owner.
-These functions need to be tested.
-On the date of the account, information about everyone who has ever made donations should be stored.
-It should be possible to access the contract deployed in devnet through a typescript client (example https://project-serum.github.io/anchor/tutorials/tutorial-0.html#generating-a-client) so that you can:
- Make a donation
-Withdraw donations to the owner's account,
-Display a list of all users who have ever donated,
-Display the sum of all donations made by a certain user.
-The project is published on github/gitlab/bitbucket.
-The contract is deployed in devnet.

## Extra
-https://doc.rust-lang.ru/book/ - the main documentation on rust, recommended for reading chapters 1-8. 10, 13 and 17 can be viewed additionally.

-https://docs.solana.com/developing/programming-model/overview - of.doc on Solana, it is recommended to read the entire "Developing" section.

-https://solanacookbook.com/#contributing - in addition to the official documents, we recommend reading it (more "live" theory and code snippets).

-https://youtu.be/sl8zY6bturs - an example of the implementation of the "subtask" of the test task; we offer to look, get inspired (git clone), translate to Anchor and implement the missing functionality.

-https://project-serum.github.io/anchor/tutorials/tutorial-0.html and
https://book.anchor-lang.com/ - basic materials on Anchor.


