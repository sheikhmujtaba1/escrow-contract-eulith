use web3::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct Escrow {
    payer: Address,
    recipient: Address,
    amount: U256,
    release_time: U256,
    released: bool,
}

impl Escrow {
    // Constructor function to initialize the contract
    pub fn new(payer: Address, recipient: Address, amount: U256, release_time: U256) -> Self {
        Self {
            payer,
            recipient,
            amount,
            release_time,
            released: false,
        }
    }

    // Function to set the recipient of the escrowed funds
    pub fn set_recipient(&mut self, recipient: Address) {
        self.recipient = recipient;
    }

    // Function to set the release time for the escrowed funds
    pub fn set_release_time(&mut self, release_time: U256) {
        self.release_time = release_time;
    }

    // Function to request the release of the escrowed funds
    pub fn release(&mut self) -> Result<(), &'static str> {
        if self.released {
            return Err("Funds have already been released.");
        }
        if self.release_time > U256::from(web3::time::now().secs()) {
            return Err("Release time has not been reached.");
        }
        self.released = true;
        Ok(())
    }

    // Function to reset the contract state
    pub fn reset(&mut self) {
        self.released = false;
        self.amount = U256::zero();
        self.release_time = U256::zero();
        self.recipient = Address::zero();
    }
}