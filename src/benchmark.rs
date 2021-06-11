#![allow(warnings)]

use rayon::prelude::*;
use std::time::Instant;
use std::{time, thread};
use fiftyonedegrees::properties::PropertyName;
use fiftyonedegrees::api::DeviceDetection;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[cfg(debug_assertions)]
fn confirm(agent: &str, result: Result<Option<bool>, &str>) {
    // cargo run --example benchmark
    if let Ok(value) = result {
        if let Some(bool) = value {
            println!("{}: {}", bool, agent);
        } else {
            println!("invalid: {}", agent);
        }
    }
}

#[cfg(not(debug_assertions))]
fn confirm(_: &str, _: Result<Option<bool>, &str>) {
    // cargo run --example benchmark --release
}

// similar to https://github.com/51Degrees/device-detection-cxx/blob/master/examples/C/Hash/PerfHash.c
fn main() {
    let properties = vec![PropertyName::IsMobile];

    println!("Building Engine...");

    let mut engine = DeviceDetection::new("device-detection-cxx/device-detection-data/51Degrees-LiteV4.1.hash", properties);

    println!("Doing Lookup...");

    let file = File::open("device-detection-cxx/device-detection-data/20000 User Agents.csv").unwrap();

    let agents: Vec<String> = BufReader::new(file).lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let count = agents.len() as u32;

    println!("Benchmarking...");

    let now = Instant::now();

    agents.into_par_iter().for_each(|agent| {
        let result = engine.lookup(agent.as_str());

        // use this instead of HashGetDeviceIdFromResults to prove the lookup is real
        let value = result.getValueAsBoolean(&PropertyName::IsMobile);

        confirm(agent.as_str(), value);
    });

    let elapsed = now.elapsed();

    println!("\nTime per Lookup: {:.2?}\n", elapsed / count);
}