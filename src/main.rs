mod engine;
mod logger;

use std::process;
use structopt::StructOpt;

use crate::engine::{run_search, Config};
use crate::logger::{LogExtraParams, Logger, DEBUG};

fn main() {
    let args = Config::from_args();

    let logger = if args.debug { Some(DEBUG) } else { None };

    let logger = Logger::get_logger(logger);

    logger.debug("Huston firing up", None);

    let mut log_params = LogExtraParams::new();
    log_params.insert("pattern", &args.lookup_pattern);
    logger.debug("Got params", Some(&log_params));

    if let Err(e) = run_search(args, logger) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
