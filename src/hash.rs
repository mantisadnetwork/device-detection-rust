use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr::slice_from_raw_parts;
use crate::shim::ffi::{new_engine_hash};
use std::pin::Pin;
use std::borrow::BorrowMut;
use std::time::Instant;
use rayon::prelude::*;
use std::sync::Arc;

pub fn get_device(mut userAgent: String) {
    unsafe {
        let engine = new_engine_hash("/Users/pholley/Downloads/Enterprise-HashV41.hash".to_string());

        let now = Instant::now();

        (0..1000).into_par_iter().for_each(|_| {
            let results = engine.process("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36");

            let DeviceType = results.getValueAsInteger("DeviceType").unwrap();
            let IsSmartPhone = results.getValueAsBool("IsSmartPhone").unwrap();
            let IsTablet = results.getValueAsBool("IsTablet").unwrap();
            let HardwareName = results.getValueAsString("HardwareName").unwrap();
            let HardwareModel = results.getValueAsString("HardwareModel").unwrap();
            let HardwareVendor = results.getValueAsString("HardwareVendor").unwrap();
            let PlatformName = results.getValueAsString("PlatformName").unwrap();
            let PlatformVersion = results.getValueAsString("PlatformVersion").unwrap();
            let BrowserName = results.getValueAsString("BrowserName").unwrap();
            let BrowserVersion = results.getValueAsString("BrowserVersion").unwrap();
        });

        let elapsed = now.elapsed();
        println!("Elapsed: {:.10?}", elapsed);
    }
}