use crate::infrastruct::{config::Config, custom_error::*};

use async_trait::async_trait;
use lapin::{
    options::*, //publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties,
    Connection,
    ConnectionProperties,
};

#[async_trait]
pub trait IMQContext {
    async fn get_mq_connection(&self) -> Result<Connection, CustomError>;
    async fn send_message(&self, t: &str, queue: &str) -> Result<(), CustomError>;
}

pub struct MQContext {}

impl MQContext {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IMQContext for MQContext {
    async fn get_mq_connection(&self) -> Result<Connection, CustomError> {
        let _config = Config::new();
        let conn_str = _config.amqp_addr;
        let connect = Connection::connect(&conn_str, ConnectionProperties::default()).await?;
        Ok(connect)
    }
    #[tracing::instrument(skip(self))]
    async fn send_message(&self, t: &str, queue: &str) -> Result<(), CustomError> {
        let connection = self.get_mq_connection().await?;
        let channel = connection.create_channel().await?;
        let _queue = channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await?;
        let publish_confirm = channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                t.as_bytes().to_vec(),
                BasicProperties::default(),
            )
            .await?;
        let confirm = publish_confirm.await?;
        tracing::info!("send_message result: {:#?}", confirm);
        Ok(())
    }
}
