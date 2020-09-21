//! This crate is based on the `lighthouse/environment` crate at github.com/sigp/lighthouse.
//! A light version, as features relative to eth2 specs are not needed in this package.
//!
//! The idea is that the main thread creates an `Environment`, which is then used to spawn a
//! `Context` which can be handed to any service that wishes to start async tasks or perform
//! logging.

mod executor;
pub use executor::TaskExecutor;
use futures::channel::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};
use futures::{future, StreamExt};
use slog::{info, o, Drain, Level, Logger};
use sloggers::{null::NullLoggerBuilder, Build};
use std::cell::RefCell;
use std::ffi::OsStr;
use std::fs::{rename as FsRename, OpenOptions};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::{Builder as RuntimeBuilder, Runtime};

const LOG_CHANNEL_SIZE: usize = 2048;

/// Builds an `Environment`.
pub struct EnvironmentBuilder {
    runtime: Option<Runtime>,
    log: Option<Logger>,
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentBuilder {
    pub fn new() -> Self {
        Self {
            runtime: None,
            log: None,
        }
    }

    /// Specifies that all logs should be sent to `null` (i.e., ignored).
    pub fn null_logger(mut self) -> Result<Self, String> {
        let log_builder = NullLoggerBuilder;
        match log_builder.build() {
            Ok(null_logger) => {
                self.log = Some(null_logger);
                Ok(self)
            }
            Err(e) => Err(format!("Failed to start null logger: {:?}", e)),
        }
    }

    /// Sets the logger (and all child loggers) to log to a file.
    pub fn log_to_file(
        mut self,
        path: PathBuf,
        debug_level: &str,
        log_format: Option<&str>,
    ) -> Result<Self, String> {
        // Creating a backup if the logfile already exists.
        if path.exists() {
            let start = SystemTime::now();
            let timestamp = start
                .duration_since(UNIX_EPOCH)
                .map_err(|e| e.to_string())?
                .as_secs();
            let file_stem = path
                .file_stem()
                .ok_or_else(|| "Invalid file name".to_string())?
                .to_str()
                .ok_or_else(|| "Failed to create str from filename".to_string())?;
            let file_ext = path.extension().unwrap_or_else(|| OsStr::new(""));
            let backup_name = format!("{}_backup_{}", file_stem, timestamp);
            let backup_path = path.with_file_name(backup_name).with_extension(file_ext);
            FsRename(&path, &backup_path).map_err(|e| e.to_string())?;
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| format!("Unable to open logfile: {:?}", e))?;

        // Setting up the initial logger format and building it.
        let drain = if let Some(format) = log_format {
            match format.to_uppercase().as_str() {
                "JSON" => {
                    let drain = slog_json::Json::default(file).fuse();
                    slog_async::Async::new(drain)
                        .chan_size(LOG_CHANNEL_SIZE)
                        .build()
                }
                _ => return Err("Logging format provided is not supported".to_string()),
            }
        } else {
            let decorator = slog_term::PlainDecorator::new(file);
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            slog_async::Async::new(drain)
                .chan_size(LOG_CHANNEL_SIZE)
                .build()
        };

        let drain = match debug_level {
            "info" => drain.filter_level(Level::Info),
            "debug" => drain.filter_level(Level::Debug),
            "trace" => drain.filter_level(Level::Trace),
            "warn" => drain.filter_level(Level::Warning),
            "error" => drain.filter_level(Level::Error),
            "crit" => drain.filter_level(Level::Critical),
            unknown => return Err(format!("Unknown debug-level: {}", unknown)),
        };

        let log = Logger::root(drain.fuse(), o!());
        info!(
            log,
            "Logging to file";
            "path" => format!("{:?}", path)
        );

        self.log = Some(log);

        Ok(self)
    }

    /// Specifies that the `slog` asynchronous logger should be used. Ideal for production.
    ///
    /// The logger is "async" because it has a dedicated thread that accepts logs and then
    /// asynchronously flushes them to stdout/files/etc. This means the thread that raised the log
    /// does not have to wait for the logs to be flushed.
    pub fn async_logger(
        mut self,
        debug_level: &str,
        log_format: Option<&str>,
    ) -> Result<Self, String> {
        // Setting up the initial logger format and building it.
        let drain = if let Some(format) = log_format {
            match format.to_uppercase().as_str() {
                "JSON" => {
                    let drain = slog_json::Json::default(std::io::stdout()).fuse();
                    slog_async::Async::new(drain)
                        .chan_size(LOG_CHANNEL_SIZE)
                        .build()
                }
                _ => return Err("Logging format provided is not supported".to_string()),
            }
        } else {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            slog_async::Async::new(drain)
                .chan_size(LOG_CHANNEL_SIZE)
                .build()
        };

        let drain = match debug_level {
            "info" => drain.filter_level(Level::Info),
            "debug" => drain.filter_level(Level::Debug),
            "trace" => drain.filter_level(Level::Trace),
            "warn" => drain.filter_level(Level::Warning),
            "error" => drain.filter_level(Level::Error),
            "crit" => drain.filter_level(Level::Critical),
            unknown => return Err(format!("Unknown debug-level: {}", unknown)),
        };

        self.log = Some(Logger::root(drain.fuse(), o!()));
        Ok(self)
    }

    /// Specifies that a multi-threaded tokio runtime should be used. Ideal for production uses.
    ///
    /// The `Runtime` used is just the standard tokio runtime.
    pub fn multi_threaded_tokio_runtime(mut self) -> Result<Self, String> {
        self.runtime = Some(
            RuntimeBuilder::new()
                .threaded_scheduler()
                .enable_all()
                .build()
                .map_err(|e| format!("Failed to start runtime: {:?}", e))?,
        );
        Ok(self)
    }

    /// Consumes the builder, returning an `Environment`.
    pub fn build(self) -> Result<Environment, String> {
        let (signal, exit) = exit_future::signal();
        let (signal_tx, signal_rx) = channel(1);
        Ok(Environment {
            runtime: self
                .runtime
                .ok_or_else(|| "Cannot build environment without runtime".to_string())?,

            log: self
                .log
                .ok_or_else(|| "Cannot build environment without log".to_string())?,
            signal_tx,
            signal_rx: Some(signal_rx),
            signal: Some(signal),
            exit,
        })
    }
}

/// An environment where the service can run. Used to start a production API
/// or to run tests that involve logging and async task execution.
pub struct Environment {
    runtime: Runtime,
    log: Logger,

