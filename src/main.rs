extern crate riimote_sys;

use riimote_sys::riimote::Wiimote;
use std::process;

fn main() {
    if let Some(wmote) = Wiimote::new() {
        println!("WiiMote found!");
        let mut event = wmote.get_event();
        while event.is_none() {
            event = wmote.get_event();
        }
        let event = event.unwrap();
    } else {
        println!("No WiiMote found!");
        process::exit(-1);
    };
}
