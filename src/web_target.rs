use std::marker::PhantomData;
use log::Record;
use nitro_log::error::Error;
use nitro_log::loggers::Logger;
use nitro_log::loggers::target::{LoggerTarget, LoggerTargetBuilder};
use nitro_log::loggers::writer::LoggerWriter;
use nitro_log::placeholder::PlaceHolders;
use tokio::sync::{RwLock, RwLockWriteGuard};
use crate::comms::{LogCommunication, Sender};

pub struct WebLoggerTargetBuilder<SenderType: Sender, LC: LogCommunication<SenderType>> {
    pub(crate) logger_communication: RwLock<LC>,
    pub(crate) ph: PhantomData<SenderType>,
}

impl<SenderType: Sender +'static, LC: LogCommunication<SenderType>> LoggerTargetBuilder for WebLoggerTargetBuilder< SenderType, LC> {
    fn name(&self) -> &'static str {
        "web_logger"
    }

    fn build(&mut self, logger: &Logger, _: serde_json::value::Value, _: &PlaceHolders) -> Result<Box<dyn LoggerTarget>, Error> {
        let mut guard = self.logger_communication.blocking_write();
        let sender_type = guard.add_logger(logger);
        let target = WebLoggerTarget {
            sender: sender_type,
        };
        Ok(Box::new(target))
    }
}

pub struct WebLoggerTarget<SenderType: Sender> {
    pub sender: SenderType,
}

impl<SenderType: Sender> LoggerTarget for WebLoggerTarget<SenderType> {
    fn start_write<'log>(&'log self, record: &'log Record) -> anyhow::Result<LoggerWriter<'log>> {
        todo!()
    }
}