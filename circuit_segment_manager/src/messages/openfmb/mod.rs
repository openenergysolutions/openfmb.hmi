use log::warn;
use openfmb_messages::commonmodule::StatusDps;
use chrono::{offset::TimeZone, DateTime, NaiveDateTime, Utc};
use enum_as_inner::EnumAsInner;

use openfmb_messages::{
    breakermodule::{BreakerDiscreteControlProfile, BreakerReadingProfile, BreakerStatusProfile},
    commonmodule::{
        ControlValue, DbPosKind, DynamicTestKind, EnsDynamicTestKind, IdentifiedObject, Timestamp,
    },
    essmodule::{EssControlProfile, EssReadingProfile, EssStatusProfile},
    generationmodule::{
        GenerationControlProfile, GenerationReadingProfile, GenerationStatusProfile,
    },
    loadmodule::{LoadControlProfile, LoadReadingProfile, LoadStatusProfile},
    metermodule::MeterReadingProfile,
    regulatormodule::{RegulatorReadingProfile, RegulatorStatusProfile},
    solarmodule::{SolarControlProfile, SolarReadingProfile, SolarStatusProfile},
    switchmodule::{SwitchDiscreteControlProfile, SwitchReadingProfile, SwitchStatusProfile},
    resourcemodule::{ResourceStatusProfile, ResourceDiscreteControlProfile},
};
use prost::Message;

use snafu::{OptionExt, ResultExt, Snafu};
use std::str::FromStr;

pub trait OpenFMBCommon {
    fn device_name(&self) -> Result<String, OpenFMBError>;
    fn device_mrid(&self) -> Result<Uuid, OpenFMBError>;
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError>;
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError>;
    fn summarize(&self) -> Result<String, OpenFMBError> {
        let mut result = String::new();
        result.push_str(&format!("Device Id: {}", self.device_mrid()?));
        result.push_str(&format!("Message Id: {}", self.message_mrid()?));
        result.push_str(&format!(
            "Message time: {}",
            DateTime::from(OpenFMBTimestampWrapper(self.message_timestamp()?))
        ));

        //todo summarize message payloads as well
        Ok(result)
        //  result.push_str()
    }
}

pub trait OpenFMBReading {}

#[derive(Debug)]
pub struct OpenFMBTimestampWrapper(pub Timestamp);

impl From<OpenFMBTimestampWrapper> for DateTime<Utc> {
    fn from(ts: OpenFMBTimestampWrapper) -> Self {
        datetime_from_timestamp(ts.0)
    }
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum OpenFMBMessage {
    GenerationReading(Box<GenerationReadingProfile>),
    GenerationStatus(Box<GenerationStatusProfile>),
    SwitchReading(Box<SwitchReadingProfile>),
    SwitchStatus(Box<SwitchStatusProfile>),
    MeterReading(Box<MeterReadingProfile>),
    SolarReading(Box<SolarReadingProfile>),
    SolarStatus(Box<SolarStatusProfile>),
    ESSReading(Box<EssReadingProfile>),
    ESSStatus(Box<EssStatusProfile>),
    LoadReading(Box<LoadReadingProfile>),
    LoadStatus(Box<LoadStatusProfile>),
//    ShuntReading(Box<ShuntReadingProfile>),
//    ShuntStatus(Box<ShuntStatusProfile>),
    RecloserReading(Box<RecloserReadingProfile>),
    RecloserStatus(Box<RecloserStatusProfile>),
    BreakerReading(Box<BreakerReadingProfile>),
    BreakerStatus(Box<BreakerStatusProfile>),
    RegulatorReading(Box<RegulatorReadingProfile>),
    RegulatorStatus(Box<RegulatorStatusProfile>),
    LoadControl(Box<LoadControlProfile>),
    SwitchControl(Box<SwitchDiscreteControlProfile>),
    EssControl(Box<EssControlProfile>),
    SolarControl(Box<SolarControlProfile>),
    GeneratorControl(Box<GenerationControlProfile>),
    BreakerControl(Box<BreakerDiscreteControlProfile>),
    ResourceStatus(Box<ResourceStatusProfile>),
    ResourceControl(Box<ResourceDiscreteControlProfile>),
}

impl OpenFMBMessage {
    //     pub fn into_inner(&self) ->  Box<dyn OpenFMBCommon> {
    //         match self {
    //             GenerationReading(msg) => Box::new(msg.clone() as Box<dyn OpenFMBCommon>),
    //             // GenerationStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // SwitchReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // SwitchStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // MeterReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // SolarReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // SolarStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // ESSReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // ESSStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // LoadReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // LoadStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // ShuntReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // ShuntStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // RecloserReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // RecloserStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // BreakerReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // BreakerStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // RegulatorReading(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //             // RegulatorStatus(msg)=> *msg as Box<dyn OpenFMBCommon>,
    //         }
    //     }

