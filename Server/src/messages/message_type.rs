// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::hmi::pubsub::*;

#[derive(Clone, Debug)]
pub struct StartProcessing {
    pub pubsub_options: PubSubOptions
}