use crate::properties::device_type::DeviceType;
use crate::properties::platform_name::PlatformName;
use crate::properties::browser_name::BrowserName;

pub mod device_type;
pub mod platform_name;
pub mod browser_name;

pub enum PropertyName {
    DeviceType,
    IsSmartPhone,
    IsMobile,
    IsTablet,
    IsCrawler,
    HardwareName,
    HardwareModel,
    HardwareVendor,
    PlatformName,
    PlatformVersion,
    BrowserName,
    BrowserVersion,
}

impl From<&PropertyName> for usize {
    fn from(property_type: &PropertyName) -> Self {
        match property_type {
            PropertyName::DeviceType => 0,
            PropertyName::IsSmartPhone => 1,
            PropertyName::IsTablet => 2,
            PropertyName::HardwareName => 3,
            PropertyName::HardwareModel => 4,
            PropertyName::HardwareVendor => 5,
            PropertyName::PlatformName => 6,
            PropertyName::PlatformVersion => 7,
            PropertyName::BrowserName => 8,
            PropertyName::BrowserVersion => 9,
            PropertyName::IsMobile => 10,
            PropertyName::IsCrawler => 11
        }
    }
}

impl PropertyName {
    pub fn as_str(&self) -> &'static str {
        match self {
            PropertyName::DeviceType => "DeviceType",
            PropertyName::IsSmartPhone => "IsSmartPhone",
            PropertyName::IsTablet => "IsTablet",
            PropertyName::IsMobile => "IsMobile",
            PropertyName::HardwareName => "HardwareName",
            PropertyName::HardwareModel => "HardwareModel",
            PropertyName::HardwareVendor => "HardwareVendor",
            PropertyName::PlatformName => "PlatformName",
            PropertyName::PlatformVersion => "PlatformVersion",
            PropertyName::BrowserName => "BrowserName",
            PropertyName::BrowserVersion => "BrowserVersion",
            PropertyName::IsCrawler => "IsCrawler"
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PropertyBooleanValue {
    IsSmartPhone(bool),
    IsTablet(bool),
    IsCrawler(bool),
}

#[derive(PartialEq, Debug)]
pub enum PropertyStringValue<'a> {
    DeviceType(DeviceType),
    HardwareName(&'a str),
    HardwareModel(&'a str),
    HardwareVendor(&'a str),
    PlatformName(PlatformName),
    PlatformVersion(&'a str),
    BrowserName(BrowserName),
    BrowserVersion(&'a str),
}

impl PropertyStringValue<'_> {
    pub fn new<'a>(property_name: &PropertyName, value: &'a str) -> Option<PropertyStringValue<'a>> {
        match property_name {
            PropertyName::HardwareName => Some(PropertyStringValue::HardwareName(value)),
            PropertyName::HardwareVendor => Some(PropertyStringValue::HardwareVendor(value)),
            PropertyName::HardwareModel => Some(PropertyStringValue::HardwareModel(value)),
            PropertyName::DeviceType => match DeviceType::from(value) {
                Some(converted) => Some(PropertyStringValue::DeviceType(converted)),
                _ => None
            },
            PropertyName::PlatformName => Some(PropertyStringValue::PlatformName(PlatformName::from(value))),
            PropertyName::BrowserName => Some(PropertyStringValue::BrowserName(BrowserName::from(value))),
            PropertyName::PlatformVersion => Some(PropertyStringValue::PlatformVersion(value)),
            PropertyName::BrowserVersion => Some(PropertyStringValue::BrowserVersion(value)),
            _ => panic!("The property name provided does not support string lookup.")
        }
    }
}

impl PropertyBooleanValue {
    pub fn new(property_name: &PropertyName, value: bool) -> PropertyBooleanValue {
        match property_name {
            PropertyName::IsSmartPhone => PropertyBooleanValue::IsSmartPhone(value),
            PropertyName::IsTablet => PropertyBooleanValue::IsTablet(value),
            PropertyName::IsCrawler => PropertyBooleanValue::IsCrawler(value),
            _ => panic!("The property name provided does not support boolean lookup.")
        }
    }
}
