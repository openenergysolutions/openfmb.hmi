use config::Config;
use openfmb_ops_protobuf::openfmb::commonmodule::{
    DbPosKind, EngGridConnectModeKind, GridConnectModeKind, StateKind,
};
use openfmb_ops_protobuf::openfmb::essmodule::EssStatus;
use openfmb_ops_protobuf::openfmb::switchmodule::SwitchStatus;
use std::{
    fmt::{Display, Formatter},
    time::SystemTime,
};

/// The state of the microgrid from a high level
///
/// In transient states the microgrid state may be Unknown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrogridState {
    Unknown,
    Islanded,
    Connected,
}

impl Display for MicrogridState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let state = match self {
            MicrogridState::Unknown => "Unknown",
            MicrogridState::Islanded => "Islanded",
            MicrogridState::Connected => "Connected",
        };
        write!(f, "{}", state)
    }
}

impl Default for MicrogridState {
    fn default() -> Self {
        MicrogridState::Unknown
    }
}

/// ReconnectingState describes the state (or step) the controller is in while
/// reconnecting the microgrid to the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconnectingState {
    VerifySwitchSynchro,
    VerifyEssSynchro,
    CleanupReconnect,
    VerifyReconnect,
}

/// Control state tracks the internal state the microgrid controller
/// is in. Typically these describe a transitional control objectives and the
/// step they are in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrogridControlState {
    Idle,
    Islanding,
    Reconnecting(ReconnectingState),
}

impl Display for ReconnectingState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let state = match self {
            ReconnectingState::VerifySwitchSynchro => "Verifying Switch Synchro Check",
            ReconnectingState::VerifyEssSynchro => "Verifying ESS Synchro Check",
            ReconnectingState::VerifyReconnect => "Verifying Reconnection",
            ReconnectingState::CleanupReconnect => "Cleaning Up After Reconnection",
        };
        write!(f, "{}", state)
    }
}

impl Display for MicrogridControlState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let state: String = match self {
            MicrogridControlState::Idle => "Idle".into(),
            MicrogridControlState::Islanding => "Islanding".into(),
            MicrogridControlState::Reconnecting(step) => format!("Reconnecting({})", step),
        };
        write!(f, "{}", state)
    }
}

impl Default for MicrogridControlState {
    fn default() -> Self {
        MicrogridControlState::Idle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetZeroState {
    Normal,
    LoadDischarge,
    ESSDischarge,
    ESSCharge,
    Disabled,
    ShuttingDown,
}

impl Display for NetZeroState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Default for NetZeroState {
    fn default() -> Self {
        NetZeroState::Disabled
    }
}

#[derive(Debug)]
pub struct CircuitSegmentStatus {
    pub ctl_state: MicrogridControlState,
    pub way1: SwitchState,
    pub way2: SwitchState,
    pub brk3: SwitchState,
    pub way3: SwitchState,
    pub way4: SwitchState,
    pub solar: SolarState,
    pub generator: GeneratorState,
    pub shop: ShopState,
    pub battery: BatteryState,
    pub loadbank: LoadbankState,
    pub algorithm: NetZeroState,
}

impl CircuitSegmentStatus {
    pub fn state(&self) -> MicrogridState {
        // Typhoon simulation looks at this battery grid mode to determine the islanded state
        let battery_grid_mode = if let Some(mode) = &self.battery.mode {
            if let Some(grid_mode) = GridConnectModeKind::from_i32(mode.set_val) {
                grid_mode
            } else {
                GridConnectModeKind::Undefined
            }
        } else {
            GridConnectModeKind::Undefined
        };

        match (self.way1.status, self.brk3.status, battery_grid_mode) {
            (Some(DbPosKind::Open), Some(DbPosKind::Closed), GridConnectModeKind::VsiIso) => {
                MicrogridState::Islanded
            }
            (Some(DbPosKind::Closed), Some(DbPosKind::Open), GridConnectModeKind::VsiPq) => {
                MicrogridState::Connected
            }
            _ => MicrogridState::Unknown,
        }
    }

    /// Update the control state returning the previous state
    pub fn update_ctl_state(&mut self, ctl_state: MicrogridControlState) -> MicrogridControlState {
        let old_state = self.ctl_state;
        self.ctl_state = ctl_state;
        old_state
    }

    /// Update the net zero state returning the previous state
    pub fn update_net_zero_state(&mut self, net_zero_state: NetZeroState) -> NetZeroState {
        let old_state = self.algorithm;
        self.algorithm = net_zero_state;
        old_state
    }
}

impl Display for CircuitSegmentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "\n\
             Microgrid State:      {}\n\
             Controller State:     {}\n\
             Algorithm State:      {}\n\
             way1:      {}\n\
             way2:      {}\n\
             brk3:      {}\n\
             way3:      {}\n\
             way4:      {}\n\
             solar:     {}\n\
             turbine:   {}\n\
             shopload:  {}\n\
             battery:   {}\n\
             loadbank:  {}\n\
             ",
            self.state(),
            self.ctl_state,
            self.algorithm,
            self.way1,
            self.way2,
            self.brk3,
            self.way3,
            self.way4,
            self.solar,
            self.generator,
            self.shop,
            self.battery,
            self.loadbank,
        )
    }
}

