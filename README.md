I want to express my sincere gratitude for the opportunity to interview with your company. I am very impressed by the innovative work that your team is doing, and I am excited at the possibility of joining such a talented group of individuals.

The interview process was well-organized, professional and personalized, and I appreciate the time and effort that the team took to speak with me. I am confident that my skills, experience and learning mindset align well with the needs of the role and the values of the company. I do not care about money and I just want to learn.

Thank you again for considering me for this opportunity. I am eager to see what the future holds and hope to be able to contribute to the success of Eulith in the future.

Explanation of my approach and solution:

I initially coded my contract in rust, however after communicating with Lucas it was agreed that the best approach would be to use solidity. I implemented and tested the contract on remix.etherium.org and it seemed to be working fine. File should be named EscrowContract.sol within the contracts directory. Currently there is no error handling using the result type but my initial intent was to implement it.

For the CLI part; I have been able to write the CLI commands and subcommands to interact with the contract. You can test those out by running the following command " cargo run escrow-contract -h" or after selecting the subcommand you want to run i.e. "cargo run escrow-contract request-release -h". This should give you the description of the subcommands. The code for this is available in the contract_args.rs. I decided to use CLAP to parse CLI arguments due to the ease of handling complex arguments structure.

To deploy the contract on the local chain I settled on using a hardhat node.

I have also implemented another file using structopt called structopt_cli, however I have not had the time to test. However it should be a good testament of the approach that I expected to take. The file currently only has placeholder such as set or get, but these are intended to be replaced with the contract methods.

Please feel free to reach out to muhammad.mujtaba@centre.edu for any questions.