    pub fn message_type(&self) -> &str {
        use OpenFMBMessage::*;
        match self {
            GenerationReading(_) => "GenerationReading",
            GenerationStatus(_) => "GenerationStatus",
            SwitchReading(_) => "SwitchReading",
            SwitchStatus(_) => "SwitchStatus",
            MeterReading(_) => "MeterReading",
            SolarReading(_) => "SolarReading",
            SolarStatus(_) => "SolarStatus",
            ESSReading(_) => "ESSReading",
            ESSStatus(_) => "ESSStatus",
            LoadReading(_) => "LoadReading",
            LoadStatus(_) => "LoadStatus",
            // ShuntReading(_) => "ShuntReading",
            // ShuntStatus(_) => "ShuntStatus",
            RecloserReading(_) => "RecloserReading",
            RecloserStatus(_) => "RecloserStatus",
            BreakerReading(_) => "BreakerReading",
            BreakerStatus(_) => "BreakerStatus",
            RegulatorReading(_) => "RegulatorReading",
            RegulatorStatus(_) => "RegulatorStatus",
            LoadControl(_) => "LoadControl",
            SwitchControl(_) => "SwitchControl",
            EssControl(_) => "EssControl",
            SolarControl(_) => "SolarControl",
            GeneratorControl(_) => "GeneratorControl",
            BreakerControl(_) => "BreakerControl",
            ResourceStatus(_) => "ResourceStatus",
            ResourceControl(_) => "ResourceControl",
        }
    }
}

impl OpenFMBCommon for OpenFMBMessage {
    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        match self {
            GenerationReading(r) => r.device_mrid(),
            GenerationStatus(s) => s.device_mrid(),
            SwitchReading(r) => r.device_mrid(),
            SwitchStatus(s) => s.device_mrid(),
            MeterReading(r) => r.device_mrid(),
            SolarReading(r) => r.device_mrid(),
            SolarStatus(s) => s.device_mrid(),
            ESSReading(r) => r.device_mrid(),
            ESSStatus(s) => s.device_mrid(),
            LoadReading(r) => r.device_mrid(),
            LoadStatus(s) => s.device_mrid(),
            // ShuntReading(r) => r.device_mrid(),
            // ShuntStatus(s) => s.device_mrid(),
            RecloserReading(r) => r.device_mrid(),
            RecloserStatus(s) => s.device_mrid(),
            BreakerReading(r) => r.device_mrid(),
            BreakerStatus(s) => s.device_mrid(),
            RegulatorReading(r) => r.device_mrid(),
            RegulatorStatus(s) => s.device_mrid(),
            LoadControl(c) => c.device_mrid(),
            SwitchControl(c) => c.device_mrid(),
            EssControl(c) => c.device_mrid(),
            SolarControl(c) => c.device_mrid(),
            GeneratorControl(c) => c.device_mrid(),
            BreakerControl(c) => c.device_mrid(),
            ResourceStatus(c) => c.device_mrid(),
            ResourceControl(c) => c.device_mrid(),
        }
    }

