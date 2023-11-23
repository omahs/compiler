#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden::account;
use miden::asset::Asset;
use miden::note::Recipient;
use miden::note::Tag;
use miden::tx;

pub struct MyWallet;

impl MyWallet {
    #[no_mangle]
    pub fn receive_asset(&self, asset: Asset) {
        account::add_asset(asset);
    }

    #[no_mangle]
    pub fn send_asset(&self, asset: Asset, recipient: Recipient) {
        let asset = account::remove_asset(asset);
        let tag = Tag::new(4);
        tx::create_note(asset, tag, recipient);
    }
}
