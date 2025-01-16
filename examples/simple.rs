use smart_house::SmartSocket;
use smart_house::SmartThermometer;
use smart_house::{SmartDevice, SmartDevicePowerState};

fn main() {
    let mut my_plug = SmartSocket::new("MyPreciousPlug_1");
    let my_thermometer = SmartThermometer::new("ThatFamousThing_0");

    my_plug
        .set_power_state(SmartDevicePowerState::Enabled)
        .unwrap();

    println!("{}", my_plug.get_text_report());
    println!("{}", my_thermometer.get_text_report());
}