    fn device_name(&self) -> Result<String, OpenFMBError> {
        match self {
            GenerationReading(r) => r.device_name(),
            GenerationStatus(s) => s.device_name(),
            SwitchReading(r) => r.device_name(),
            SwitchStatus(s) => s.device_name(),
            MeterReading(r) => r.device_name(),
            SolarReading(r) => r.device_name(),
            SolarStatus(s) => s.device_name(),
            ESSReading(r) => r.device_name(),
            ESSStatus(s) => s.device_name(),
            LoadReading(r) => r.device_name(),
            LoadStatus(s) => s.device_name(),
            // ShuntReading(r) => r.device_name(),
            // ShuntStatus(s) => s.device_name(),
            RecloserReading(r) => r.device_name(),
            RecloserStatus(s) => s.device_name(),
            BreakerReading(r) => r.device_name(),
            BreakerStatus(s) => s.device_name(),
            RegulatorReading(r) => r.device_name(),
            RegulatorStatus(s) => s.device_name(),
            LoadControl(c) => c.device_name(),
            SwitchControl(c) => c.device_name(),
            EssControl(c) => c.device_name(),
            SolarControl(c) => c.device_name(),
            GeneratorControl(c) => c.device_name(),
            BreakerControl(c) => c.device_name(),
            ResourceStatus(c) => c.device_name(),
            ResourceControl(c) => c.device_name(),
            ResourceControl(c) => c.device_name(),
        }
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        match self {
            GenerationReading(r) => r.message_mrid(),
            GenerationStatus(s) => s.message_mrid(),
            SwitchReading(r) => r.message_mrid(),
            SwitchStatus(s) => s.message_mrid(),
            MeterReading(r) => r.message_mrid(),
            SolarReading(r) => r.message_mrid(),
            SolarStatus(s) => s.message_mrid(),
            ESSReading(r) => r.message_mrid(),
            ESSStatus(s) => s.message_mrid(),
            LoadReading(r) => r.message_mrid(),
            LoadStatus(s) => s.message_mrid(),
            // ShuntReading(r) => r.message_mrid(),
            // ShuntStatus(s) => s.message_mrid(),
            RecloserReading(r) => r.message_mrid(),
            RecloserStatus(s) => s.message_mrid(),
            BreakerReading(r) => r.message_mrid(),
            BreakerStatus(s) => s.message_mrid(),
            RegulatorReading(r) => r.message_mrid(),
            RegulatorStatus(s) => s.message_mrid(),
            LoadControl(c) => c.message_mrid(),
            SwitchControl(c) => c.message_mrid(),
            EssControl(c) => c.message_mrid(),
            SolarControl(c) => c.message_mrid(),
            GeneratorControl(c) => c.message_mrid(),
            BreakerControl(c) => c.message_mrid(),
            ResourceStatus(c) => c.message_mrid(),
            ResourceControl(c) => c.message_mrid(),
            ResourceControl(c) => c.message_mrid(),
        }
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        match self {
            GenerationReading(r) => r.message_timestamp(),
            GenerationStatus(s) => s.message_timestamp(),
            SwitchReading(r) => r.message_timestamp(),
            SwitchStatus(s) => s.message_timestamp(),
            MeterReading(r) => r.message_timestamp(),
            SolarReading(r) => r.message_timestamp(),
            SolarStatus(s) => s.message_timestamp(),
            ESSReading(r) => r.message_timestamp(),
            ESSStatus(s) => s.message_timestamp(),
            LoadReading(r) => r.message_timestamp(),
            LoadStatus(s) => s.message_timestamp(),
            // ShuntReading(r) => r.message_timestamp(),
            // ShuntStatus(s) => s.message_timestamp(),
            RecloserReading(r) => r.message_timestamp(),
            RecloserStatus(s) => s.message_timestamp(),
            BreakerReading(r) => r.message_timestamp(),
            BreakerStatus(s) => s.message_timestamp(),
            RegulatorReading(r) => r.message_timestamp(),
            RegulatorStatus(s) => s.message_timestamp(),
            LoadControl(c) => c.message_timestamp(),
            SwitchControl(c) => c.message_timestamp(),
            EssControl(c) => c.message_timestamp(),
            SolarControl(c) => c.message_timestamp(),
            GeneratorControl(c) => c.message_timestamp(),
            BreakerControl(c) => c.message_timestamp(),
            ResourceStatus(c) => c.message_timestamp(),
            ResourceControl(c) => c.message_timestamp(), 
        }
    }
}

pub trait SwitchStatusExt {
    fn switch_status(&self) -> DbPosKind;
    fn dynamic_testing_enabled(&self) -> bool;
}

pub trait EssStatusExt {
    fn ess_gen_syn_t(&self) -> bool;
}

impl EssStatusExt for EssStatusProfile {
    fn ess_gen_syn_t(&self) -> bool {
        //      dbg!(self.ess_status.clone());
        match self
            .ess_status
            .clone()
            .unwrap()
            .ess_status_zgen
            .unwrap()
            .e_ss_event_and_status_zgen
            .unwrap()
            .gn_syn_st
        {
            Some(gn_syn_st) => gn_syn_st.st_val,
            None => false,
        }
    }
}

pub trait BreakerStatusExt {
    fn breaker_status(&self) -> DbPosKind;
}

