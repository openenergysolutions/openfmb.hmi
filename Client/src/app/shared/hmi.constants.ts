// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

export const Symbol = {
    breaker: "breaker",
    statusIndicator: "state-indicator",
    switchVertical: "switch-vertical",
    switchHorizontal: "switch-horizontal",
    recloser: "recloser",
    regulator: "regulator",
    measureBox: "measure-box",
    label: "label",
    text: "text",
    button: "button",
    setPointButton: "set-point-button",
    rectangle: "rectangle",
    image: "image",
    line: "line",
    curve: "curve",
    arrow: "arrow",
    flow: "flow",
    battery: "battery",
    batteryHorizontal: "battery-horizontal",
    battery3D : "battery3d",
    pcc: "pcc",
    pcc2: "pcc2"
}

export const CommandAction = {
    OPEN: "OPEN",
    CLOSE: "CLOSE",
    TAP_LOWER_PHS3: "TAP-LOWER-PHS3",
    TAP_RAISE_PHS3: "TAP-RAISE-PHS3",
    TAP_LOWER_PHSA: "TAP-LOWER-PHSA",
    TAP_RAISE_PHSA: "TAP-RAISE-PHSA",
    TAP_LOWER_PHSB: "TAP-LOWER-PHSB",
    TAP_RAISE_PHSB: "TAP-RAISE-PHSB",
    TAP_LOWER_PHSC: "TAP-LOWER-PHSC",
    TAP_RAISE_PHSC: "TAP-RAISE-PHSC",
    SETVALUE: "SET-VALUE",
    PRECONFIGURED: "PRECONFIGURED",
    VERB: "VERB",
}

export const Pos = {
    undefined: 0,
    transient: 1,
    closed: 2,
    open: 3,
    invalid: 4
}

export const PosString = {
    undefined: 'undefined',
    transient: 'transient',
    closed: 'closed',
    open: 'open',
    invalid: 'invalid' 
}

export const ButtonFunction = {
    link: 'link',
    command: 'command'
}

export const Hmi = {
    isControllable: (type: string) => {
        if (type === Symbol.breaker || 
            type === Symbol.switchVertical || 
            type === Symbol.switchHorizontal || 
            type === Symbol.setPointButton || 
            type === Symbol.statusIndicator ||
            type === Symbol.recloser || 
            type === Symbol.regulator ||
            type === Symbol.pcc ||
            type === Symbol.button) {
            return true;
        }
        return false;
    },
    isDataConnectable: (type: string) => {
        return type && type !== Symbol.label && type !== Symbol.text && type !== Symbol.rectangle;
    },
    isMeasureBox: (type: string) => {
        return type === Symbol.measureBox;
    },
    isSwitchgear: (type: string) => {
        if (type === Symbol.breaker || 
            type === Symbol.switchVertical || 
            type === Symbol.switchHorizontal || 
            type === Symbol.recloser ||
            type === Symbol.pcc) {
            return true;
        }
        return false;
    },
    isVoltageRegulator: (type: string) => {
        if (type === Symbol.regulator) {
            return true;
        }
        return false;
    },
    isBattery: (type: string) => {
        if (type === Symbol.battery || type === Symbol.batteryHorizontal || type === Symbol.battery3D) {
            return true;
        }
        return false;
    },
    isPowerFlow: (type: string) => {
        return type && (type.startsWith(Symbol.arrow) || type.startsWith(Symbol.flow));
    },
    isPCC: (type: string) => {
        return type === Symbol.pcc
    }    
}

export const Helpers = {
    convertPos: (pos: number) => {
        if (pos === Pos.closed) {
            return PosString.closed;
        }
        else if (pos === Pos.open) {
            return PosString.open;
        }
        else {
            return PosString.invalid;
        }
    },
    getBatteryPercentage: (val: number) => {
        if (1 < val && val <= 25) return 20;
        if (25 < val && val <= 50) return 40;
        if (50 < val && val <= 79) return 60;
        if (79 < val && val <= 96) return 80;
        if (96 < val && val <= 100) return 100;
        if (val > 100) return 100;
        return 0;
    },
    currentTimestamp: () => {
        return new Date().toLocaleString();
    },
    buttonBackColor: () => {
        return '#0e0f21';
    },
    buttonForeColor: () => {
        return '#a6a6af';
    }
}