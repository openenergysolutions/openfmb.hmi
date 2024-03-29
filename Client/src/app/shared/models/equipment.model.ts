// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export interface Equipment {
    mrid?: string,
    name?: string,
    device_type?: string
}

export const getEquipmentTypeList = () => {
    return EQUIPMENT_TYPES;
}

const EQUIPMENT_TYPES = [
    "generic",
    "breaker",
    "capbank",
    "coordinationservice",
    "ess",
    "generation",
    "load",
    "meter",
    "recloser",
    "regulator",
    "resource",
    "solar",
    "switch"
];