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
    curve: "curve"
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
            type === Symbol.regulator) {
            return true;
        }
        return false;
    },
    isDataConnectable: (type: string) => {
        return type && type !== Symbol.label && type !== Symbol.text && type !== Symbol.rectangle && type !== Symbol.button;
    },
    isMeasureBox: (type: string) => {
        return type === Symbol.measureBox;
    },
    isSwitchgear: (type: string) => {
        if (type === Symbol.breaker || 
            type === Symbol.switchVertical || 
            type == Symbol.switchHorizontal || 
            type == Symbol.recloser) {
            return true;
        }
        return false;
    },
    isVoltageRegulator: (type: string) => {
        if (type === Symbol.regulator) {
            return true;
        }
        return false;
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