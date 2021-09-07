// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::hmi::pubsub::*;
use nats::Connection;

#[derive(Clone, Debug)]
pub struct StartProcessingMessages {
    pub pubsub_options: PubSubOptions,
    pub nats_client: Option<Connection>,
}