impl BreakerStatusExt for BreakerStatusProfile {
    fn breaker_status(&self) -> DbPosKind {
        //      warn!("{:#?}",&self);
        //        unimplemented!()
        let st_val = self
            .breaker_status
            .clone()
            .unwrap()
            .status_and_event_xcbr
            .unwrap()
            .pos
            .unwrap()
            .phs3.unwrap().st_val;
        match st_val {
            0 => DbPosKind::Undefined,
            1 => DbPosKind::Transient,
            2 => DbPosKind::Closed,
            3 => DbPosKind::Open,
            4 => DbPosKind::Invalid,
            _ => unreachable!(),
        }
    }
}

impl SwitchStatusExt for SwitchStatusProfile {
    fn dynamic_testing_enabled(&self) -> bool {
        //        dbg!(self.switch_status.clone());
        match self
            .switch_status
            .clone()
            .unwrap()
            .switch_status_xswi
            .unwrap()
            .dynamic_test
        {
            Some(dynamic_test) => dynamic_test.st_val == DynamicTestKind::Testing as i32,
            None => false,
        }
    }

    fn switch_status(&self) -> DbPosKind {
        let st_val = self
            .switch_status
            .clone()
            .unwrap()
            .switch_status_xswi
            .unwrap()
            .pos
            .unwrap()
            .phs3.unwrap().st_val;
        match st_val {
            0 => DbPosKind::Undefined,
            1 => DbPosKind::Transient,
            2 => DbPosKind::Closed,
            3 => DbPosKind::Open,
            4 => DbPosKind::Invalid,
            _ => unreachable!(),
        }
    }
}

impl OpenFMBCommon for GenerationReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        self.generating_unit
            .clone()
            .context(NoGeneratingUnit)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .generating_unit
                .clone()
                .context(NoGeneratingUnit)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for SwitchDiscreteControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        // dbg!(self);
        match self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .identified_object
            .unwrap_or(IdentifiedObject {
                description: None,
                m_rid: None,
                name: Some("".to_string()),
            })
            .name
        {
            Some(name) => Ok(name),
            None => todo!(),
        }
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //dbg!(self);
        Uuid::from_str(
            &self
                .protected_switch
                .clone()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError) //FIXME is this the right mrid?
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoControlMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
}

impl OpenFMBCommon for BreakerDiscreteControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        // dbg!(self);
        match self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .identified_object
            .unwrap_or(IdentifiedObject {
                description: None,
                m_rid: None,
                name: Some("".to_string()),
            })
            .name
        {
            Some(name) => Ok(name),
            None => todo!(),
        }
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //warn!("{:#?}",self);
        Uuid::from_str(
            &self
                .breaker
                .clone()
                .context(NoBreaker)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError) //FIXME is this the right mrid?
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoControlMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
}

impl OpenFMBCommon for GenerationControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        // dbg!(self);
        match self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .identified_object
            .unwrap_or(IdentifiedObject {
                description: None,
                m_rid: None,
                name: Some("".to_string()),
            })
            .name
        {
            Some(name) => Ok(name),
            None => todo!(),
        }
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //dbg!(self);
        Uuid::from_str(
            &self
                .generating_unit
                .clone()
                .unwrap()
                .conducting_equipment
                .unwrap()
                .m_rid,
        )
        .context(UuidError) //FIXME is this the right mrid?
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoControlMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
}

impl OpenFMBCommon for SolarControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        // dbg!(self);
        match self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .identified_object
            .unwrap_or(IdentifiedObject {
                description: None,
                m_rid: None,
                name: Some("".to_string()),
            })
            .name
        {
            Some(name) => Ok(name),
            None => todo!(),
        }
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //dbg!(self);
        Uuid::from_str(
            &self
                .solar_inverter
                .clone()
                .context(NoSolarInverter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError) //FIXME is this the right mrid?
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoControlMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
}

impl OpenFMBCommon for EssControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .ess
            .clone()
            .context(NoEss)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .ess
                .clone()
                .context(NoEss)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoControlMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
}

