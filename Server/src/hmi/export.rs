// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use roxmltree::*;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Intersection {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Breaker {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Switch {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Recloser {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct EnergySource {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct CapBank {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
    #[serde(rename = "maxVa")]
    pub max_va: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Solar {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
    #[serde(rename = "maxVa")]
    pub max_va: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ESS {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
    #[serde(rename = "maxVa")]
    pub max_va: f32,
    #[serde(rename = "maxCharge")]
    pub max_charge: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ControllableLoad {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
    #[serde(rename = "watts")]
    pub watts: f32,
    #[serde(rename = "vars")]
    pub vars: f32,
    #[serde(rename = "normalOpen")]
    pub normal_open: bool,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Regulator {
    pub name: String,
    pub mrid: String,

    #[serde(rename = "cartCoords")]
    pub coordinate: Coordinate,
    #[serde(rename = "steps")]
    pub steps: i32,
    #[serde(rename = "normal")]
    pub normal: i32,
    #[serde(rename = "svi")]
    pub svi: f32,
    #[serde(rename = "phase")]
    pub phase: String,
    #[serde(rename = "group")]
    pub group: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(rename = "autoGridForm")]
    auto_grid_form: bool,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Devices {
    #[serde(rename = "breaker")]
    breakers: Vec<Breaker>,
    #[serde(rename = "cap")]
    capbanks: Vec<CapBank>,
    #[serde(rename = "controllableLoad")]
    controllable_loads: Vec<ControllableLoad>,
    #[serde(rename = "energySource")]
    energy_sources: Vec<EnergySource>,
    #[serde(rename = "ess")]
    energy_storages: Vec<ESS>,
    #[serde(rename = "recloser")]
    reclosers: Vec<Recloser>,
    #[serde(rename = "solar")]
    solars: Vec<Solar>,
    #[serde(rename = "switch")]
    switches: Vec<Switch>,
    #[serde(rename = "regulator")]
    regulators: Vec<Regulator>,
    #[serde(rename = "intersection")]
    intersections: Vec<Intersection>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Voltage {
    pub name: String,
    pub mrid: String,
    pub voltage: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Segment {
    pub id: String,
    pub device1: String,
    pub device2: String,
    #[serde(rename = "r")]
    pub r: f32,
    #[serde(rename = "watts")]
    pub watts: f32,
    #[serde(rename = "vars")]
    pub vars: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Powers {
    voltages: Vec<Voltage>,
    segments: Vec<Segment>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Reading {
    net: f32,
    a: f32,
    b: f32,
    c: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Input {
    volt: Reading,
    va: Reading,
    var: Reading,
    watt: Reading,

    #[serde(rename = "openClose")]
    open_close: u32,
    percentage: f32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Output {
    #[serde(rename = "openClose")]
    open_close: u32,
    percentage: f32,
    watts: u32,
    vars: u32,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DNP3Device {
    pub name: String,
    pub mrid: String,
    pub outstation: String,
    pub profile: String,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    name: String,
    input: Input,
    output: Output,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DNP3 {
    #[serde(rename = "threadHint")]
    thread_hint: u32,
    profiles: Vec<Profile>,
    devices: Vec<DNP3Device>,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Mav {
    settings: Settings,
    devices: Devices,
    connections: Vec<Vec<String>>,
    powers: Powers,
    dnp3: DNP3,
}

// TODO
fn transform_coords(x: f32, y: f32) -> (f32, f32) {
    (x, y)
}

pub fn export() {
    let filename = "design.xml";

    match File::open(&filename) {
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            // Create xml document
            let doc = Document::parse(&content).unwrap();

            // Root tag
            let root = doc.descendants().find(|n| n.has_tag_name("root")).unwrap();

            let mut mav = Mav::default();

            for elem in root.children() {
                if elem.node_type() == NodeType::Element {
                    if let Some(parent) = elem.attribute("parent") {
                        if parent == "1" {
                            // Get mxGeometry
                            match elem.children().find(|n| n.has_tag_name("mxGeometry")) {
                                Some(geometry) => {
                                    if let Some(relative) = geometry.attribute("relative") {
                                        if relative == "1" {
                                            println!("This is a line");
                                        }
                                    } else {
                                        // Get coordinate
                                        let x = geometry
                                            .attribute("x")
                                            .unwrap_or("0.0")
                                            .parse::<f32>()
                                            .unwrap_or(0.0);
                                        let y = geometry
                                            .attribute("y")
                                            .unwrap_or("0.0")
                                            .parse::<f32>()
                                            .unwrap_or(0.0);

                                        let mut asset_type = "";
                                        let mut mrid = "";
                                        let mut label = "";

                                        let coords: (f32, f32) = transform_coords(x, y);

                                        match elem.children().find(|n| n.has_tag_name("Object")) {
                                            Some(obj) => match obj
                                                .children()
                                                .find(|n| n.has_tag_name("Object"))
                                            {
                                               
                                                Some(obj) => {
                                                     // Get type
                                                    match obj.attribute("type") {
                                                        Some(atype) => {
                                                            asset_type = atype;
                                                        }
                                                        None => {
                                                            log::error!("Unable to get asset type");
                                                        }
                                                    }
                                                    // Get mrid
                                                    match obj.attribute("mRID") {
                                                        Some(id) => {
                                                            mrid = id;
                                                        }
                                                        None => {
                                                            log::error!("Unable to get mrid");
                                                        }
                                                    }
                                                    // Get label
                                                    match obj.attribute("label") {
                                                        Some(s) => {
                                                            label = s;
                                                        }
                                                        None => {
                                                            log::error!("Unable to get label");
                                                        }
                                                    }
                                                },
                                                None => log::error!("Missing Object tag"),
                                            },
                                            None => log::error!("Missing Object tag"),
                                        }

                                        println!("Type = {} [{}, {}]", asset_type, coords.0, coords.1);

                                        match asset_type {
                                            "breaker" => {
                                                mav.devices.breakers.push(Breaker {
                                                    name: label.to_string(),
                                                    mrid: mrid.to_string(),
                                                    coordinate: Coordinate {
                                                        x: coords.0,
                                                        y: coords.1,
                                                    },                                                    
                                                });
                                            },
                                            "switch" | "switch-horizontal" | "switch-vertical" => {
                                                mav.devices.switches.push(Switch {
                                                    name: label.to_string(),
                                                    mrid: mrid.to_string(),
                                                    coordinate: Coordinate {
                                                        x: coords.0,
                                                        y: coords.1,
                                                    },                                                    
                                                });
                                            },
                                            "recloser" => {
                                                mav.devices.reclosers.push(Recloser {
                                                    name: label.to_string(),
                                                    mrid: mrid.to_string(),
                                                    coordinate: Coordinate {
                                                        x: coords.0,
                                                        y: coords.1,
                                                    },                                                    
                                                });
                                            },
                                            "regulator" => {
                                                mav.devices.regulators.push(Regulator {
                                                    name: label.to_string(),
                                                    mrid: mrid.to_string(),
                                                    coordinate: Coordinate {
                                                        x: coords.0,
                                                        y: coords.1,
                                                    },
                                                    ..Default::default()                                                   
                                                });
                                            }
                                            _ => {
                                                log::error!("Can't handle asset type: {}", asset_type)
                                            }
                                        }
                                    }
                                }
                                None => {
                                    log::warn!("No mxGeometry Node for element {:?}.", elem);
                                }
                            }
                        }
                    }
                }
            }

            let serialized = serde_json::to_string(&mav).unwrap();

            println!("{}", serialized);
        }
        _ => {}
    }
}