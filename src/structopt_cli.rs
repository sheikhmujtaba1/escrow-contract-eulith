use structopt::StructOpt;
use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{Address, U256};

#[derive(StructOpt, Debug)]
#[structopt(name = "contract-cli")]
struct Opt {
    #[structopt(long = "contract-address", parse(try_from_str))]
    contract_address: Address,
    #[structopt(long = "hardhat-node-url", default_value = "http://localhost:8545")]
    hardhat_node_url: String,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "set")]
    Set { value: U256 },
    #[structopt(name = "get")]
    Get,
}

fn main() {
    let opt = Opt::from_args();

    // Connect to the Hardhat node
    let (_eloop, transport) = web3::transports::Http::new(&opt.hardhat_node_url).unwrap();
    let web3 = web3::Web3::new(transport);

    // Load the contract ABI and bytecode
    let abi = include_str!("contract.abi");
    let bytecode = include_bytes!("contract.bin");

    // Create a contract instance
    let contract =
        Contract::from_abi(web3.eth(), opt.contract_address, abi.as_bytes(), bytecode).unwrap();

    // Dispatch the command
    match opt.cmd {
        Command::Set { value } => {
            let options = Options::default();
            let result = contract
                .call("set", (value,), options, Default::default())
                .wait()
                .unwrap();
            println!("Transaction hash: {}", result);
        }
        Command::Get => {
            let result = contract
                .call("get", (), Options::default(), Default::default())
                .wait()
                .unwrap();
            let value: U256 = result.try_into().unwrap();
            println!("Value: {}", value);
        }
    }
}
