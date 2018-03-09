#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/xwiimote.rs"));

struct Wiimote {
    dev: xwii_iface,
}

impl Wiimote {
    fn new(syspath: *const i8) -> Self {
        let mut dev: xwii_iface = xwii_iface{_unused:[]};
        unsafe {
            let mut dev: *mut xwii_iface = &mut dev as *mut xwii_iface;
            let dev: *mut *mut xwii_iface = &mut dev as *mut *mut xwii_iface;
            let ret = xwii_iface_new(dev, syspath);
            assert!(ret == 0,"xwii_iface_new Error: {}",ret);

            let ret = xwii_iface_open(*dev, xwii_iface_available(*dev) |
            xwii_iface_type_XWII_IFACE_WRITABLE);
            assert!(ret == 0,"xwii_iface_open Error: {}",ret);

            let ret = xwii_iface_watch(*dev, true);
            assert!(ret == 0,"xwii_iface_watch Error: {}",ret);
        }
        return Wiimote{dev:dev}
    }
}
