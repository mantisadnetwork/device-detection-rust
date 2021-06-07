#[derive(PartialEq, Debug)]
pub enum PlatformName {
    Android,
    MacOS,
    IOS,
    Windows,
    Other(String),
}

impl From<String> for PlatformName {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Android" => PlatformName::Android,
            "macOS" => PlatformName::MacOS,
            "iOS" => PlatformName::IOS,
            "Windows" => PlatformName::Windows,
            _ => PlatformName::Other(value)
        }
    }
}