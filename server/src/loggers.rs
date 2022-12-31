/// Writes an info! message to the app::req logger
#[macro_export]
macro_rules! req_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::req", $($arg)+);
    };
}

/// Writes an error! message to the app::req logger
#[macro_export]
macro_rules! req_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::req", $($arg)+);
    };
}

/// Writes a debug! message to the app::req logger
#[macro_export]
macro_rules! req_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::req", $($arg)+);
    };
}

/// Writes an info! message to the app::grpc logger
#[macro_export]
macro_rules! grpc_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::grpc", $($arg)+);
    };
}

/// Writes an error! message to the app::grpc logger
#[macro_export]
macro_rules! grpc_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::grpc", $($arg)+);
    };
}

/// Writes a debug! message to the app::grpc logger
#[macro_export]
macro_rules! grpc_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::grpc", $($arg)+);
    };
}
