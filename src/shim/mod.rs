#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/shim/shim.hpp");

        pub type Result;

        pub fn getValueAsString(self: &Result, propertyName: i32) -> Result<String>;
        pub fn getValueAsBool(self: &Result, propertyName: i32) -> Result<bool>;
        pub fn getValueAsInteger(self: &Result, propertyName: i32) -> Result<i32>;
        pub fn getValueAsDouble(self: &Result, propertyName: i32) -> Result<f64>;

        pub fn new_engine(agent: &str, properties: Vec<&str>) -> UniquePtr<Engine>;

        pub type Engine;

        pub fn indexes(self: &Engine) -> Vec<i32>;
        pub fn lookup(self: &Engine, userAgent:&str) -> UniquePtr<Result>;
    }
}

unsafe impl Send for ffi::Engine {}
unsafe impl Sync for ffi::Engine {}