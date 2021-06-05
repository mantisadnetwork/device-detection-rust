#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/shim/shim.hpp");

        pub type Result;

        pub fn getValueAsString(self: &Result, propertyName: &str) -> Result<String>;
        pub fn getValueAsBool(self: &Result, propertyName: &str) -> Result<bool>;
        pub fn getValueAsInteger(self: &Result, propertyName: &str) -> Result<i32>;
        pub fn getValueAsDouble(self: &Result, propertyName: &str) -> Result<f64>;

        pub fn new_engine_hash(agent:String) -> UniquePtr<Engine>;

        pub type Engine;

        pub fn process(self: &Engine, userAgent:&str) -> UniquePtr<Result>;
    }
}

unsafe impl Send for ffi::Engine {}
unsafe impl Sync for ffi::Engine {}