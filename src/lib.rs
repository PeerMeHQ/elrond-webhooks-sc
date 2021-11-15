#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait Webhooks {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn deposit(&self) -> SCResult<()> {
        Ok(())
    }
}
