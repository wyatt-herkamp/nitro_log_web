pub mod actix;
pub mod web_target;
pub mod comms;

use log::SetLoggerError;
use nitro_log::NitroLogger;
#[derive(Clone)]
pub struct WebLogger{
    pub(crate) nitro_logger: &'static NitroLogger,
}
impl WebLogger{
    pub fn new(nitro_logger: NitroLogger) -> Result<Self, SetLoggerError>{
        let value = Box::new(nitro_logger);
        let value = Box::leak::<'static>(value);
        let logger = Self {
            nitro_logger:value,
        };
        log::set_logger(value)?;
        Ok(logger)
    }
}