#[derive(Debug)]
pub struct BatteryState {
    pub status: Option<EssStatus>,
    pub state: Option<StateKind>,
    pub mode: Option<EngGridConnectModeKind>,
    pub soc: f32,
    pub charge_rate: f32,
    pub last_net: f32,
    pub min_soc: f32,
    pub max_soc: f32,
}

#[derive(Debug, Default)]
pub struct LoadbankState {
    pub state: Option<StateKind>,
    pub load_reading: f32,
}

impl Display for LoadbankState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for BatteryState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let status = match &self.status {
            None => "Unknown".to_string(),
            Some(status) => match status
                .ess_status_zgen
                .clone()
                .unwrap()
                .e_ss_event_and_status_zgen
            {
                Some(value) => {
                    if value.dynamic_test == None {
                        "None".to_string()
                    } else {
                        value
                            .dynamic_test
                            .clone()
                            .unwrap()
                            .st_val
                            .clone()
                            .to_string()
                    }
                }
                None => "Unknown".to_string(),
            },
        };

        let _state: &str = match self.state {
            Some(state) => match state {
                StateKind::Off => "Off",
                StateKind::On => "On",
                StateKind::Standby => "Standby",
            },
            None => "",
        };

        let _mode_str = match &self.mode {
            None => "None",
            Some(mode) => match GridConnectModeKind::from_i32(mode.set_val) {
                Some(GridConnectModeKind::Csi) => "Csi",
                Some(GridConnectModeKind::VcVsi) => "VcVsi",
                Some(GridConnectModeKind::CcVsi) => "CcVsi",
                Some(GridConnectModeKind::Undefined) => "Undefined",
                Some(GridConnectModeKind::None) => "None",
                Some(GridConnectModeKind::Other) => "Other",
                Some(GridConnectModeKind::VsiPq) => "VsiPq",
                Some(GridConnectModeKind::VsiVf) => "VsiVf",
                Some(GridConnectModeKind::VsiIso) => "VsiIso",
                None => "Unknown",
            },
        };

        let _xswi = match &self.status {
            None => None,
            Some(status) => Some(status),
        };

        write!(
            f,
            "Status: {}, SoC: {}%, Charge Rate: {}, Last Net: {}, Mode: {:?}, State: {:?}",
            status,
            self.soc * 100.0,
            self.charge_rate,
            self.last_net,
            self.mode,
            self.state,
        )
    }
}

#[derive(Default, Debug)]
pub struct SolarState {
    pub state: Option<StateKind>,
    pub last_net: f32,
}

impl Display for SolarState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "State: {:?}, Reading: {}", self.state, self.last_net)
    }
}

#[derive(Debug)]
pub struct ShopState {
    pub reading: ShopLoadReading,
    pub ts: SystemTime,
}

impl Default for ShopState {
    fn default() -> Self {
        ShopState {
            reading: ShopLoadReading::Unknown,
            ts: SystemTime::now(),
        }
    }
}

#[derive(Debug)]
pub enum ShopLoadReading {
    Unknown,
    Load(f32),
}

impl Display for ShopState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Load: {:?}, Time: {:?}]",
            self.reading,
            self.ts.elapsed()
        )
    }
}

impl ShopLoadReading {
    pub fn to_float(&self) -> f32 {
        match self {
            ShopLoadReading::Unknown => 0.0,
            ShopLoadReading::Load(f) => *f,
        }
    }
}

impl SolarState {
    pub fn to_float(&self) -> f32 {
        self.last_net
    }
}

#[derive(Default, Clone, Debug)]
pub struct GeneratorState {
    pub state: Option<StateKind>,
    pub last_net: f32,
    pub disabled: bool,
}

impl GeneratorState {
    pub fn kw(&self) -> f32 {
        self.last_net
    }
}

// enum BreakerState {
//     Open,
//     Closed,
// }
//
// enum ESSState {
//     Charging,
//     Discharging,
//     Off,
// }

impl Display for GeneratorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "State: {:?}, Reading: {}", self.state, self.last_net)
    }
}

#[derive(Default, Debug)]
pub struct SwitchState {
    pub status: Option<DbPosKind>,
    pub state: Option<SwitchStatus>,
    pub last_net: f32,
}

impl Display for SwitchState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // let status = match self.status {
        //     None => "Unknown",
        //     Some(status) => match status {
        //         DbPosKind::Transient => "Transient",
        //         DbPosKind::Closed => "Closed",
        //         DbPosKind::Invalid => "Invalid",
        //         DbPosKind::Open => "Open",
        //     },
        // };

        let xswi = match &self.state {
            None => None,
            Some(state) => state.switch_status_xswi.clone(),
        };

        let dynamic_test = match &xswi {
            None => None,
            Some(xswi) => xswi.dynamic_test.clone(),
        };

        // let synchro = self.status.unwrap().
        // let status = match &self.state {
        //     None => "Unknown".to_string(),
        //     Some(state) => match state.switch_status_xswi.clone() {
        //         Some(value) => value.dynamic_test.clone().unwrap().st_val.clone().to_string(),
        //         None => "Unknown".to_string(),

        //         // DbPosKind::Invalid => "Invalid",
        //         // DbPosKind::Open => "Open",
        //     },
        // };
        write!(
            f,
            "dynamic_test: {:?}, Status: {:?}, Reading: {:?}",
            dynamic_test, self.last_net, self.status
        )
    }
}
