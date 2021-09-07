// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { WeatherStatusDefinition } from "./models/userobject.model"

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
    buttonpcc: "button-pcc",
    weather: "weather"
}

export const CommandAction = {
    OPEN:           "Open",
    CLOSE:          "Close",
    TAP_LOWER_PHS3: "TapChangeLowerPhs3",
    TAP_RAISE_PHS3: "TapChangeRaisePhs3",
    TAP_LOWER_PHSA: "TapChangeLowerPhsA",
    TAP_RAISE_PHSA: "TapChangeRaisePhsA",
    TAP_LOWER_PHSB: "TapChangeLowerPhsB",
    TAP_RAISE_PHSB: "TapChangeRaisePhsB",
    TAP_LOWER_PHSC: "TapChangeLowerPhsC",
    TAP_RAISE_PHSC: "TapChangeRaisePhsC",
    SETVALUE:       "SetValue",
    PRECONFIGURED:  "PRECONFIGURED",
    VERB:           "VERB",
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

export const NetZeroState = {
    normal: 0,
    loadDischarge: 1,
    essDischarge: 2,
    essCharge: 3,
    disabled: 4,
    shuttingDown: 5
}

export const NetZeroStateString = {
    normal: 'normal',
    loadDischarge: 'load bank activated',
    essDischarge: 'ess discharging',
    essCharge: 'ess charging',
    disabled: 'disabled',
    shuttingDown: 'shutting down'
}

export const ButtonFunction = {
    link: 'link',
    command: 'command',
    setPoint: 'set-point'
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
            type === Symbol.pcc || type === Symbol.buttonpcc ||
            type === Symbol.button) {
            return true;
        }
        return false;
    },
    isDataConnectable: (type: string) => {
        //return type && type !== Symbol.label && type !== Symbol.text && type !== Symbol.rectangle;
        return type && type !== Symbol.text && type !== Symbol.rectangle;
    },
    isMeasureBox: (type: string) => {
        return type === Symbol.measureBox;
    },
    isSwitchgear: (type: string) => {
        if (type === Symbol.breaker || 
            type === Symbol.switchVertical || 
            type === Symbol.switchHorizontal || 
            type === Symbol.recloser ||
            type === Symbol.pcc || type === Symbol.buttonpcc) {
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
    isLabel: (type: string) => {
        return type === Symbol.label;
    },
    isStatusIndicator: (type: string) => {
        return type === Symbol.statusIndicator;
    },
    isWeather: (type: string) => {
        return type === Symbol.weather;
    },
    isVisibilitySupport: (type: string) => {
        return Hmi.isPowerFlow(type) || Hmi.isMeasureBox(type) || Hmi.isLabel(type) || Hmi.isStatusIndicator(type);
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
    convertNetZeroState: (pos: number) => {
        if (pos === NetZeroState.normal) {
            return NetZeroStateString.normal;
        }
        else if (pos === NetZeroState.loadDischarge) {
            return NetZeroStateString.loadDischarge;
        }
        else if (pos === NetZeroState.essDischarge) {
            return NetZeroStateString.essDischarge;
        }
        else if (pos === NetZeroState.essCharge) {
            return NetZeroStateString.essCharge;
        }
        else if (pos === NetZeroState.disabled) {
            return NetZeroStateString.disabled;
        }
        else if (pos === NetZeroState.shuttingDown) {
            return NetZeroStateString.shuttingDown;
        }
        else {
            return 'Unknown';
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
    getWeatherStatusIcon: (val: number, definitions: WeatherStatusDefinition[]) => {
        var ret = "";

        if (definitions) {
            for(var j = 0; j < definitions.length; ++j) {
                if (definitions[j].from <= val && val <= definitions[j].to) {
                    ret = definitions[j].text;
                    break;
                }
            }
        }

        return ret;
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