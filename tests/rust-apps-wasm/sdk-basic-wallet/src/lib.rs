#![no_std]

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static A: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

extern crate alloc;

use miden::Asset;
use miden::Recipient;
use miden::Tag;

// The code written by user:
// ------------------------------------------------------------------------------------------------

//#[miden::account]
pub struct MyWallet;

//#[miden::account]
#[cfg(not(feature = "build_notes"))]
impl MyWallet {
    pub fn receive_asset(&self, asset: Asset) {
        self.add_asset(asset);
    }

    pub fn send_asset(&self, asset: Asset, tag: Tag, recipient: Recipient) {
        let asset = self.remove_asset(asset);
        miden::sat::tx::create_note(asset, tag, recipient);
    }
}

// Macros-generated code
// ------------------------------------------------------------------------------------------------

impl MyWallet {
    pub fn add_asset(&self, asset: Asset) {
        miden::account::add_asset(asset);
    }

    pub fn remove_asset(&self, asset: Asset) -> Asset {
        miden::account::remove_asset(asset)
    }
}

// To be compiled with `cargo build --features build_notes` to build a note's side of the account code.
#[cfg(feature = "build_notes")]
extern "C" {
    #[link_name = "my_wallet::receive_asset"]
    fn receive_asset_extern(
        call_conv: miden::Felt,
        sv1: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
        sv5: miden::Felt,
        sv6: miden::Felt,
        sv7: miden::Felt,
        sv8: miden::Felt,
        sv9: miden::Felt,
        sv10: miden::Felt,
        sv11: miden::Felt,
        sv12: miden::Felt,
        sv13: miden::Felt,
        sv14: miden::Felt,
        sv15: miden::Felt,
    );

    #[link_name = "my_wallet::send_asset"]
    fn send_asset_extern(
        call_conv: miden::Felt,
        sv1: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
        sv5: miden::Felt,
        sv6: miden::Felt,
        sv7: miden::Felt,
        sv8: miden::Felt,
        sv9: miden::Felt,
        sv10: miden::Felt,
        sv11: miden::Felt,
        sv12: miden::Felt,
        sv13: miden::Felt,
        sv14: miden::Felt,
        sv15: miden::Felt,
    );
}

// Substitution for the user's code for the note's code
#[cfg(feature = "build_notes")]
impl MyWallet {
    pub fn receive_asset(&self, asset: miden::Asset) {
        let felts = {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Args {
                asset: miden::Asset,
            }
            let args_bytes = postcard::to_allocvec(&Args { asset }).unwrap();
            miden::bytes_to_felts(args_bytes)
        };
        if felts.len() <= 15 {
            let call_conv = miden::call_conv::FuncArgPassingConv {
                medium: miden::call_conv::FuncArgPassingMedium::Stack,
                felt_count: felts.len() as u32,
            };
            unsafe {
                receive_asset_extern(
                    call_conv.to_felt(),
                    felts[0],
                    felts[1],
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                );
            }
        } else {
            todo!("use advice provider");
        }
    }

    pub fn send_asset(&self, asset: miden::Asset, tag: miden::Tag, recipient: miden::Recipient) {
        let felts = {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct Args {
                asset: miden::Asset,
                tag: miden::Tag,
                recipient: miden::Recipient,
            }
            let args_bytes = postcard::to_allocvec(&Args {
                asset,
                tag,
                recipient,
            })
            .unwrap();
            miden::bytes_to_felts(args_bytes)
        };
        if felts.len() <= 15 {
            let call_conv = miden::call_conv::FuncArgPassingConv {
                medium: miden::call_conv::FuncArgPassingMedium::Stack,
                felt_count: felts.len() as u32,
            };
            unsafe {
                /*
                The actual macro-generated code will actually look like this:
                if felts.len() == 1 {
                    send_asset_extern(call_conv, felt[0], 0 ... );
                } else if felts.len() == 2 {
                    send_asset_extern(call_conv, felt[0], felt[1], 0 ...);
                } else if felts.len() == 3 {
                    send_asset_extern(call_conv, felt[0], felt[1], felt[2], 0 ...);
                } ... etc.

                 */
                send_asset_extern(
                    call_conv.to_felt(),
                    felts[0],
                    felts[1],
                    felts[2],
                    felts[3],
                    felts[4],
                    felts[5],
                    felts[6],
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                    0.into(),
                );
            }
        } else {
            todo!("use advice provider");
        }
    }
}

#[cfg(not(feature = "build_notes"))]
impl MyWallet {
    #[export_name = "my_wallet::receive_asset"]
    pub fn receive_asset_account_export(
        &self,
        call_conv: miden::Felt,
        sv1: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
        sv5: miden::Felt,
        sv6: miden::Felt,
        sv7: miden::Felt,
        sv8: miden::Felt,
        sv9: miden::Felt,
        sv10: miden::Felt,
        sv11: miden::Felt,
        sv12: miden::Felt,
        sv13: miden::Felt,
        sv14: miden::Felt,
        sv15: miden::Felt,
    ) {
        let call_conv = miden::call_conv::FuncArgPassingConv::from_felt(call_conv);
        if call_conv.medium == miden::call_conv::FuncArgPassingMedium::Stack {
            /*

            The actual macro-generated code will actually look like this:
            let len = call_conv.felt_count;
            if len == 1 {
                [sv1]
            } else if len == 2 {
                [sv1, sv2]
            }  ... etc.

             */
            let felts = [sv1, sv2, sv3];
            let bytes = miden::felts_to_bytes(felts.to_vec());

            #[derive(serde::Serialize, serde::Deserialize)]
            struct Args {
                asset: miden::Asset,
            }
            let args: Args = postcard::from_bytes(&bytes).unwrap();
            self.receive_asset(args.asset);
        } else {
            todo!("use advice provider");
        };
    }

    #[export_name = "my_wallet::send_asset"]
    pub fn send_asset_account_export(
        &self,
        call_conv: miden::Felt,
        sv1: miden::Felt,
        sv2: miden::Felt,
        sv3: miden::Felt,
        sv4: miden::Felt,
        sv5: miden::Felt,
        sv6: miden::Felt,
        sv7: miden::Felt,
        sv8: miden::Felt,
        sv9: miden::Felt,
        sv10: miden::Felt,
        sv11: miden::Felt,
        sv12: miden::Felt,
        sv13: miden::Felt,
        sv14: miden::Felt,
        sv15: miden::Felt,
    ) {
        let call_conv = miden::call_conv::FuncArgPassingConv::from_felt(call_conv);
        if call_conv.medium == miden::call_conv::FuncArgPassingMedium::Stack {
            let felts = [sv1, sv2, sv3, sv4, sv5, sv6, sv7];
            let bytes = miden::felts_to_bytes(felts.to_vec());

            #[derive(serde::Serialize, serde::Deserialize)]
            struct Args {
                asset: miden::Asset,
                tag: miden::Tag,
                recipient: miden::Recipient,
            }
            let args: Args = postcard::from_bytes(&bytes).unwrap();
            self.send_asset(args.asset, args.tag, args.recipient);
        } else {
            todo!("use advice provider");
        };
    }
}
