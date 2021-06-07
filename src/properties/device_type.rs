#[derive(PartialEq, Debug)]
pub enum DeviceType {
    Console,
    Desktop,
    EReader,
    MediaHub,
    Mobile,
    SmallScreen,
    SmartPhone,
    SmartWatch,
    Tablet,
    Tv,
}

impl DeviceType {
    pub fn from(value: String) -> Option<DeviceType> {
        match value.as_str() {
            "Console" => Some(DeviceType::Console),
            "Desktop" => Some(DeviceType::Desktop),
            "EReader" => Some(DeviceType::EReader),
            "MediaHub" => Some(DeviceType::MediaHub),
            "Mobile" => Some(DeviceType::Mobile),
            "SmallScreen" => Some(DeviceType::SmallScreen),
            "SmartPhone" => Some(DeviceType::SmartPhone),
            "SmartWatch" => Some(DeviceType::SmartWatch),
            "Tablet" => Some(DeviceType::Tablet),
            "Tv" => Some(DeviceType::Tv),
            _ => None
        }
    }
}