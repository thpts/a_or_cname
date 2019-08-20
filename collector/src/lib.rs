#[macro_use]
extern crate diesel;

use chrono::prelude::Utc;
use std::{thread,time};
use std::time::{Instant,SystemTime};

pub mod dns;
pub mod model;
pub mod schema;

static SLEEP_PERIOD: &'static time::Duration = &time::Duration::from_millis(100);

/// Log out time stamp with message
/// # Arguments
/// * `fmt` - String formatting to apply to variables, uses `format!`
/// * `args` - All subsequent arguments form variables in use
///
/// # Examples
/// ```
/// debug_msg!();
/// debug_msg!("Received error - {}", "something not right")
/// ```
macro_rules! debug_msg {
 ($fmt:expr, $($arg:tt)+) => {
    let msg = format!($fmt, $($arg)+);
    println!("{}: {}", Utc::now().to_rfc3339(), msg);
 };
}

/// Print timestamp and return Instant to assist in calculating run duration
///
/// # Arguments
/// * `message` - String template to print to STDOUT
pub fn start_processing_marker(message: String) -> Instant {
    debug_msg!("{}", message);
    return Instant::now();
}

/// Print end timestamp and calculate start delta
///
/// # Arguments
/// * `message` - String template to print to STDOUT.
/// * `start` - Starting point to calculate from
pub fn end_processing_marker(message: &str, start: Instant) {
    let duration = start.elapsed();
    debug_msg!("{} - taking {} seconds", message, duration.as_secs());
}

/// Return the current timestamp as seconds from UNIX Epoch
pub fn unix_time() -> i64 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
}

/// Log, then sleep for a given period
pub fn stall(message: String) {
    debug_msg!("Sleeping for {:?} - {}", SLEEP_PERIOD, message);
    thread::sleep(*SLEEP_PERIOD);
}