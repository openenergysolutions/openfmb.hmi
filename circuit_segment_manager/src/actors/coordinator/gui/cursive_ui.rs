use super::super::CoordinatorMsg;
use crate::{
    actors::{microgrid::MicrogridMsg, PublisherMsg},
    messages::*,
};

use microgrid_protobuf::{
    device_control::DeviceControlMessage, microgrid_control::ControlMessage, MicrogridControl,
};

use cursive::align::HAlign;

use cursive::{
    traits::*,
    views::{Checkbox, DummyView, LinearLayout, Panel, TextView},
    Cursive,
};

use log::info;

use openfmb_ops_protobuf::openfmb::switchmodule::SwitchReadingProfile;
use riker::actors::*;

#[actor(
    StartProcessing,
    RequestActorStats,
    OpenFMBMessage,
    SwitchReadingProfile
)]
#[derive(Clone, Debug)]
pub struct CursiveUI {
    message_count: u32,
    publisher: ActorRef<PublisherMsg>,
    microgrid: ActorRef<MicrogridMsg>, //openfmb_file_subscriber: Option<OpenFMBFileSubscriber>,
}

impl ActorFactoryArgs<(ActorRef<PublisherMsg>, ActorRef<MicrogridMsg>)> for CursiveUI {
    fn create_args(args: (ActorRef<PublisherMsg>, ActorRef<MicrogridMsg>)) -> Self {
        CursiveUI {
            message_count: 0,
            publisher: args.0,
            microgrid: args.1,
        }
    }
}

impl Actor for CursiveUI {
    type Msg = CursiveUIMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        //Load the app name from config/app.toml

        let microgrid1 = self.microgrid.clone();
        let microgrid2 = self.microgrid.clone();
        let microgrid3 = self.microgrid.clone();
        let microgrid4 = self.microgrid.clone();
        let microgrid5 = self.microgrid.clone();
        let microgrid6 = self.microgrid.clone();
        let microgrid7 = self.microgrid.clone();
        let microgrid8 = self.microgrid.clone();
        let microgrid9 = self.microgrid.clone();
        let microgrid10 = self.microgrid.clone();
        let microgrid11 = self.microgrid.clone();
        let microgrid12 = self.microgrid.clone();
        let microgrid13 = self.microgrid.clone();
        let microgrid14 = self.microgrid.clone();
        let _microgrid15 = self.microgrid.clone();
        let _microgrid16 = self.microgrid.clone();
        let microgrid17 = self.microgrid.clone();
        let microgrid18 = self.microgrid.clone();
        let microgrid19 = self.microgrid.clone();
        let microgrid20 = self.microgrid.clone();
        let microgrid21 = self.microgrid.clone();
        let microgrid22 = self.microgrid.clone();
        let microgrid23 = self.microgrid.clone();
        let microgrid24 = self.microgrid.clone();
        let microgrid25 = self.microgrid.clone();
        let microgrid26 = self.microgrid.clone();
        let microgrid27 = self.microgrid.clone();
        let microgrid28 = self.microgrid.clone();
        let microgrid29 = self.microgrid.clone();
        let mut siv = Cursive::default();

        //siv.add_global_callback('q', |s| s.quit());

