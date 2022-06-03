use std::collections::HashMap;
use nitro_log::loggers::Logger;
use tokio::sync::broadcast;
use crate::comms::{LogCommunication, Sender, tokio_comms};

pub struct TokioCommunication {
    pub loggers: HashMap<String, broadcast::Receiver<String>>,
}

impl LogCommunication<TokioSender> for TokioCommunication {
    fn add_logger(&mut self, logger: &Logger) -> TokioSender {
        let (tx, mut rx) = broadcast::channel(1);
        self.loggers.insert(logger.logger_name.clone(), rx);

        TokioSender { sender: tx }
    }
}


pub struct TokioSender {
    pub(crate) sender: broadcast::Sender<String>,
}

impl Sender for TokioSender {}
