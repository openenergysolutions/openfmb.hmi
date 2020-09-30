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

export const Helpers = {
    convertPos: (pos: any) => {
        if (pos === "closed" || pos === true || pos === 1 || pos === "1") {
            return PosString.closed;
        }
        else if (pos === "open" || pos === false || pos === 0 || pos === "0") {
            return PosString.open;
        }
        else {
            return PosString.invalid;
        }
    }
}