impl OpenFMBCommon for LoadControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        // dbg!(self);
        match self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .identified_object
            .unwrap_or(IdentifiedObject {
                description: None,
                m_rid: None,
                name: Some("".to_string()),
            })
            .name
        {
            Some(name) => Ok(name),
            None => todo!(),
        }
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //dbg!(self);
        Uuid::from_str(
            &self
                .energy_consumer
                .clone()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError) //FIXME is this the right mrid?
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoControlMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .control_message_info
            .clone()
            .context(NoControlMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for SwitchReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .protected_switch
            .clone()
            .context(NoProtectedSwitch)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .clone()
                .context(NoGeneratingUnit)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }

    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for MeterReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .meter
            .clone()
            .context(NoMeter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .meter
                .clone()
                .context(NoMeter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for SolarReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .solar_inverter
            .clone()
            .context(NoSolarInverter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .solar_inverter
                .clone()
                .context(NoSolarInverter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for LoadReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .energy_consumer
            .clone()
            .context(NoEnergyConsumer)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .clone()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for EssReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .ess
            .clone()
            .context(NoEss)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .ess
                .clone()
                .context(NoEss)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for RegulatorReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .regulator_system
            .clone()
            .context(NoRegulatorSystem)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .regulator_system
                .clone()
                .context(NoRegulatorSystem)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for RecloserReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .recloser
            .clone()
            .context(NoRecloser)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .recloser
                .clone()
                .context(NoRecloser)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

// impl OpenFMBCommon for ShuntReadingProfile {
//     fn device_name(&self) -> Result<String, OpenFMBError> {
//         Ok(self
//             .shunt_system
//             .clone()
//             .context(NoShuntSystem)?
//             .conducting_equipment
//             .context(NoConductingEquipment)?
//             .named_object
//             .context(NoNamedObject)?
//             .name
//             .context(NoName)?)
//     }
//
//     fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
//         Ok(Uuid::from_str(
//             &self
//                 .shunt_system
//                 .clone()
//                 .context(NoShuntSystem)?
//                 .conducting_equipment
//                 .context(NoConductingEquipment)?
//                 .m_rid,
//         )
//         .context(UuidError)?)
//     }
//     fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
//         Ok(Uuid::from_str(
//             &self
//                 .reading_message_info
//                 .clone()
//                 .context(NoReadingMessageInfo)?
//                 .message_info
//                 .context(NoMessageInfo)?
//                 .identified_object
//                 .context(NoIdentifiedObject)?
//                 .m_rid
//                 .context(NoMRID)?,
//         )
//         .context(UuidError)?)
//     }
//     fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
//         Ok(self
//             .reading_message_info
//             .clone()
//             .context(NoReadingMessageInfo)?
//             .message_info
//             .context(NoMessageInfo)?
//             .message_time_stamp
//             .context(NoMessageTimestamp)?)
//     }
// }

impl OpenFMBCommon for BreakerReadingProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .breaker
            .clone()
            .context(NoBreaker)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .breaker
                .clone()
                .context(NoBreaker)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .reading_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .reading_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for GenerationStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .generating_unit
            .clone()
            .context(NoMeter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .generating_unit
                .clone()
                .context(NoGeneratingUnit)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for SolarStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .solar_inverter
            .clone()
            .context(NoSolarInverter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .solar_inverter
                .clone()
                .context(NoSolarInverter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for EssStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .ess
            .clone()
            .context(NoEss)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .ess
                .clone()
                .context(NoEss)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoReadingMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for SwitchStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .protected_switch
            .clone()
            .context(NoProtectedSwitch)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .clone()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for LoadStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .energy_consumer
            .clone()
            .context(NoEnergyConsumer)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .clone()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

// impl OpenFMBCommon for ShuntStatusProfile {
//     fn device_name(&self) -> Result<String, OpenFMBError> {
//         Ok(self
//             .shunt_system
//             .clone()
//             .context(NoShuntSystem)?
//             .conducting_equipment
//             .context(NoConductingEquipment)?
//             .named_object
//             .context(NoNamedObject)?
//             .name
//             .context(NoName)?)
//     }
//
//     fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
//         Ok(Uuid::from_str(
//             &self
//                 .shunt_system
//                 .clone()
//                 .context(NoShuntSystem)?
//                 .conducting_equipment
//                 .context(NoConductingEquipment)?
//                 .m_rid,
//         )
//         .context(UuidError)?)
//     }
//     fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
//         Ok(Uuid::from_str(
//             &self
//                 .status_message_info
//                 .clone()
//                 .context(NoStatusMessageInfo)?
//                 .message_info
//                 .context(NoMessageInfo)?
//                 .identified_object
//                 .context(NoIdentifiedObject)?
//                 .m_rid
//                 .context(NoMRID)?,
//         )
//         .context(UuidError)?)
//     }
//     fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
//         Ok(self
//             .status_message_info
//             .clone()
//             .context(NoReadingMessageInfo)?
//             .message_info
//             .context(NoMessageInfo)?
//             .message_time_stamp
//             .context(NoMessageTimestamp)?)
//     }
// }

impl OpenFMBCommon for RecloserStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .recloser
            .clone()
            .context(NoRecloser)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .recloser
                .clone()
                .context(NoRecloser)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for BreakerStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .breaker
            .clone()
            .context(NoBreaker)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .breaker
                .clone()
                .context(NoBreaker)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for RegulatorStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {
        Ok(self
            .regulator_system
            .clone()
            .context(NoRegulatorSystem)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .regulator_system
                .clone()
                .context(NoRegulatorSystem)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
        .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for ResourceStatusProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {        
        Ok(self           
            .clone()            
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //       panic!("{:#?}",self);
        Ok(Uuid::from_str(
            &self                
                .clone()                
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {        
        Ok(Uuid::from_str(
            &self
                .status_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
            .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {        
        Ok(self
            .status_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

impl OpenFMBCommon for ResourceDiscreteControlProfile {
    fn device_name(&self) -> Result<String, OpenFMBError> {        
        Ok(self           
            .clone()            
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }

    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        //       panic!("{:#?}",self);
        Ok(Uuid::from_str(
            &self                
                .clone()                
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {        
        Ok(Uuid::from_str(
            &self
                .control_message_info
                .clone()
                .context(NoStatusMessageInfo)?
                .message_info
                .context(NoMessageInfo)?
                .identified_object
                .context(NoIdentifiedObject)?
                .m_rid
                .context(NoMRID)?,
        )
            .context(UuidError)?)
    }
    fn message_timestamp(&self) -> Result<Timestamp, OpenFMBError> {        
        Ok(self
            .control_message_info
            .clone()
            .context(NoReadingMessageInfo)?
            .message_info
            .context(NoMessageInfo)?
            .message_time_stamp
            .context(NoMessageTimestamp)?)
    }
}

use openfmb_messages::reclosermodule::{
    RecloserReadingProfile, RecloserStatusProfile,
};

use std::convert::TryFrom;
use uuid::Uuid;
use OpenFMBMessage::*;

impl OpenFMBMessage {
    pub(crate) fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        let mrid = match self {
            GenerationReading(msg) => {
                // dbg!(msg.clone());
                msg.device_mrid()
            }
            GenerationStatus(msg) => msg.device_mrid(),
            SwitchReading(msg) => msg.device_mrid(),
            SwitchStatus(msg) => msg.device_mrid(),
            MeterReading(msg) => msg.device_mrid(),
            SolarReading(msg) => msg.device_mrid(),
            SolarStatus(msg) => msg.device_mrid(),
            ESSReading(msg) => msg.device_mrid(),
            ESSStatus(msg) => msg.device_mrid(),
            LoadReading(msg) => msg.device_mrid(),
            LoadStatus(msg) => msg.device_mrid(),
            // ShuntReading(msg) => msg.device_mrid(),
            // ShuntStatus(msg) => msg.device_mrid(),
            RecloserReading(msg) => msg.device_mrid(),
            RecloserStatus(msg) => msg.device_mrid(),
            BreakerReading(msg) => msg.device_mrid(),
            BreakerStatus(msg) => msg.device_mrid(),
            RegulatorReading(msg) => msg.device_mrid(),
            RegulatorStatus(msg) => msg.device_mrid(),
            LoadControl(msg) => msg.device_mrid(),
            SwitchControl(msg) => msg.device_mrid(),
            EssControl(msg) => msg.device_mrid(),
            SolarControl(msg) => msg.device_mrid(),
            GeneratorControl(msg) => msg.device_mrid(),
            BreakerControl(msg) => msg.device_mrid(),
            ResourceStatus(msg) => msg.device_mrid(),
            ResourceControl(msg) => msg.device_mrid(),
        };

        //todo!()
        mrid
    }
}

impl TryFrom<(&str, &str)> for OpenFMBMessage {
    type Error = OpenFMBError;

    fn try_from(parts: (&str, &str)) -> Result<Self, OpenFMBError> {
        let (profile, payload) = parts;
        let bytes = base64::decode(payload).unwrap();

        match profile {
            "GenerationReadingProfile" => Ok(GenerationReading(Box::new(
                GenerationReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "GenerationStatusProfile" => Ok(GenerationStatus(Box::new(
                GenerationStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SwitchReadingProfile" => Ok(SwitchReading(Box::new(
                SwitchReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SwitchStatusProfile" => Ok(SwitchStatus(Box::new(
                SwitchStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "MeterReadingProfile" => Ok(MeterReading(Box::new(
                MeterReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SolarReadingProfile" => Ok(SolarReading(Box::new(
                SolarReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SolarStatusProfile" => Ok(SolarStatus(Box::new(
                SolarStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ESSReadingProfile" => Ok(ESSReading(Box::new(
                EssReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ESSStatusProfile" => Ok(ESSStatus(Box::new(
                EssStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "LoadReadingProfile" => Ok(LoadReading(Box::new(
                LoadReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "LoadStatusProfile" => Ok(LoadStatus(Box::new(
                LoadStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            // "ShuntReadingProfile" => Ok(ShuntReading(Box::new(
            //     ShuntReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            // ))),
            // "ShuntStatusProfile" => Ok(ShuntStatus(Box::new(
            //     ShuntStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            // ))),
            "RecloserReadingProfile" => Ok(RecloserReading(Box::new(
                RecloserReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "RecloserStatusProfile" => Ok(RecloserStatus(Box::new(
                RecloserStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "BreakerReadingProfile" => Ok(BreakerReading(Box::new(
                BreakerReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "BreakerStatusProfile" => Ok(BreakerStatus(Box::new(
                BreakerStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "RegulatorReadingProfile" => Ok(RegulatorReading(Box::new(
                RegulatorReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "RegulatorStatusProfile" => Ok(RegulatorStatus(Box::new(
                RegulatorStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "LoadControlProfile" => Ok(LoadControl(Box::new(
                LoadControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SwitchDiscreteControlProfile" => Ok(SwitchControl(Box::new(
                SwitchDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ESSControlProfile" => Ok(EssControl(Box::new(
                EssControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SolarControlProfile" => Ok(SolarControl(Box::new(
                SolarControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "GenerationControlProfile" => Ok(GeneratorControl(Box::new(
                GenerationControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "BreakerDiscreteControlProfile" => Ok(BreakerControl(Box::new(
                BreakerDiscreteControlProfile::decode(bytes.as_slice())
                    .context(ProstDecodeError)?,
            ))),
            _ => Err(OpenFMBError::UnsupportedOpenFMBProfileError {
                profile: profile.to_string(),
            }),
        }
    }
}

impl TryFrom<&nats::Message> for OpenFMBMessage {
    type Error = OpenFMBError;

    fn try_from(msg: &nats::Message) -> Result<Self, OpenFMBError> {
        let bytes = &msg.data;
        let profile: Vec<&str> = msg.subject.split(".").collect();
        if profile.len() <=1 {
            warn!("PROFILE: {:?}", &profile);
        }
        let profile = profile.get(2).unwrap();
        match *profile {
            "GenerationReadingProfile" => Ok(GenerationReading(Box::new(
                GenerationReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "GenerationStatusProfile" => Ok(GenerationStatus(Box::new(
                GenerationStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SwitchReadingProfile" => Ok(SwitchReading(Box::new(
                SwitchReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SwitchStatusProfile" => Ok(SwitchStatus(Box::new(
                SwitchStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "MeterReadingProfile" => Ok(MeterReading(Box::new(
                MeterReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SolarReadingProfile" => Ok(SolarReading(Box::new(
                SolarReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SolarStatusProfile" => Ok(SolarStatus(Box::new(
                SolarStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ESSReadingProfile" => Ok(ESSReading(Box::new(
                EssReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ESSStatusProfile" => Ok(ESSStatus(Box::new(
                EssStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "LoadReadingProfile" => Ok(LoadReading(Box::new(
                LoadReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "LoadStatusProfile" => Ok(LoadStatus(Box::new(
                LoadStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            // "ShuntReadingProfile" => Ok(ShuntReading(Box::new(
            //     ShuntReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            // ))),
            // "ShuntStatusProfile" => Ok(ShuntStatus(Box::new(
            //     ShuntStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            // ))),
            "RecloserReadingProfile" => Ok(RecloserReading(Box::new(
                RecloserReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "RecloserStatusProfile" => Ok(RecloserStatus(Box::new(
                RecloserStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "BreakerReadingProfile" => Ok(BreakerReading(Box::new(
                BreakerReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "BreakerStatusProfile" => Ok(BreakerStatus(Box::new(
                BreakerStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "RegulatorReadingProfile" => Ok(RegulatorReading(Box::new(
                RegulatorReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "RegulatorStatusProfile" => Ok(RegulatorStatus(Box::new(
                RegulatorStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "LoadControlProfile" => Ok(LoadControl(Box::new(
                LoadControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SwitchDiscreteControlProfile" => Ok(SwitchControl(Box::new(
                SwitchDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ESSControlProfile" => Ok(EssControl(Box::new(
                EssControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "SolarControlProfile" => Ok(SolarControl(Box::new(
                SolarControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "GenerationControlProfile" => Ok(GeneratorControl(Box::new(
                GenerationControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "BreakerDiscreteControlProfile" => Ok(BreakerControl(Box::new(
                BreakerDiscreteControlProfile::decode(bytes.as_slice())
                    .context(ProstDecodeError)?,
            ))),
            "ResourceStatusProfile" => Ok(ResourceStatus(Box::new(
                ResourceStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?,
            ))),
            "ResourceDiscreteControlProfile" => Ok(ResourceControl(Box::new(
                ResourceDiscreteControlProfile::decode(bytes.as_slice())
                    .context(ProstDecodeError)?,
            ))),
            // "control" => Ok(MicrogridControlMsg(Box::new(MicrogridControl::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            _ => Err(OpenFMBError::UnsupportedOpenFMBProfileError {
                profile: profile.to_string(),
            }),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum OpenFMBError {
    #[snafu(display("Prost Decode Error"))]
    ProstDecodeError {
        source: prost::DecodeError,
    },
    #[snafu(display("Unsupported OpenFMBProfile"))]
    UnsupportedOpenFMBProfileError {
        profile: String,
    },
    #[snafu(display("Unsupported OpenFMB type"))]
    UnsupportedOpenFMBTypeError {
        fmb_type: String,
    },
    NoProtectedSwitch,
    NoDiscreteBreaker,
    NoConductingEquipment,
    NoReadingMessageInfo,
    NoControlMessageInfo,
    NoMessageInfo,
    NoIdentifiedObject,
    NoLogicalNode,
    NoLogicalControlNode,
    NoIED,
    InvalidOpenFMBMessage,
    NoMRID,
    NoStatusMessageInfo,
    NoMessageTimestamp,
    NoMeter,
    NoSolarInverter,
    NoEnergyConsumer,
    NoRegulatorSystem,
    NoEss,
    NoGenerationStatus,
    NoRecloser,
    NoControlValue,
    NoShuntSystem,
    NoBreaker,
    NoName,
    NoLoadControl,
    NoNamedObject,
    NoGeneratingUnit,
    #[snafu(display("Actor System Error"))]
    IOError {
        source: std::io::Error,
    },
    UuidError {
        source: uuid::Error,
    },
}

use openfmb_messages::{commonmodule};

pub fn get_current_timestamp() -> Timestamp {
    timestamp_from_datetime(Utc::now())
}

pub fn fraction_to_ms(fraction: u32) -> u32 {
    (fraction as f64 / 1000f64 * ((2 ^ 32) as f64)) as u32
}

pub fn ms_to_fraction(ms: u32) -> u32 {
    ((ms as f64) * 1000f64 / (2 ^ 32) as f64) as u32
}

pub fn timestamp_from_datetime(t: DateTime<Utc>) -> Timestamp {
    let tq = commonmodule::TimeQuality {
        clock_failure: false,
        clock_not_synchronized: false,
        leap_seconds_known: true,
        time_accuracy: commonmodule::TimeAccuracyKind::Unspecified as i32,
    };

    commonmodule::Timestamp {
        nanoseconds: (ms_to_fraction(t.timestamp_subsec_millis()) as u32),
        seconds: t.timestamp() as u64,
        tq: Some(tq),
    }
}

pub fn datetime_from_timestamp(t: Timestamp) -> DateTime<Utc> {
    let _nanoseconds: u32 =
        (t.nanoseconds as f64 / (2u64.pow(32) as f64) * 1000000000f64 as f64) as u32;

    let ms = fraction_to_ms(t.nanoseconds);
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(t.seconds as i64, ms), Utc)
}
