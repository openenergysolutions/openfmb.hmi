export interface Equipment {
    mrid?: string,
    name?: string,
    device_type?: string
}

export const getEquipmentTypeList = () => {
    return EQUIPMENT_TYPES;
}

const EQUIPMENT_TYPES = [
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