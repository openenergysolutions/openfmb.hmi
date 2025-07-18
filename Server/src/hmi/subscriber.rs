// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use futures::StreamExt;
use openfmb_messages_ext::OpenFMBMessage;
use std::{error::Error, sync::Arc};

use crate::{
    configuration::{NatsConfiguration, SubMessage},
    processor::Processor,
};

#[derive(Clone, Debug)]
pub struct Subscriber {
    pubsub_config: NatsConfiguration,
    processor: Processor,
}

impl Subscriber {
    pub fn new(pubsub_config: NatsConfiguration, processor: Processor) -> Self {
        Subscriber {
            pubsub_config,
            processor,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("Start subscriber...");

        loop {
            match self.pubsub_config.connector().connect().await {
                Ok(c) => {
                    let client = c.clone();
                    let processor = self.processor.clone();
                    let mut sub = client.subscribe(">".to_string()).await?;

                    while let Some(msg) = sub.next().await {
                        let subject = msg.subject.clone();
                        let tokens = subject.split('.').collect::<Vec<&str>>();
                        let nats_msg = SubMessage(Arc::new(msg));

                        if tokens[0] == "openfmb" {
                            let result: Result<OpenFMBMessage, _> = nats_msg.0.as_ref().try_into();
                            if let Ok(msg) = result {
                                processor.receive_openfmb_message(msg).await;
                            } else {
                                log::warn!("Not an openfmb message: {}", subject);
                            }
                        }
                    }
                }
                Err(e) => log::error!("Failed to connect to nats: {}", e),
            }
        }
    }
}
