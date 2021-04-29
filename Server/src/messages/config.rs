// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use config::Config;

#[derive(Clone, Debug)]
pub struct ActorConfig(Box<Config>);

impl Default for ActorConfig {
    fn default() -> Self {
        unimplemented!()
    }
}
