export const Symbol = {
    breaker: "breaker",
    twoStateButton: "two-state-button",
    switchVertical: "switch-vertical",
    switchHorizontal: "switch-horizontal",
    measureBox: "measure-box",
    label: "label",
    text: "text",
    button: "button",
    image: "image"
}

export const Hmi = {
    isControllable: (type: string) => {
        if (type === Symbol.breaker || type === Symbol.switchVertical || type == Symbol.switchHorizontal) {
            return true;
        }
        return false;
    },
    isDataConnectable: (type: string) => {
        return type !== Symbol.label;
    },
    isMeasureBox: (type: string) => {
        return type === Symbol.measureBox;
    }
}