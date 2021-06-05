#![allow(warnings)]

use rayon::prelude::*;
use fiftyonedegrees::api::{DeviceDetection};
use fiftyonedegrees::properties::PropertyName;
use std::time::Instant;

fn main() {
    let properties = vec![
        PropertyName::DeviceType,
        PropertyName::IsSmartPhone,
        PropertyName::IsTablet,
        PropertyName::HardwareName,
        PropertyName::HardwareModel,
        PropertyName::HardwareVendor,
        PropertyName::PlatformName,
        PropertyName::PlatformVersion,
        PropertyName::BrowserName,
        PropertyName::BrowserVersion
    ];

    println!("Building Engine...");

    let engine = DeviceDetection::new("/Users/pholley/Downloads/Enterprise-HashV41.hash", properties);

    println!("Doing Lookup...\n\n");

    let result = engine.lookup("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36");

    let deviceType = result.getValueAsString(PropertyName::DeviceType).unwrap();
    let isSmartPhone = result.getValueAsBoolean(PropertyName::IsSmartPhone).unwrap();
    let isTablet = result.getValueAsBoolean(PropertyName::IsTablet).unwrap();
    let hardwareName = result.getValueAsString(PropertyName::HardwareName).unwrap();
    let hardwareModel = result.getValueAsString(PropertyName::HardwareModel).unwrap();
    let hardwareVendor = result.getValueAsString(PropertyName::HardwareVendor).unwrap();
    let platformName = result.getValueAsString(PropertyName::PlatformName).unwrap();
    let platformVersion = result.getValueAsString(PropertyName::PlatformVersion).unwrap();
    let browserName = result.getValueAsString(PropertyName::BrowserName).unwrap();
    let browserVersion = result.getValueAsString(PropertyName::BrowserVersion).unwrap();

    println!("Device Type: {}", deviceType);
    println!("Is Smart Phone?: {}", isSmartPhone);
    println!("Is Tablet?: {}", isTablet);
    println!("Hardware Name: {}", hardwareName);
    println!("Hardware Model: {}", hardwareModel);
    println!("Hardware Vendor: {}", hardwareVendor);
    println!("Platform Name: {}", platformName);
    println!("Platform Version: {}", platformVersion);
    println!("Browser Name: {}", browserName);
    println!("Browser Version: {}", browserVersion);

    println!("\n\nBenchmarking...");

    let now = Instant::now();

    (0..1000000).into_par_iter().for_each(|_| {
        let result = engine.lookup("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36");

        result.getValueAsString(PropertyName::DeviceType).unwrap();
        result.getValueAsBoolean(PropertyName::IsSmartPhone).unwrap();
        result.getValueAsBoolean(PropertyName::IsTablet).unwrap();
        result.getValueAsString(PropertyName::HardwareName).unwrap();
        result.getValueAsString(PropertyName::HardwareModel).unwrap();
        result.getValueAsString(PropertyName::HardwareVendor).unwrap();
        result.getValueAsString(PropertyName::PlatformName).unwrap();
        result.getValueAsString(PropertyName::PlatformVersion).unwrap();
        result.getValueAsString(PropertyName::BrowserName).unwrap();
        result.getValueAsString(PropertyName::BrowserVersion).unwrap();
    });

    let elapsed = now.elapsed();

    println!("\nTime per Lookup (ms): {:.2?}\n", elapsed / 1000000);
}