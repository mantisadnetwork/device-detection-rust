use crate::shim::ffi::{Engine, Result, new_engine};
use crate::properties::{PropertyValue, PropertyName};
use cxx::UniquePtr;

pub type PropertyIndexes = [i32; 10];

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

    pub fn getValueAsString(&self, property: PropertyName) -> std::result::Result<String, cxx::Exception> {
        self.result.getValueAsString(self.engine.mapping[usize::from(&property)])
    }

    pub fn getValueAsBoolean(&self, property: PropertyName) -> std::result::Result<bool, cxx::Exception> {
        self.result.getValueAsBool(self.engine.mapping[usize::from(&property)])
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

        let mut mapping: PropertyIndexes = [-1; 10];

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