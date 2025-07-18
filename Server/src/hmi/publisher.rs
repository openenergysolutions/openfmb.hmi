// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use async_nats::RequestError;
use bytes::Bytes;

use openfmb_messages_ext::OpenFMBMessage;
use prost::{bytes, Message};

use crate::configuration::{NatsConfiguration, PubMessage};

async fn publish(nats_connection: &async_nats::Client, subject: String, buffer: bytes::Bytes) {
    if let Err(e) = nats_connection.publish(subject, buffer).await {
        log::error!("Failed to publish message.  Reason: {}", e);
    }
}

#[derive(Clone, Debug)]
pub struct Publisher {
    nats_connection: Option<async_nats::Client>,
    pubsub_config: NatsConfiguration,
}

impl Publisher {
    pub fn new(pubsub_config: NatsConfiguration) -> Self {
        Publisher {
            nats_connection: None,
            pubsub_config,
        }
    }

    pub async fn start(&mut self) {
        self.nats_connection = None;

        loop {
            match self.pubsub_config.connector().connect().await {
                Ok(c) => {
                    self.nats_connection = Some(c);
                    break;
                }
                Err(e) => {
                    log::error!("Failed to connect to nats: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    pub async fn request(
        &self,
        subject: String,
        payload: Bytes,
    ) -> Result<async_nats::Message, RequestError> {
        let conn = self.nats_connection.as_ref().unwrap();
        conn.request(subject, payload).await
    }

    pub async fn publish(&self, subject: String, buffer: &mut [u8]) {
        // convert the buffer to bytes
        let buffer = bytes::Bytes::copy_from_slice(buffer);
        let conn = self.nats_connection.as_ref().unwrap();
        publish(conn, subject, buffer).await;
    }

    pub async fn publish_message(&self, msg: PubMessage) {
        let buffer = bytes::Bytes::copy_from_slice(&msg.0.data);
        let conn = self.nats_connection.as_ref().unwrap();
        publish(conn, msg.0.subject, buffer).await;
    }

    pub async fn publish_openfmb_message(&self, msg: OpenFMBMessage) {
        use OpenFMBMessage::*;

        let profile_name = format!("{}Profile", msg.message_type());
        let module_name = msg.message_module();
        let mrid = msg.device_mrid().unwrap();

        let subject = format!("openfmb.{}.{}.{}", module_name, profile_name, mrid);

        let nats_client = self.nats_connection.as_ref().unwrap();
        let mut buffer = bytes::BytesMut::new();

        match &msg {
            BreakerDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            BreakerEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            BreakerReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            BreakerStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CapBankControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CapBankDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CapBankEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CapBankReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CapBankStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CircuitSegmentControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CircuitSegmentEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            CircuitSegmentStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ESSEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ESSReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ESSStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ESSControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            GenerationControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            GenerationDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            GenerationReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            GenerationEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            GenerationStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            LoadControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            LoadEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            LoadReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            LoadStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            MeterReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RecloserDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RecloserEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RecloserReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RecloserStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RegulatorControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RegulatorDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RegulatorEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RegulatorReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            RegulatorStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ResourceDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ResourceReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ResourceEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            ResourceStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SolarControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SolarEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SolarReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SolarStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SwitchDiscreteControl(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SwitchEvent(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SwitchReading(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            SwitchStatus(p) => match p.encode(&mut buffer) {
                Ok(_) => {
                    publish(nats_client, subject, buffer.freeze()).await;
                }
                Err(e) => log::error!("Error decoding message: {}", e),
            },
            _ => log::warn!("OpenFMB message not supported: {:?}", msg),
        }
    }
}
