use crate::config::Config;
use async_trait::async_trait;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties, ExchangeKind,
};
use tool::custom_error::CustomError;

#[async_trait]
pub trait IMQContext {
    async fn get_mq_connection(&self) -> Result<Connection, CustomError>;
    async fn send_message(
        &self,
        message: &str,
        queue: &str,
        routing_key: &str,
    ) -> Result<(), CustomError>;
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
    async fn send_message(
        &self,
        message: &str,
        queue_name: &str,
        routing_key: &str,
    ) -> Result<(), CustomError> {
        let connection = self.get_mq_connection().await?;
        let channel = connection.create_channel().await?;
        // let queue = channel
        //     .queue_declare(
        //         queue_name,
        //         QueueDeclareOptions {
        //             durable: true,
        //             ..QueueDeclareOptions::default()
        //         },
        //         FieldTable::default(),
        //     )
        //     .await?;
        channel
            .exchange_declare(
                "animal-exchange",
                ExchangeKind::Direct,
                ExchangeDeclareOptions {
                    durable: true,
                    ..ExchangeDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await?;
        // channel
        //     .queue_bind(
        //         queue.name().as_str(),
        //         "animal-exchange",
        //         routing_key,
        //         QueueBindOptions::default(),
        //         FieldTable::default(),
        //     )
        //     .await?;

        let publish_confirm = channel
            .basic_publish(
                "animal-exchange",
                routing_key,
                BasicPublishOptions::default(),
                message.as_bytes().to_vec(),
                BasicProperties::default(),
            )
            .await?;
        let confirm = publish_confirm.await?;
        tracing::info!("send_message result: {:#?}", confirm);
        Ok(())
    }
}
