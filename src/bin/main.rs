extern crate iot_home;
use iot_home::{SmartPlug, SmartPlugPowerState, SmartThermometer};
fn main() {
    let mut my_smart_socket = SmartPlug::new("MySocket #1");
    my_smart_socket.set_power_state(SmartPlugPowerState::Enabled);
    my_smart_socket.get_status();

    let my_smart_thermometer = SmartThermometer::new();
    println!("Current temperature is: {}", my_smart_thermometer._get_current_temperature());
}