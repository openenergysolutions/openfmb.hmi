export const Symbol = {
    breaker: "breaker",
    statusIndicator: "state-indicator",
    switchVertical: "switch-vertical",
    switchHorizontal: "switch-horizontal",
    measureBox: "measure-box",
    label: "label",
    text: "text",
    button: "button",
    setPointButton: "set-point-button",
    image: "image"
}

export const CommandAction = {
    OPEN: "OPEN",
    CLOSE: "CLOSE",
    SETVALUE: "SET-VALUE",
    PRECONFIGURED: "PRECONFIGURED",
    VERB: "VERB"
}

export const Pos = {
    transient: 0,
    closed: 1,
    open: 2,
    invalid: 3        
}

export const PosString = {
    transient: 'transient',
    closed: 'closed',
    open: 'open',
    invalid: 'invalid' 
}

export const Hmi = {
    isControllable: (type: string) => {
        if (type === Symbol.breaker || type === Symbol.switchVertical || type == Symbol.switchHorizontal || type == Symbol.setPointButton || type == Symbol.statusIndicator) {
            return true;
        }
        return false;
    },
    isDataConnectable: (type: string) => {
        return type !== Symbol.label && type !== Symbol.setPointButton && type !== Symbol.button;
    },
    isMeasureBox: (type: string) => {
        return type === Symbol.measureBox;
    },
    isSwitchgear: (type: string) => {
        if (type === Symbol.breaker || type === Symbol.switchVertical || type == Symbol.switchHorizontal) {
            return true;
        }
        return false;
    },
}

export const Helpers = {
    convertPos: (pos: any) => {
        if (pos === PosString.closed || pos === true || pos === 1 || pos === "1") {
            return PosString.closed;
        }
        else if (pos === PosString.open || pos === false || pos === 0 || pos === "0") {
            return PosString.open;
        }
        else {
            return PosString.invalid;
        }
    }
}