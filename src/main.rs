extern crate riimote_sys;

use riimote_sys::riimote::Wiimote;

fn main() {
    if let Some(wmote) = Wiimote::new() {
        println!("WiiMote found!");
    } else {
        println!("No WiiMote found!");
    };
}
