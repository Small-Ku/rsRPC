pub fn log(message: impl AsRef<str>) {
  // If LOGS_ENABLED is 1, log the message with a timestamp
  if std::env::var("RSRPC_LOGS_ENABLED").unwrap_or("0".to_string()) == "1" {
    println!(
      "[{}] {}",
      chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
      message.as_ref()
    );
  }
}

#[macro_export]
macro_rules! log {
  ($($arg:tt)*) => {
    $crate::logger::log(format!($($arg)*))
  };
}
