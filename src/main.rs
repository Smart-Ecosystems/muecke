extern crate mosquitto_client as mosq;

use mosq::Mosquitto;


fn main() {
    let m = Mosquitto::new("test");

    m.connect("broker.hivemq.com", 1883).expect("can't connect");
    m.subscribe("bilbo/1", 1).expect("can't subscribe to bonzo");

    let mut mc = m.callbacks(0);
    mc.on_message(|data, msg| {
        println!("Topic: {}, Msg: {:?}", msg.topic(), msg.text());
        *data += 1;
    });

    m.loop_until_disconnect(200).expect("broken loop");
    println!("received {} messages", mc.data);
}

