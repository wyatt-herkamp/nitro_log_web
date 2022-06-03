pub mod tokio_comms;

use nitro_log::loggers::Logger;

pub trait LogCommunication<SenderType: Sender> {
    fn add_logger(&mut self, logger: &Logger) -> SenderType;
}

pub trait Sender: Send +Sync {}