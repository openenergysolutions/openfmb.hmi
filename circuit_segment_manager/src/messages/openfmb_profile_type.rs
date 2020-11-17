#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpenFMBProfileType {
    Generation,
    Switch,
    Meter,
    Solar,
    ESS,
    Load,
    Shunt,
    Recloser,
    Breaker,
    Regulator,
    Resource,
    CNC,
}
