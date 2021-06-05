include!(concat!(env!("OUT_DIR"), "/DeviceDetectionCxx.rs"));

/*
#[repr(C)]
pub struct FiftyoneDegrees_Common_EngineBase__bindgen_vtable(::std::os::raw::c_void);

#[repr(C)]
#[derive(Debug)]
pub struct FiftyoneDegrees_Common_EngineBase {
    pub vtable_: *const FiftyoneDegrees_Common_EngineBase__bindgen_vtable,
    pub config: *mut FiftyoneDegrees_Common_ConfigBase,
    pub manager: [u64; 2usize],
    pub requiredProperties: *mut FiftyoneDegrees_Common_RequiredPropertiesConfig,
    pub metaData: *mut FiftyoneDegrees_Common_MetaData,
    pub licenceKey: std_string,
    pub updateUrl: std_string,
    pub defaultDataKey: std_string,
    pub keys: [u64; 3usize],
}

#[repr(C)]
#[derive(Debug)]
pub struct FiftyoneDegrees_DeviceDetection_EngineDeviceDetection {
    pub _base: FiftyoneDegrees_Common_EngineBase,
}

#[repr(C)]
#[derive(Debug)]
pub struct FiftyoneDegrees_DeviceDetection_Hash_EngineHash {
    pub _base: FiftyoneDegrees_DeviceDetection_EngineDeviceDetection,
}

extern "C" {
    #[link_name = "\u{1}__ZN15FiftyoneDegrees15DeviceDetection4Hash10EngineHash7processEPKc"]
    pub fn FiftyoneDegrees_DeviceDetection_Hash_EngineHash_process1(this: *mut FiftyoneDegrees_DeviceDetection_Hash_EngineHash, userAgent: *const ::std::os::raw::c_char) -> *mut FiftyoneDegrees_DeviceDetection_Hash_ResultsHash;
}

extern "C" {
    #[link_name = "\u{1}__ZN15FiftyoneDegrees15DeviceDetection4Hash10EngineHashC1EPKcPNS1_10ConfigHashEPNS_6Common24RequiredPropertiesConfigE"]
    pub fn FiftyoneDegrees_DeviceDetection_Hash_EngineHash_EngineHash(this: *mut FiftyoneDegrees_DeviceDetection_Hash_EngineHash, fileName: *const ::std::os::raw::c_char, config: *mut FiftyoneDegrees_DeviceDetection_Hash_ConfigHash, properties: *mut FiftyoneDegrees_Common_RequiredPropertiesConfig);
}

impl FiftyoneDegrees_DeviceDetection_Hash_EngineHash {
    #[inline]
    pub unsafe fn process1(&mut self, userAgent: *const ::std::os::raw::c_char) -> *mut FiftyoneDegrees_DeviceDetection_Hash_ResultsHash { FiftyoneDegrees_DeviceDetection_Hash_EngineHash_process1(self, userAgent) }

    #[inline]
    pub unsafe fn new(fileName: *const ::std::os::raw::c_char, config: *mut FiftyoneDegrees_DeviceDetection_Hash_ConfigHash, properties: *mut FiftyoneDegrees_Common_RequiredPropertiesConfig) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        FiftyoneDegrees_DeviceDetection_Hash_EngineHash_EngineHash(__bindgen_tmp.as_mut_ptr(), fileName, config, properties);
        __bindgen_tmp.assume_init()
    }
}
*/