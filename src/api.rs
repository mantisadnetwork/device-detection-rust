use crate::shim::ffi::{Engine, Result, new_engine};
use crate::properties::{PropertyName, PropertyStringValue, PropertyBooleanValue};
use cxx::UniquePtr;
use crate::properties::device_type::DeviceType;

pub type PropertyIndexes = [i32; 11];

pub struct DeviceDetection {
    engine: UniquePtr<Engine>,
    mapping: PropertyIndexes,
}

pub struct DeviceDetectionResult<'a> {
    engine: &'a DeviceDetection,
    result: UniquePtr<Result>,
}


impl DeviceDetectionResult<'_> {
    pub fn getValueAsInteger(&self, property: PropertyName) -> std::result::Result<i32, cxx::Exception> {
        self.result.getValueAsInteger(self.engine.mapping[usize::from(&property)])
    }

    pub fn getValueAsPropertyString(&self, property: PropertyName) -> std::result::Result<Option<PropertyStringValue>, cxx::Exception> {
        let result = self.result.getValueAsString(self.engine.mapping[usize::from(&property)]);

        match result {
            Ok(value) => Ok(PropertyStringValue::new(&property, value)),
            Err(e) => Err(e)
        }
    }

    pub fn getValueAsString(&self, property: PropertyName) -> std::result::Result<String, cxx::Exception> {
        self.result.getValueAsString(self.engine.mapping[usize::from(&property)])
    }

    pub fn getValueAsBoolean(&self, property: PropertyName) -> std::result::Result<bool, cxx::Exception> {
        self.result.getValueAsBool(self.engine.mapping[usize::from(&property)])
    }

    pub fn getValueAsPropertyBoolean(&self, property: PropertyName) -> std::result::Result<PropertyBooleanValue, cxx::Exception> {
        let result = self.result.getValueAsBool(self.engine.mapping[usize::from(&property)]);

        match result {
            Ok(value) => Ok(PropertyBooleanValue::new(&property, value)),
            Err(e) => Err(e)
        }
    }
}

impl DeviceDetection {
    pub fn new(dataFile: &str, properties: Vec<PropertyName>) -> DeviceDetection {
        let mut converted = Vec::new();

        for property in &properties {
            converted.push(property.as_str());
        }

        let engine = unsafe {
            new_engine(dataFile, converted)
        };

        let indexes = engine.indexes();

        let mut mapping: PropertyIndexes = [-1; 11];

        for (propertyIndex, datasetIndex) in indexes.iter().enumerate() {
            mapping[usize::from(&properties[propertyIndex])] = *datasetIndex as i32;
        }

        DeviceDetection {
            engine,
            mapping,
        }
    }

    pub fn lookup(&self, userAgent: &str) -> DeviceDetectionResult {
        let result = self.engine.lookup(userAgent);

        DeviceDetectionResult {
            engine: &self,
            result,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::properties::PropertyName;
    use crate::api::DeviceDetection;
    use crate::properties::PropertyName::DeviceType;

    const ua: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_2 like Mac OS X) AppleWebKit/603.2.4 (KHTML, like Gecko) FxiOS/7.5b3349 Mobile/14F89 Safari/603.2.4";

    #[test]
    fn engine() {
        let properties = vec![
            PropertyName::PlatformName,
            PropertyName::BrowserName,
            PropertyName::IsMobile,
            PropertyName::PlatformVersion,
            PropertyName::BrowserVersion
        ];

        let engine = DeviceDetection::new("device-detection-cxx/device-detection-data/51Degrees-LiteV4.1.hash", properties);

        let matched = engine.lookup(ua);

        assert_eq!(matched.getValueAsBoolean(PropertyName::IsMobile).unwrap(), true);
        assert_eq!(matched.getValueAsString(PropertyName::BrowserName).unwrap(), "Firefox for iOS");
        assert_eq!(matched.getValueAsString(PropertyName::PlatformName).unwrap(), "iOS");
        assert_eq!(matched.getValueAsString(PropertyName::BrowserVersion).unwrap(), "7.5");
        assert_eq!(matched.getValueAsString(PropertyName::PlatformVersion).unwrap(), "10.3.2");
    }
}