    /// Receiver side of an internal shutdown signal.
    signal_rx: Option<Receiver<&'static str>>,
    /// Sender to request shutting down.
    signal_tx: Sender<&'static str>,
    signal: Option<exit_future::Signal>,
    exit: exit_future::Exit,
}

impl Environment {
    /// Returns a mutable reference to the `tokio` runtime.
    ///
    /// Useful in the rare scenarios where it's necessary to block the current thread until a task
    /// is finished (e.g., during testing).
    pub fn runtime(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// Returns a `Context` where no "service" has been added to the logger output.
    pub fn core_context(&mut self) -> RuntimeContext {
        RuntimeContext {
            executor: TaskExecutor {
                exit: self.exit.clone(),
                signal_tx: self.signal_tx.clone(),
                handle: self.runtime().handle().clone(),
                log: self.log.clone(),
            },
        }
    }

    /// Block the current thread until a shutdown signal is received.
    ///
    /// This can be either the user Ctrl-C'ing or a task requesting to shutdown.
    pub fn block_until_shutdown_requested(&mut self) -> Result<(), String> {
        // future of a task requesting to shutdown
        let mut rx = self
            .signal_rx
            .take()
            .ok_or("Inner shutdown already received")?;
        let inner_shutdown =
            async move { rx.next().await.ok_or("Internal shutdown channel exhausted") };
        futures::pin_mut!(inner_shutdown);

        // setup for handling a Ctrl-C
        let (ctrlc_send, ctrlc_oneshot) = oneshot::channel();
        let ctrlc_send_c = RefCell::new(Some(ctrlc_send));
        ctrlc::set_handler(move || {
            if let Some(ctrlc_send) = ctrlc_send_c.try_borrow_mut().unwrap().take() {
                ctrlc_send.send(()).expect("Error sending ctrl-c message");
            }
        })
        .map_err(|e| format!("Could not set ctrlc handler: {:?}", e))?;

        // Block this thread until a shutdown signal is received.
        match self
            .runtime()
            .block_on(future::select(inner_shutdown, ctrlc_oneshot))
        {
            future::Either::Left((Ok(reason), _)) => {
                info!(self.log, "Internal shutdown received"; "reason" => reason);
                Ok(())
            }
            future::Either::Left((Err(e), _)) => Err(e.into()),
            future::Either::Right((x, _)) => x.map_err(|e| format!("Ctrlc oneshot failed: {}", e)),
        }
    }

    /// Fire exit signal which shuts down all spawned services
    pub fn fire_signal(&mut self) {
        if let Some(signal) = self.signal.take() {
            let _ = signal.fire();
        }
    }
}

/// An execution context that can be used by a service.
///
/// Distinct from an `Environment` because a `Context` is not able to give a mutable reference to a
/// `Runtime`, instead it only has access to a `Runtime`.
#[derive(Clone)]
pub struct RuntimeContext {
    pub executor: TaskExecutor,
}

impl RuntimeContext {
    /// Returns a reference to the logger for this service.
    pub fn log(&self) -> &slog::Logger {
        self.executor.log()
    }
}
