// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use riker::actor::ActorRef;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ActorRefWrap<T: 'static + Send + Clone + Debug>(pub ActorRef<T>);
