use crate::properties::device_type::DeviceType;
use crate::properties::platform_name::PlatformName;
use crate::properties::browser_name::BrowserName;

pub mod device_type;
pub mod platform_name;
pub mod browser_name;

pub enum PropertyName {
    DeviceType,
    IsSmartPhone,
    IsTablet,
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
            PropertyName::BrowserVersion => 9
        }
    }
}

impl PropertyName {
    pub fn as_str(&self) -> &'static str {
        match self {
            PropertyName::DeviceType => "DeviceType",
            PropertyName::IsSmartPhone => "IsSmartPhone",
            PropertyName::IsTablet => "IsTablet",
            PropertyName::HardwareName => "HardwareName",
            PropertyName::HardwareModel => "HardwareModel",
            PropertyName::HardwareVendor => "HardwareVendor",
            PropertyName::PlatformName => "PlatformName",
            PropertyName::PlatformVersion => "PlatformVersion",
            PropertyName::BrowserName => "BrowserName",
            PropertyName::BrowserVersion => "BrowserVersion"
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum PropertyValue<'detector> {
    DeviceType(DeviceType),
    IsSmartPhone(bool),
    IsTablet(bool),
    HardwareName(&'detector str),
    HardwareModel(&'detector str),
    HardwareVendor(&'detector str),
    PlatformName(PlatformName<'detector>),
    PlatformVersion(&'detector str),
    BrowserName(BrowserName<'detector>),
    BrowserVersion(&'detector str),
}

fn value_to_bool(value: &str) -> Option<bool> {
    match value {
        "True" => Some(true),
        "False" => Some(false),
        _ => None
    }
}

impl<'detector> PropertyValue<'detector> {
    pub fn new(property_name: &PropertyName, value: &'detector str) -> Option<PropertyValue<'detector>> {
        match property_name {
            PropertyName::HardwareName => Some(PropertyValue::HardwareName(value)),
            PropertyName::HardwareVendor => Some(PropertyValue::HardwareVendor(value)),
            PropertyName::HardwareModel => Some(PropertyValue::HardwareModel(value)),
            PropertyName::IsSmartPhone => match value_to_bool(value) {
                Some(converted) => Some(PropertyValue::IsSmartPhone(converted)),
                _ => None
            },
            PropertyName::IsTablet => match value_to_bool(value) {
                Some(converted) => Some(PropertyValue::IsTablet(converted)),
                _ => None
            },
            PropertyName::DeviceType => match DeviceType::from_str(value) {
                Some(converted) => Some(PropertyValue::DeviceType(converted)),
                _ => None
            },
            PropertyName::PlatformName => Some(PropertyValue::PlatformName(PlatformName::from(value))),
            PropertyName::BrowserName => Some(PropertyValue::BrowserName(BrowserName::from(value))),
            PropertyName::PlatformVersion => Some(PropertyValue::PlatformVersion(value)),
            PropertyName::BrowserVersion => Some(PropertyValue::BrowserVersion(value))
        }
    }
}