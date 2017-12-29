#[macro_use]
extern crate log;
extern crate env_logger;


fn main() {
    // available in "log":
    //
    // * error!
    // * warn!
    // * info!
    // * debug!
    // * trace!
    //

    // env_logger:
    // use "env RUST_LOG=info" to control log show up
    env_logger::init();

    info!("this is a test program");
    warn!("this is a wraning");
}
