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
    rectangle: "rectangle",
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
        if (type === Symbol.breaker || type === Symbol.switchVertical || type == Symbol.switchHorizontal || type == Symbol.setPointButton || type == Symbol.statusIndicator) {
            return true;
        }
        return false;
    },
    isDataConnectable: (type: string) => {
        return type  && type !== Symbol.label /*&& type !== Symbol.setPointButton*/ && type !== Symbol.button;
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