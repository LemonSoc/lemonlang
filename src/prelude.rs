use clap::Parser;
use env_logger::Env;

const DEBUG_LOG: &'static str = "debug";
const RELEASE_LOG: &'static str = "info";

pub(crate) fn startup_logger() {
    let (default_filter, default_write) = match in_release_build() {
        true => (RELEASE_LOG, "always"),
        false => (DEBUG_LOG, "always"),
    };

    let env = Env::default()
        .filter_or("MY_LOG", default_filter)
        .write_style_or("MY_LOG_STYLE", default_write);

    env_logger::init_from_env(env);

    if in_release_build() {
        log::trace!("This is a trace log. This should NOT be visible in release");
        log::debug!("This is a debug log. This should only be visible in debug builds.");
        log::info!("This is an info log. If you are in a release build, you should see [INFO] [WARN] and [ERROR] logs only.");
        log::warn!("This is a warning log. This should be visible in all builds.");
        log::error!("This is an error log. If you see this, something has gone horribly wrong.");

        log::info!("");
        log::info!("");
        log::info!("Logger set up successfully!");
    }
}

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(short, long)]
    pub file: String,
}

#[cfg(debug_assertions)]
#[inline(always)]
const fn in_release_build() -> bool {
    false
}

#[cfg(not(debug_assertions))]
#[inline(always)]
const fn in_release_build() -> bool {
    true
}
