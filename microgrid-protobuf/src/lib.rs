include!(concat!(env!("OUT_DIR"), "/openfmb.microgrid.rs"));

use std::str::FromStr;

impl FromStr for microgrid_control::ControlMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<microgrid_control::ControlMessage, Self::Err> {
        match input {
            "ResetDevices"  => Ok(microgrid_control::ControlMessage::ResetDevices("".to_string())), 
            "Shutdown"  => Ok(microgrid_control::ControlMessage::Shutdown("".to_string())),
            "InitiateIsland"  => Ok(microgrid_control::ControlMessage::InitiateIsland("".to_string())),
            "InitiateGridConnect"  => Ok(microgrid_control::ControlMessage::InitiateGridConnect("".to_string())),
            "EnableNetZero"  => Ok(microgrid_control::ControlMessage::EnableNetZero("".to_string())),
            "DisableNetZero"  => Ok(microgrid_control::ControlMessage::DisableNetZero("".to_string())),             
            "ReconnectPretestOne"  => Ok(microgrid_control::ControlMessage::ReconnectPretestOne("".to_string())), 
            "ReconnectPretestTwo"  => Ok(microgrid_control::ControlMessage::ReconnectPretestTwo("".to_string())),
            "ReconnectTest"  => Ok(microgrid_control::ControlMessage::ReconnectTest("".to_string())), 
            "PccControl"  => Ok(microgrid_control::ControlMessage::PccControl("".to_string())),                    
            _ => Err(()),
        }
    }
}

impl FromStr for device_control::DeviceControlMessage {
    type Err = ();

    fn from_str(input: &str) -> Result<device_control::DeviceControlMessage, Self::Err> {
        match input {
            "EnableSolarInverter"  => Ok(device_control::DeviceControlMessage::EnableSolarInverter), 
            "DisableSolarInverter"  => Ok(device_control::DeviceControlMessage::DisableSolarInverter),
            "EnableLoadbank"  => Ok(device_control::DeviceControlMessage::EnableLoadbank),
            "DisableLoadbank"  => Ok(device_control::DeviceControlMessage::DisableLoadbank),
            "EssStart"  => Ok(device_control::DeviceControlMessage::EssStart),
            "EssDischarge"  => Ok(device_control::DeviceControlMessage::EssDischarge),
            "EssSocManage"  => Ok(device_control::DeviceControlMessage::EssSocManage),
            "EssSocLimits"  => Ok(device_control::DeviceControlMessage::EssSocLimits),
            "EssStop"  => Ok(device_control::DeviceControlMessage::EssStop),
            "GeneratorOn"  => Ok(device_control::DeviceControlMessage::GeneratorOn),
            "GeneratorDisabled"  => Ok(device_control::DeviceControlMessage::GeneratorDisabled),
            "GeneratorEnabled"  => Ok(device_control::DeviceControlMessage::GeneratorEnabled),
            "GeneratorOff"  => Ok(device_control::DeviceControlMessage::GeneratorOff),
            "SwitchOneOpen"  => Ok(device_control::DeviceControlMessage::SwitchOneOpen),
            "SwitchOneClosed"  => Ok(device_control::DeviceControlMessage::SwitchOneClosed),
            "SwitchTwoOpen"  => Ok(device_control::DeviceControlMessage::SwitchTwoOpen),
            "SwitchTwoClosed"  => Ok(device_control::DeviceControlMessage::SwitchTwoClosed),
            "BreakerThreeOpen"  => Ok(device_control::DeviceControlMessage::BreakerThreeOpen),
            "BreakerThreeClosed"  => Ok(device_control::DeviceControlMessage::BreakerThreeClosed),
            "SwitchFourOpen"  => Ok(device_control::DeviceControlMessage::SwitchFourOpen),
            "SwitchFourClosed"  => Ok(device_control::DeviceControlMessage::SwitchFourClosed),
            "ResetSwitchOne"  => Ok(device_control::DeviceControlMessage::ResetSwitchOne),
            "ResetSwitchTwo"  => Ok(device_control::DeviceControlMessage::ResetSwitchTwo),
            "ResetSwitchThree"  => Ok(device_control::DeviceControlMessage::ResetSwitchThree),
            "ResetSwitchFour"  => Ok(device_control::DeviceControlMessage::ResetSwitchFour),
            "ResetBreakerThree"  => Ok(device_control::DeviceControlMessage::ResetBreakerThree),
            "ResetEss"  => Ok(device_control::DeviceControlMessage::ResetEss),
            "ResetSolar"  => Ok(device_control::DeviceControlMessage::ResetSolar),
            "ResetLoadbank"  => Ok(device_control::DeviceControlMessage::ResetLoadbank),                             
            _ => Err(()),
        }
    }
}

impl FromStr for hmi_control::ControlType {
    type Err = ();

    fn from_str(input: &str) -> Result<hmi_control::ControlType, Self::Err> {
        match input {
            "ToggleEnvironment"  => Ok(hmi_control::ControlType::ToggleEnvironment),                
            _ => Err(()),
        }
    }
}

impl FromStr for generic_control::ControlType {
    type Err = ();

    fn from_str(input: &str) -> Result<generic_control::ControlType, Self::Err> {
        match input {
            "Open"  => Ok(generic_control::ControlType::Open),
            "Close"  => Ok(generic_control::ControlType::Close),
            "SetModBlkOn"  => Ok(generic_control::ControlType::SetModBlkOn),
            "SetModBlkOff"  => Ok(generic_control::ControlType::SetModBlkOff),
            "StateOn"  => Ok(generic_control::ControlType::StateOn),
            "StateOff"  => Ok(generic_control::ControlType::StateOff),
            "SetValue"  => Ok(generic_control::ControlType::SetValue),
            "SetWNetMag"  => Ok(generic_control::ControlType::SetWNetMag),
            "SetVarNetMag"  => Ok(generic_control::ControlType::SetVarNetMag),
            "TapChangeLowerPhs3"  => Ok(generic_control::ControlType::TapChangeLowerPhs3),
            "TapChangeRaisePhs3"  => Ok(generic_control::ControlType::TapChangeRaisePhs3), 
            "TapChangeLowerPhsA"  => Ok(generic_control::ControlType::TapChangeLowerPhsA), 
            "TapChangeRaisePhsA"  => Ok(generic_control::ControlType::TapChangeRaisePhsA), 
            "TapChangeLowerPhsB"  => Ok(generic_control::ControlType::TapChangeLowerPhsB), 
            "TapChangeRaisePhsB"  => Ok(generic_control::ControlType::TapChangeRaisePhsB), 
            "TapChangeLowerPhsC"  => Ok(generic_control::ControlType::TapChangeLowerPhsC), 
            "TapChangeRaisePhsC"  => Ok(generic_control::ControlType::TapChangeRaisePhsC),  
            _ => Err(()),
        }
    }
}