use log::{info, error, warn};
use fast_log::config::Config;
use fast_log::plugin::file_split::{RollingType, KeepType, Rolling, DateType};
use fast_log::plugin::packer::LogPacker;
use std::thread;
use std::time::Duration;

fn main() {
    fast_log::init(
        Config::new()
            .console()
            .file_split(
                "target/logs/",
                Rolling::new(RollingType::ByDate(DateType::Day)),
                KeepType::KeepNum(7),
                LogPacker {},
            )
            .chan_len(Some(100_000))
    )
    .expect("Failed to init fast_log");

    for i in 0..100 {
        info!("🚀 Tick {}", i);
        warn!("⚠️ Warning {}", i);
        if i % 10 == 0 {
            error!("❌ Error occurred at {}", i);
        }
        thread::sleep(Duration::from_millis(500));
    }

    log::logger().flush();
}
