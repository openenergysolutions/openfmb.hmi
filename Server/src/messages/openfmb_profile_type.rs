#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpenFMBProfileType {
    Breaker,
    CapBank,
    CoordinationService,
    ESS,
    Generation,
    Load,
    Meter,
    Recloser,
    Regulator,
    Resource,
    Solar,
    Switch,    
}
