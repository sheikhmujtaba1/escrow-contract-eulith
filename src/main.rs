mod contract_args;

use clap::Parser;
use contract_args::EscrowContractArgs;
fn main() {
    let args = EscrowContractArgs::parse();
    println!("{:?}", args);
}
