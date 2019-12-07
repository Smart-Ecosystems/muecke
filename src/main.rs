extern crate mosquitto_client;

use mosquitto_client::Mosquitto;
use std::env;
use std::process;

fn main() {
    let m = Mosquitto::new("bilbo_baggins_1911");
    let args = env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        println!("MQTT topic not provided.");
        process::exit(1);
    }

    let topic = args[1].clone();

    m.connect("broker.hivemq.com", 1883)
        .expect("Can't connect to the broker.");
    m.subscribe(&topic, 1)
        .expect("can't subscribe to specified topic.");

    let mut mc = m.callbacks(0);

    mc.on_message(|_, msg| {
        if !msg.retained() {
            // Only read live, non-retained messages
            println!("Received Msg {:?} on Topic {}.", msg.text(), msg.topic());
        }
    });

    m.loop_until_disconnect(200).expect("Broken loop, exiting.");
}