        siv.add_global_callback('c', move |_s| {
            microgrid1.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::InitiateGridConnect("".to_string())),
                },
                None,
            );
            info!("reconnecting");
        });
        siv.add_global_callback('i', move |_s| {
            microgrid2.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::InitiateIsland("".to_string())),
                },
                None,
            );

            info!("disconnecting");
        });
        siv.add_global_callback('r', move |_s| {
            microgrid5.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::ResetDevices("".to_string())),
                },
                None,
            );
            info!("resetting microgrid");
        });
        siv.add_global_callback('-', move |_s| {
            microgrid13.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::EnableNetZero("".to_string())),
                },
                None,
            );
            info!("enable net0");
        });
        siv.add_global_callback('=', move |_s| {
            microgrid14.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::DisableNetZero("".to_string())),
                },
                None,
            );
            info!("disable battery");
        });

        siv.add_global_callback('1', move |_s| {
            microgrid17.tell(DeviceControlMessage::EnableSolarInverter, None);
            info!("Test Solar On");
        });
        siv.add_global_callback('2', move |_s| {
            microgrid18.tell(DeviceControlMessage::DisableSolarInverter, None);
            info!("Test Solar Off");
        });
        siv.add_global_callback('5', move |_s| {
            microgrid21.tell(DeviceControlMessage::EnableLoadbank, None);
            info!("Test Loadbank On");
        });
        siv.add_global_callback('6', move |_s| {
            microgrid22.tell(DeviceControlMessage::DisableLoadbank, None);
            info!("Test Loadbank Off");
        });
        siv.add_global_callback('3', move |_s| {
            microgrid19.tell(DeviceControlMessage::GeneratorOn, None);
            info!("Test Turbine On");
        });
        siv.add_global_callback('4', move |_s| {
            microgrid20.tell(DeviceControlMessage::GeneratorOff, None);
            info!("Test Turbine Off");
        });

        siv.add_global_callback('7', move |_s| {
            microgrid23.tell(DeviceControlMessage::EssStart, None);
            info!("Test ESS Start");
        });
        siv.add_global_callback('8', move |_s| {
            microgrid24.tell(DeviceControlMessage::EssDischarge, None);
            info!("Test ESS Discharge");
        });
        siv.add_global_callback('9', move |_s| {
            microgrid25.tell(DeviceControlMessage::EssSocManage, None);
            info!("Test ESS SOC Manage");
        });
        siv.add_global_callback('0', move |_s| {
            microgrid12.tell(DeviceControlMessage::EssSocLimits, None);
            info!("Test ESS SOC Limits");
        });
        siv.add_global_callback('`', move |_s| {
            microgrid26.tell(DeviceControlMessage::EssStop, None);
            info!("Test ESS Stop");
        });

        siv.add_global_callback('a', move |_s| {
            microgrid3.tell(DeviceControlMessage::SwitchOneClosed, None);
            info!("Closing Switch One");
        });
        siv.add_global_callback('s', move |_s| {
            microgrid4.tell(DeviceControlMessage::SwitchOneOpen, None);
            info!("Opening Switch One");
        });
        siv.add_global_callback('d', move |_s| {
            microgrid6.tell(DeviceControlMessage::SwitchTwoClosed, None);
            info!("Closing Switch Two");
        });
        siv.add_global_callback('f', move |_s| {
            microgrid7.tell(DeviceControlMessage::SwitchTwoOpen, None);
            info!("Opening Switch Two");
        });
        siv.add_global_callback('g', move |_s| {
            microgrid8.tell(DeviceControlMessage::BreakerThreeClosed, None);
            info!("Closing Breaker Three");
        });
        siv.add_global_callback('h', move |_s| {
            microgrid9.tell(DeviceControlMessage::BreakerThreeOpen, None);
            info!("Opening Breaker Three");
        });
        siv.add_global_callback('j', move |_s| {
            microgrid10.tell(DeviceControlMessage::SwitchFourClosed, None);
            info!("Closing Switch Four");
        });
        siv.add_global_callback('k', move |_s| {
            microgrid11.tell(DeviceControlMessage::SwitchFourOpen, None);
            info!("Opening Switch Four");
        });
        siv.add_global_callback('y', move |_s| {
            microgrid27.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::ReconnectPretestOne("".to_string())),
                },
                None,
            );
            info!("Reconnect Pretest One");
        });
        siv.add_global_callback('u', move |_s| {
            microgrid28.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::ReconnectPretestTwo("".to_string())),
                },
                None,
            );
            info!("Reconnect Pretest Two");
        });
        siv.add_global_callback('p', move |_s| {
            microgrid29.tell(
                MicrogridControl {
                    control_message: Some(ControlMessage::ReconnectTest("".to_string())),
                },
                None,
            );
            info!("Reconnect Test");
        });
        // siv.add_global_callback('g', move |_s| {
        //     microgrid8.tell(MicrogridControlMessage::GeneratorOn, None);
        //     info!("generator on");
        // });
        // siv.add_global_callback('h', move |_s| {
        //     microgrid9.tell(MicrogridControlMessage::GeneratorOff, None);
        //     info!("generator off");
        // });
        // siv.add_global_callback('j', move |_s| {
        //     microgrid10.tell(MicrogridControlMessage::GeneratorDisabled, None);
        //     info!("disabling generator");
        // });
        // siv.add_global_callback('b', move |_s| {
        //     microgrid11.tell(MicrogridControlMessage::EnableBattery, None);
        //     info!("enable battery");
        // });
        // siv.add_global_callback('n', move |_s| {
        //     microgrid12.tell(MicrogridControlMessage::DisableBattery, None);
        //     info!("disable battery");
        // });

        // siv.add_global_callback('t', move |_s| {
        //     microgrid15.tell(MicrogridControlMessage::ForceEssDischarge, None);
        //     info!("FORCE ESS DISCHARGE");
        // });
        // siv.add_global_callback('y', move |_s| {
        //     microgrid16.tell(MicrogridControlMessage::ForceEssCharge, None);
        //     info!("FORCE ESS CHARGE");
        // });

        let text = "\
        r: Reset\t\
        c: Grid Connect\n\
        i: Island\n\
        -: Algo Enabled\t\
        =: Algo Disabled\n\n\
        y: reconnect pre test 1\t\
        u: reconnect pre test 2\n\
        p: reconnect test\t\
        1: solar_on  \t\
        2: solar_off\n\
        3: turbine_on\t\
        4: turbine_off\n\
        5: loadbank_on\t\
        6: loadbank_off\n\
        7: ess_start   \t\
        8: ess_discharge\n\
        9: ess_soc_mng\t\
        0: ess_soc_lim\n\
        `: ess_stop\n\
        a: way1_closed\t\
        s: way1_open\n\
        d: way2_closed\t\
        f: way2_open\n\
        g: breaker3_closed\t\
        h: breaker3_open\n\
        j: way4_closed\t\
        k: way4_open\n\
        ";

        // s: Solar Inverter Enabled\n\
        // d: Solar Inverter Disabled\n\
        // l: Loadbank On\n\
        // ;: Loadbank Off\n\
        // g: Turbine On\n\
        // h: Turbine Off\n\
        // j: Turbine Disabled\n\
        // b: Battery Enabled\n\
        // n: Battery Disabled\n\
        // t: Force ESSDischargeMode\n\
        // y: Force ESSChargeMode\n\

        let title =
            TextView::new("Microgrid Coordination Service                ").h_align(HAlign::Center);
        let help_view = TextView::new(text);

        // let way1 = LinearLayout::new(Orientation::Horizontal)
        //     .child(TextView::new("way1"))
        //     .child(Checkbox::new().with_name("way1"));
        // let way2 = LinearLayout::new(Orientation::Horizontal)
        //     .child(TextView::new("way2"))
        //     .child(Checkbox::new());
        // let way3 = LinearLayout::new(Orientation::Horizontal)
        //     .child(TextView::new("way3"))
        //     .child(Checkbox::new());
        // let way4 = LinearLayout::new(Orientation::Horizontal)
        //     .child(TextView::new("way4"))
        //     .child(Checkbox::new());
        //
        // let controls = LinearLayout::vertical().child(way1).child(way2).child(way3).child(way4);
        //
        // let body = LinearLayout::horizontal().child(help_view).child(controls);
        let body = LinearLayout::horizontal().child(help_view);

        siv.add_layer(Panel::new(
            LinearLayout::vertical()
                .child(title)
                // Use a DummyView as spacer
                .child(DummyView.fixed_height(1))
                // Disabling scrollable means the view cannot shrink.
                .child(body), //   .child(Dialog::around(LinearLayout::vertical()).button("Quit", |s| s.quit())),
        ));

        siv.add_layer(TextView::new(""));

        siv.call_on_name("way1", |view: &mut Checkbox| view.check());

        siv.run();
    }

    fn post_stop(&mut self) {}

    fn supervisor_strategy(&self) -> Strategy {
        Strategy::Restart
    }

    fn sys_recv(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: SystemMsg,
        _sender: Option<BasicActorRef>,
    ) {
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl Receive<StartProcessing> for CursiveUI {
    type Msg = CursiveUIMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: StartProcessing, _sender: Sender) {}
}

