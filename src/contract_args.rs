use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct EscrowContractArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Add Eth to escrow for a recipient, get balance for recipient or release funds once release condition is met
    EscrowContract(EscrowContractCommand),
}

#[derive(Debug, Args)]
pub struct EscrowContractCommand {
    #[clap(subcommand)]
    pub command: EscrowContractSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum EscrowContractSubcommand {
    /// Deposit Eth to the specified wallet address
    Deposit(WalletAddress),

    /// Release the fund for the specified wallet address
    RequestRelease(WalletAddress),

    /// Get the remaing funds in escrow for the specified wallet address
    GetBalance(WalletAddress),
}

#[derive(Debug, Args)]
pub struct WalletAddress {
    /// Wallet Address of the recipient
    pub address: String,
}
