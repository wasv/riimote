#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/xwiimote.rs"));

pub mod riimote {
    use super::*;
    
    use std::ffi::CStr;
    use std::str;
    use std::mem;
    use std::io;


    #[derive(Copy,Clone,Debug)]
    pub struct Wiimote {
        dev: xwii_iface,
    }

    impl Wiimote {
        pub fn new() -> Option<Self> {
            let mut path: Option<String> = None;
            unsafe {
                let mon: *mut xwii_monitor = xwii_monitor_new(false, false);
                assert!(!mon.is_null() ,"xwii_monitor_new failed.");
                let ent = xwii_monitor_poll(mon);
                if !ent.is_null() {
                    let c_str: &CStr = CStr::from_ptr(ent);
                    let str_slice: &str = c_str.to_str().unwrap();
                    path = Some(str_slice.to_owned());
                }
                xwii_monitor_unref(mon);
            }
            match path {
                Some(path) => Some(Wiimote::new_from_path(path)),
                None => None,
            }
        }
        pub fn new_from_path(syspath: String) -> Self {
            let mut dev: xwii_iface = xwii_iface::default();
            unsafe {
                let mut dev: *mut xwii_iface = &mut dev as *mut xwii_iface;
                let dev: *mut *mut xwii_iface = &mut dev as *mut *mut xwii_iface;
                let ret = xwii_iface_new(dev, syspath.as_ptr() as *const i8);
                assert!(ret == 0,"xwii_iface_new Error: {}",ret);

                let ret = xwii_iface_open(*dev, xwii_iface_available(*dev) |
                                          xwii_iface_type_XWII_IFACE_WRITABLE);
                assert!(ret == 0,"xwii_iface_open Error: {}",ret);

                let ret = xwii_iface_watch(*dev, true);
                assert!(ret == 0,"xwii_iface_watch Error: {}",ret);
                return Wiimote{dev:**dev}
            }
        }

        pub fn get_event(mut self) -> Option<xwii_event> {
            let mut event: xwii_event = xwii_event::default();
            unsafe {
                let ret = xwii_iface_dispatch(&mut self.dev, &mut event, mem::size_of::<xwii_event>());
                if ret == -11 {
                    return None
                }
                assert!(ret == 0,"xwii_iface_dispatch Error: {}",io::Error::from_raw_os_error(ret))
            }
            Some(event)
        }
    }
}