impl Receive<ActorRefWrap<PublisherMsg>> for CursiveUI {
    type Msg = CursiveUIMsg;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        msg: ActorRefWrap<PublisherMsg>,
        _sender: Sender,
    ) {
        self.publisher = msg.0;
    }
}

impl Receive<RequestActorStats> for CursiveUI {
    type Msg = CursiveUIMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
    }
}

// impl Receive<SwitchStatusProfile> for CursiveUI {
//     type Msg = CursiveUIMsg;
//     fn receive(&mut self, ctx: &Context<Self::Msg>, msg: SwitchStatusProfile, sender: Sender) {
//         match msg.device_name().unwrap().as_str() {
//             "way1" => info!("got message from way"),
//             _ => {}
//         }
//     }
// }
impl Receive<SwitchReadingProfile> for CursiveUI {
    type Msg = CursiveUIMsg;
    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: SwitchReadingProfile, _sender: Sender) {
        //warn!("got switch reading from {:?}", msg.device_name());
        //   panic!();
        // match msg.device_name().unwrap().as_str() {
        //     "way1" => info!("got message from way"),
        //     _ => {}
        // }
    }
}

impl Receive<OpenFMBMessage> for CursiveUI {
    type Msg = CursiveUIMsg;
    fn receive(&mut self, ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        match msg {
            OpenFMBMessage::SwitchReading(switch_reading) => ctx
                .myself
                .send_msg(switch_reading.as_ref().clone().into(), None),
            _ => {}
        }
    }
}
