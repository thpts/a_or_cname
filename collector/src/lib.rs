#[macro_use]
extern crate diesel;

use chrono::prelude::Utc;
use std::time::Instant;

pub mod dns;
pub mod model;
pub mod schema;

/// Print timestamp and return Instant to assist in calculating run duration
///
/// # Arguments
/// * `message` - String template to print to STDOUT
pub fn start_processing_marker(message: String) -> Instant {
    println!("{}: {}", Utc::now().to_rfc3339(), message);
    return Instant::now();
}

/// Print end timestamp and calculate start delta
///
/// # Arguments
/// * `message` - String template to print to STDOUT.
/// * `start` - Starting point to calculate from
pub fn end_processing_marker(message: &str, start: Instant) {
    let duration = start.elapsed();
    println!(
        "{}: {} - taking {} seconds",
        Utc::now().to_rfc3339(),
        message,
        duration.as_secs()
    );
}
