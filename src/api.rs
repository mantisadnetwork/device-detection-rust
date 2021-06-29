use std::alloc::{alloc, Layout};
use crate::properties::{PropertyBooleanValue, PropertyName, PropertyStringValue};
use crate::properties::device_type::DeviceType;
use crate::shim::{mapPropertyToIndex, freeInit, freeResults, fiftyoneDegreesManagerResults, getResultsValue, initToResults, fiftyoneDegreesManagerInit, initToDataset, fiftyoneDegreesDataSetRelease, fiftyoneDegreesHashHighPerformanceConfig, fiftyoneDegreesConfigHash, fiftyoneDegreesDataSetGet, fiftyoneDegreesException, fiftyoneDegreesManagerInitFile, fiftyoneDegreesPropertiesGetRequiredPropertyIndexFromName, fiftyoneDegreesPropertiesRequired, fiftyoneDegreesResourceManager, fiftyoneDegreesResultsHash, fiftyoneDegreesResultsHashCreate, fiftyoneDegreesResultsHashFromUserAgent, fiftyoneDegreesResultsHashGetValuesString, fiftyoneDegreesResultsHashGetValuesStringByRequiredPropertyIndex, fiftyoneDegreesResourceHandleIncUse, fiftyoneDegreesDataSetBase};
use std::ffi::{CString, CStr};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::atomic::{AtomicPtr, Ordering};

pub type PropertyIndexes = [i32; 12];

pub struct DeviceDetection {
    init: fiftyoneDegreesManagerInit,
    mapping: PropertyIndexes,
}

impl Drop for DeviceDetection {
    fn drop(&mut self) {
        unsafe {
            freeInit(self.init);
        }
    }
}

unsafe impl Send for DeviceDetection {}

unsafe impl Sync for DeviceDetection {}

pub struct DeviceDetectionResult<'a> {
    mapping: &'a PropertyIndexes,
    results: fiftyoneDegreesManagerResults,
}

impl Drop for DeviceDetectionResult<'_> {
    fn drop(&mut self) {
        unsafe {
            freeResults(self.results);
        }
    }
}

// fiftyoneDegreesResultsHashFree
impl DeviceDetectionResult<'_> {
    pub fn getValueAsInteger(&self, property: &PropertyName) -> std::result::Result<Option<i32>, &str> {
        match self.getValueAsString(property) {
            Ok(value) => match value {
                Some(string) => match string.parse() {
                    Ok(int) => Ok(Some(int)),
                    Err(_) => Err("Unable to convert property string value to int")
                },
                None => Ok(None)
            },
            Err(e) => Err("Unable to get property as string")
        }
    }

    pub fn getValueAsPropertyString(&self, property: &PropertyName) -> std::result::Result<Option<PropertyStringValue>, &str> {
        match self.getValueAsString(property) {
            Ok(value) => match value {
                Some(string) => Ok(PropertyStringValue::new(property, string)),
                None => Ok(None)
            },
            Err(e) => Err("Unable to get property as string")
        }
    }

    pub fn getValueAsString(&self, property: &PropertyName) -> std::result::Result<Option<&str>, &str> {
        let string_ptr = unsafe { getResultsValue(&self.results, self.mapping[usize::from(property)]) };

        if std::ptr::null() == string_ptr {
            return Ok(None);
        }

        let string = unsafe {
            CStr::from_ptr(string_ptr)
        };

        // TODO: use from_utf8_unchecked to remove performance penalty of utf-8 check
        // https://github.com/rust-lang/rust/issues/75196
        match string.to_str() {
            Ok(str) => Ok(Some(str)),
            Err(e) => Err("The pointer returned from C is not a valid utf-8 string")
        }
    }

    pub fn getValueAsBoolean(&self, property: &PropertyName) -> std::result::Result<Option<bool>, &str> {
        match self.getValueAsString(property) {
            Ok(value) => match value {
                Some(string) => {
                    Ok(Some(string.eq("True")))
                }
                None => Ok(None)
            },
            Err(_) => Err("Unable to get property as string")
        }
    }

    pub fn getValueAsPropertyBoolean(&self, property: &PropertyName) -> std::result::Result<Option<PropertyBooleanValue>, &str> {
        let result = self.getValueAsBoolean(property);

        match result {
            Ok(value) => match value {
                Some(bool) => Ok(Some(PropertyBooleanValue::new(property, bool))),
                None => Ok(None)
            },
            Err(_) => Err("Unable to get property as boolean")
        }
    }
}

impl DeviceDetection {
    pub fn new(dataFile: &str, properties: Vec<PropertyName>) -> DeviceDetection {
        let mut converted = Vec::new();

        for property in &properties {
            converted.push(property.as_str());
        }

        let required_properties = CString::new(converted.join(",")).expect("CString::new failed");
        let file_name = CString::new(dataFile).expect("CString::new failed");

        let init = unsafe {
            fiftyoneDegreesManagerInitFile(required_properties.as_ptr(), file_name.as_ptr())
        };

        let mut mapping: PropertyIndexes = [-1; 12];

        for property in &properties {
            let property_string = CString::new(property.as_str()).expect("CString::new failed");

            mapping[usize::from(property)] = unsafe {
                mapPropertyToIndex(&init, property_string.as_ptr())
            };
        }

        DeviceDetection {
            init,
            mapping,
        }
    }

    pub fn lookup(&self, userAgent: &str) -> DeviceDetectionResult {
        let ua_string = CString::new(userAgent).expect("CString::new failed");

        let results = unsafe {
            initToResults(&self.init, ua_string.as_ptr())
        };

        DeviceDetectionResult {
            mapping: &self.mapping,
            results,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::DeviceDetection;
    use crate::properties::PropertyName;
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

        assert_eq!(matched.getValueAsBoolean(&PropertyName::IsMobile).unwrap().unwrap(), true);
        assert_eq!(matched.getValueAsString(&PropertyName::BrowserName).unwrap().unwrap(), "Firefox for iOS");
        assert_eq!(matched.getValueAsString(&PropertyName::PlatformName).unwrap().unwrap(), "iOS");
        assert_eq!(matched.getValueAsString(&PropertyName::BrowserVersion).unwrap().unwrap(), "7.5");
        assert_eq!(matched.getValueAsString(&PropertyName::PlatformVersion).unwrap().unwrap(), "10.3.2");

        // verify our drop code doesn't cause panics
        drop(matched);
        drop(engine);
    }
}
