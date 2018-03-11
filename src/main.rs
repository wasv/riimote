extern crate riimote_sys;

use riimote_sys::riimote::Wiimote;
use std::process;
use std::io::Write;
use std::io::stdout;

fn main() {
    if let Some(wmote) = Wiimote::new() {
        println!("WiiMote found!");
        loop {
            let mut event = wmote.get_event();
            while event.is_none() {
                event = wmote.get_event();
            }
            let event = event.unwrap();
            match event.type_ {
                riimote_sys::xwii_event_types_XWII_EVENT_ACCEL => {
                    print!("x:{:4} y:{:4} z:{:4} {}\r",
                           unsafe{event.v.abs[0].x},
                           unsafe{event.v.abs[0].y},
                           unsafe{event.v.abs[0].z},
                           event.type_);
                    stdout().flush();
                },
                riimote_sys::xwii_event_types_XWII_EVENT_KEY => break,
                riimote_sys::xwii_event_types_XWII_EVENT_GONE => break,
                _ => continue,
            }
        }
    } else {
        println!("No WiiMote found!");
        process::exit(-1);
    };
}
