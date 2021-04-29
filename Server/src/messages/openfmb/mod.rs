// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use log::warn;
use openfmb_messages_ext::OpenFMBExt;
use openfmb_messages_ext::error::OpenFMBError;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use prost::Message;
use snafu::{ResultExt, Snafu};
use std::convert::TryFrom;
use uuid::Uuid;

use openfmb_messages::{
    breakermodule::*,
    capbankmodule::*,
    commonmodule::*,
    coordinationservicemodule::*,
    essmodule::*,
    generationmodule::*,
    loadmodule::*,
    metermodule::*,
    reclosermodule::*,
    regulatormodule::*,
    resourcemodule::*,
    solarmodule::*,
    switchmodule::*,
};

pub trait OpenFMBCommon {
    fn device_name(&self) -> Result<String, OpenFMBError>;
    fn device_mrid(&self) -> Result<Uuid, OpenFMBError>;
    fn message_mrid(&self) -> Result<Uuid, OpenFMBError>;    
    fn summarize(&self) -> Result<String, OpenFMBError> {
        let mut result = String::new();
        result.push_str(&format!("Device Id: {}", self.device_mrid()?));
        result.push_str(&format!("Message Id: {}", self.message_mrid()?));       
        Ok(result)        
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpenFMBMessage {
    BreakerDiscreteControl(Box<BreakerDiscreteControlProfile>),
    BreakerEvent(Box<BreakerEventProfile>),
    BreakerReading(Box<BreakerReadingProfile>),
    BreakerStatus(Box<BreakerStatusProfile>),
    CapBankControl(Box<CapBankControlProfile>),
    CapBankDiscreteControl(Box<CapBankDiscreteControlProfile>),
    CapBankEvent(Box<CapBankEventProfile>),
    CapBankReading(Box<CapBankReadingProfile>),
    CapBankStatus(Box<CapBankStatusProfile>),
    CoordinationControl(Box<CoordinationControlProfile>),
    CoordinationEvent(Box<CoordinationEventProfile>),
    CoordinationStatus(Box<CoordinationStatusProfile>),
    ESSEvent(Box<EssEventProfile>),
    ESSReading(Box<EssReadingProfile>),
    ESSStatus(Box<EssStatusProfile>),
    ESSControl(Box<EssControlProfile>),
    GenerationControl(Box<GenerationControlProfile>),
    GenerationDiscreteControl(Box<GenerationDiscreteControlProfile>),
    GenerationReading(Box<GenerationReadingProfile>),
    GenerationEvent(Box<GenerationEventProfile>),
    GenerationStatus(Box<GenerationStatusProfile>),
    LoadControl(Box<LoadControlProfile>),
    LoadEvent(Box<LoadEventProfile>),
    LoadReading(Box<LoadReadingProfile>),
    LoadStatus(Box<LoadStatusProfile>),
    MeterReading(Box<MeterReadingProfile>),
    RecloserDiscreteControl(Box<RecloserDiscreteControlProfile>),
    RecloserEvent(Box<RecloserEventProfile>),
    RecloserReading(Box<RecloserReadingProfile>),
    RecloserStatus(Box<RecloserStatusProfile>),
    RegulatorControl(Box<RegulatorControlProfile>),
    RegulatorDiscreteControl(Box<RegulatorDiscreteControlProfile>),
    RegulatorEvent(Box<RegulatorEventProfile>),
    RegulatorReading(Box<RegulatorReadingProfile>),
    RegulatorStatus(Box<RegulatorStatusProfile>),
    ResourceDiscreteControl(Box<ResourceDiscreteControlProfile>),
    ResourceReading(Box<ResourceReadingProfile>),
    ResourceEvent(Box<ResourceEventProfile>),
    ResourceStatus(Box<ResourceStatusProfile>),
    SolarControl(Box<SolarControlProfile>),
    SolarEvent(Box<SolarEventProfile>),
    SolarReading(Box<SolarReadingProfile>),
    SolarStatus(Box<SolarStatusProfile>),
    SwitchDiscreteControl(Box<SwitchDiscreteControlProfile>),
    SwitchEvent(Box<SwitchEventProfile>),
    SwitchReading(Box<SwitchReadingProfile>),
    SwitchStatus(Box<SwitchStatusProfile>),
}

impl OpenFMBMessage {    
    pub fn message_type(&self) -> &str {
        use OpenFMBMessage::*;
        match self {
            BreakerDiscreteControl(_) => "BreakerDiscreteControl",
            BreakerEvent(_) => "BreakerEvent",
            BreakerReading(_) => "BreakerReading",
            BreakerStatus(_) => "BreakerStatus",
            CapBankControl(_) => "CapBankControl",
            CapBankDiscreteControl(_) => "CapBankDiscreteControl",
            CapBankEvent(_) => "CapBankEvent",
            CapBankReading(_) => "CapBankReading",
            CapBankStatus(_) => "CapBankStatus",
            CoordinationControl(_) => "CoordinationControl",
            CoordinationEvent(_) => "CoordinationEvent",
            CoordinationStatus(_) => "CoordinationStatus",
            ESSEvent(_) => "ESSEvent",
            ESSReading(_) => "ESSReading",
            ESSStatus(_) => "ESSStatus",
            ESSControl(_) => "ESSControl",
            GenerationControl(_) => "GenerationControl",
            GenerationDiscreteControl(_) => "GenerationDiscreteControl",
            GenerationReading(_) => "GenerationReading",
            GenerationEvent(_) => "GenerationEvent",
            GenerationStatus(_) => "GenerationStatus",
            LoadControl(_) => "LoadControl",
            LoadEvent(_) => "LoadEvent",
            LoadReading(_) => "LoadReading",
            LoadStatus(_) => "LoadStatus",
            MeterReading(_) => "MeterReading",
            RecloserDiscreteControl(_) => "RecloserDiscreteControl",
            RecloserEvent(_) => "RecloserEvent",
            RecloserReading(_) => "RecloserReading",
            RecloserStatus(_) => "RecloserStatus",
            RegulatorControl(_) => "RegulatorControl",
            RegulatorDiscreteControl(_) => "RegulatorDiscreteControl",
            RegulatorEvent(_) => "RegulatorEvent",
            RegulatorReading(_) => "RegulatorReading",
            RegulatorStatus(_) => "RegulatorStatus",
            ResourceDiscreteControl(_) => "ResourceDiscreteControl",
            ResourceReading(_) => "ResourceReading",
            ResourceEvent(_) => "ResourceEvent",
            ResourceStatus(_) => "ResourceStatus",
            SolarControl(_) => "SolarControl",
            SolarEvent(_) => "SolarEvent",
            SolarReading(_) => "SolarReading",
            SolarStatus(_) => "SolarStatus",
            SwitchDiscreteControl(_) => "SwitchDiscreteControl",
            SwitchEvent(_) => "SwitchEvent",
            SwitchReading(_) => "SwitchReading",
            SwitchStatus(_) => "SwitchStatus",
        }
    }
}

impl OpenFMBCommon for OpenFMBMessage {
    fn device_mrid(&self) -> Result<Uuid, OpenFMBError> {
        use OpenFMBMessage::*;
        match self {
            BreakerDiscreteControl(p) => p.device_mrid(),
            BreakerEvent(p) => p.device_mrid(),
            BreakerReading(p) => p.device_mrid(),
            BreakerStatus(p) => p.device_mrid(),
            CapBankControl(p) => p.device_mrid(),
            CapBankDiscreteControl(p) => p.device_mrid(),
            CapBankEvent(p) => p.device_mrid(),
            CapBankReading(p) => p.device_mrid(),
            CapBankStatus(p) => p.device_mrid(),
            CoordinationControl(p) => p.device_mrid(),
            CoordinationEvent(p) => p.device_mrid(),
            CoordinationStatus(p) => p.device_mrid(),
            ESSEvent(p) => p.device_mrid(),
            ESSReading(p) => p.device_mrid(),
            ESSStatus(p) => p.device_mrid(),
            ESSControl(p) => p.device_mrid(),
            GenerationControl(p) => p.device_mrid(),
            GenerationDiscreteControl(p) => p.device_mrid(),
            GenerationReading(p) => p.device_mrid(),
            GenerationEvent(p) => p.device_mrid(),
            GenerationStatus(p) => p.device_mrid(),
            LoadControl(p) => p.device_mrid(),
            LoadEvent(p) => p.device_mrid(),
            LoadReading(p) => p.device_mrid(),
            LoadStatus(p) => p.device_mrid(),
            MeterReading(p) => p.device_mrid(),
            RecloserDiscreteControl(p) => p.device_mrid(),
            RecloserEvent(p) => p.device_mrid(),
            RecloserReading(p) => p.device_mrid(),
            RecloserStatus(p) => p.device_mrid(),
            RegulatorControl(p) => p.device_mrid(),
            RegulatorDiscreteControl(p) => p.device_mrid(),
            RegulatorEvent(p) => p.device_mrid(),
            RegulatorReading(p) => p.device_mrid(),
            RegulatorStatus(p) => p.device_mrid(),
            ResourceDiscreteControl(p) => p.device_mrid(),
            ResourceReading(p) => p.device_mrid(),
            ResourceEvent(p) => p.device_mrid(),
            ResourceStatus(p) => p.device_mrid(),
            SolarControl(p) => p.device_mrid(),
            SolarEvent(p) => p.device_mrid(),
            SolarReading(p) => p.device_mrid(),
            SolarStatus(p) => p.device_mrid(),
            SwitchDiscreteControl(p) => p.device_mrid(),
            SwitchEvent(p) => p.device_mrid(),
            SwitchReading(p) => p.device_mrid(),
            SwitchStatus(p) => p.device_mrid(),
        }
    }

    fn device_name(&self) -> Result<String, OpenFMBError> {
        use OpenFMBMessage::*;
        match self {
            BreakerDiscreteControl(p) => p.device_name(),
            BreakerEvent(p) => p.device_name(),
            BreakerReading(p) => p.device_name(),
            BreakerStatus(p) => p.device_name(),
            CapBankControl(p) => p.device_name(),
            CapBankDiscreteControl(p) => p.device_name(),
            CapBankEvent(p) => p.device_name(),
            CapBankReading(p) => p.device_name(),
            CapBankStatus(p) => p.device_name(),
            CoordinationControl(p) => p.device_name(),
            CoordinationEvent(p) => p.device_name(),
            CoordinationStatus(p) => p.device_name(),
            ESSEvent(p) => p.device_name(),
            ESSReading(p) => p.device_name(),
            ESSStatus(p) => p.device_name(),
            ESSControl(p) => p.device_name(),
            GenerationControl(p) => p.device_name(),
            GenerationDiscreteControl(p) => p.device_name(),
            GenerationReading(p) => p.device_name(),
            GenerationEvent(p) => p.device_name(),
            GenerationStatus(p) => p.device_name(),
            LoadControl(p) => p.device_name(),
            LoadEvent(p) => p.device_name(),
            LoadReading(p) => p.device_name(),
            LoadStatus(p) => p.device_name(),
            MeterReading(p) => p.device_name(),
            RecloserDiscreteControl(p) => p.device_name(),
            RecloserEvent(p) => p.device_name(),
            RecloserReading(p) => p.device_name(),
            RecloserStatus(p) => p.device_name(),
            RegulatorControl(p) => p.device_name(),
            RegulatorDiscreteControl(p) => p.device_name(),
            RegulatorEvent(p) => p.device_name(),
            RegulatorReading(p) => p.device_name(),
            RegulatorStatus(p) => p.device_name(),
            ResourceDiscreteControl(p) => p.device_name(),
            ResourceReading(p) => p.device_name(),
            ResourceEvent(p) => p.device_name(),
            ResourceStatus(p) => p.device_name(),
            SolarControl(p) => p.device_name(),
            SolarEvent(p) => p.device_name(),
            SolarReading(p) => p.device_name(),
            SolarStatus(p) => p.device_name(),
            SwitchDiscreteControl(p) => p.device_name(),
            SwitchEvent(p) => p.device_name(),
            SwitchReading(p) => p.device_name(),
            SwitchStatus(p) => p.device_name(),
        }
    }

    fn message_mrid(&self) -> Result<Uuid, OpenFMBError> {
        use OpenFMBMessage::*;
        match self {
            BreakerDiscreteControl(p) => p.message_mrid(),
            BreakerEvent(p) => p.message_mrid(),
            BreakerReading(p) => p.message_mrid(),
            BreakerStatus(p) => p.message_mrid(),
            CapBankControl(p) => p.message_mrid(),
            CapBankDiscreteControl(p) => p.message_mrid(),
            CapBankEvent(p) => p.message_mrid(),
            CapBankReading(p) => p.message_mrid(),
            CapBankStatus(p) => p.message_mrid(),
            CoordinationControl(p) => p.message_mrid(),
            CoordinationEvent(p) => p.message_mrid(),
            CoordinationStatus(p) => p.message_mrid(),
            ESSEvent(p) => p.message_mrid(),
            ESSReading(p) => p.message_mrid(),
            ESSStatus(p) => p.message_mrid(),
            ESSControl(p) => p.message_mrid(),
            GenerationControl(p) => p.message_mrid(),
            GenerationDiscreteControl(p) => p.message_mrid(),
            GenerationReading(p) => p.message_mrid(),
            GenerationEvent(p) => p.message_mrid(),
            GenerationStatus(p) => p.message_mrid(),
            LoadControl(p) => p.message_mrid(),
            LoadEvent(p) => p.message_mrid(),
            LoadReading(p) => p.message_mrid(),
            LoadStatus(p) => p.message_mrid(),
            MeterReading(p) => p.message_mrid(),
            RecloserDiscreteControl(p) => p.message_mrid(),
            RecloserEvent(p) => p.message_mrid(),
            RecloserReading(p) => p.message_mrid(),
            RecloserStatus(p) => p.message_mrid(),
            RegulatorControl(p) => p.message_mrid(),
            RegulatorDiscreteControl(p) => p.message_mrid(),
            RegulatorEvent(p) => p.message_mrid(),
            RegulatorReading(p) => p.message_mrid(),
            RegulatorStatus(p) => p.message_mrid(),
            ResourceDiscreteControl(p) => p.message_mrid(),
            ResourceReading(p) => p.message_mrid(),
            ResourceEvent(p) => p.message_mrid(),
            ResourceStatus(p) => p.message_mrid(),
            SolarControl(p) => p.message_mrid(),
            SolarEvent(p) => p.message_mrid(),
            SolarReading(p) => p.message_mrid(),
            SolarStatus(p) => p.message_mrid(),
            SwitchDiscreteControl(p) => p.message_mrid(),
            SwitchEvent(p) => p.message_mrid(),
            SwitchReading(p) => p.message_mrid(),
            SwitchStatus(p) => p.message_mrid(),       
        }
    }    
}

impl TryFrom<&nats::Message> for OpenFMBMessage {
    type Error = DecodeError;

    fn try_from(msg: &nats::Message) -> Result<Self, DecodeError> {
        let bytes = &msg.data;
        let profile: Vec<&str> = msg.subject.split(".").collect();
        if profile.len() <=1 {
            warn!("PROFILE: {:?}", &profile);
        }
        let profile = profile.get(2).unwrap();
        use OpenFMBMessage::*;
        match *profile {
            "BreakerDiscreteControlProfile" => Ok(BreakerDiscreteControl(Box::new(BreakerDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "BreakerEventProfile" => Ok(BreakerEvent(Box::new(BreakerEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "BreakerReadingProfile" => Ok(BreakerReading(Box::new(BreakerReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "BreakerStatusProfile" => Ok(BreakerStatus(Box::new(BreakerStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CapBankControlProfile" => Ok(CapBankControl(Box::new(CapBankControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CapBankDiscreteControlProfile" => Ok(CapBankDiscreteControl(Box::new(CapBankDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CapBankEventProfile" => Ok(CapBankEvent(Box::new(CapBankEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CapBankReadingProfile" => Ok(CapBankReading(Box::new(CapBankReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CapBankStatusProfile" => Ok(CapBankStatus(Box::new(CapBankStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CoordinationControlProfile" => Ok(CoordinationControl(Box::new(CoordinationControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CoordinationEventProfile" => Ok(CoordinationEvent(Box::new(CoordinationEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "CoordinationStatusProfile" => Ok(CoordinationStatus(Box::new(CoordinationStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ESSEventProfile" => Ok(ESSEvent(Box::new(EssEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ESSReadingProfile" => Ok(ESSReading(Box::new(EssReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ESSStatusProfile" => Ok(ESSStatus(Box::new(EssStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ESSControlProfile" => Ok(ESSControl(Box::new(EssControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "GenerationControlProfile" => Ok(GenerationControl(Box::new(GenerationControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "GenerationDiscreteControlProfile" => Ok(GenerationDiscreteControl(Box::new(GenerationDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "GenerationReadingProfile" => Ok(GenerationReading(Box::new(GenerationReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "GenerationEventProfile" => Ok(GenerationEvent(Box::new(GenerationEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "GenerationStatusProfile" => Ok(GenerationStatus(Box::new(GenerationStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "LoadControlProfile" => Ok(LoadControl(Box::new(LoadControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "LoadEventProfile" => Ok(LoadEvent(Box::new(LoadEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "LoadReadingProfile" => Ok(LoadReading(Box::new(LoadReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "LoadStatusProfile" => Ok(LoadStatus(Box::new(LoadStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "MeterReadingProfile" => Ok(MeterReading(Box::new(MeterReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RecloserDiscreteControlProfile" => Ok(RecloserDiscreteControl(Box::new(RecloserDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RecloserEventProfile" => Ok(RecloserEvent(Box::new(RecloserEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RecloserReadingProfile" => Ok(RecloserReading(Box::new(RecloserReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RecloserStatusProfile" => Ok(RecloserStatus(Box::new(RecloserStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RegulatorControlProfile" => Ok(RegulatorControl(Box::new(RegulatorControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RegulatorDiscreteControlProfile" => Ok(RegulatorDiscreteControl(Box::new(RegulatorDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RegulatorEventProfile" => Ok(RegulatorEvent(Box::new(RegulatorEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RegulatorReadingProfile" => Ok(RegulatorReading(Box::new(RegulatorReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "RegulatorStatusProfile" => Ok(RegulatorStatus(Box::new(RegulatorStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ResourceDiscreteControlProfile" => Ok(ResourceDiscreteControl(Box::new(ResourceDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ResourceReadingProfile" => Ok(ResourceReading(Box::new(ResourceReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ResourceEventProfile" => Ok(ResourceEvent(Box::new(ResourceEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "ResourceStatusProfile" => Ok(ResourceStatus(Box::new(ResourceStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SolarControlProfile" => Ok(SolarControl(Box::new(SolarControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SolarEventProfile" => Ok(SolarEvent(Box::new(SolarEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SolarReadingProfile" => Ok(SolarReading(Box::new(SolarReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SolarStatusProfile" => Ok(SolarStatus(Box::new(SolarStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SwitchDiscreteControlProfile" => Ok(SwitchDiscreteControl(Box::new(SwitchDiscreteControlProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SwitchEventProfile" => Ok(SwitchEvent(Box::new(SwitchEventProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SwitchReadingProfile" => Ok(SwitchReading(Box::new(SwitchReadingProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),
            "SwitchStatusProfile" => Ok(SwitchStatus(Box::new(SwitchStatusProfile::decode(bytes.as_slice()).context(ProstDecodeError)?))),          
            _ => Err(DecodeError::UnsupportedOpenFMBProfileError {
                profile: profile.to_string(),
            }),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum DecodeError {
    #[snafu(display("Prost Decode Error"))]
    ProstDecodeError {
        source: prost::DecodeError,
    },
    UnsupportedOpenFMBProfileError {
        profile: String,
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
