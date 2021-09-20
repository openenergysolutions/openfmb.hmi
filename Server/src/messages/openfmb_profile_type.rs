// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpenFMBProfileType {
    Breaker,
    CapBank,
    CoordinationService,
    ESS,
    Generation,
    Load,
    Meter,
    Recloser,
    Regulator,
    Resource,
    Solar,
    Switch,
}
