/// Интеграционный тест для проверки корректности работы функции создания отчета
use iot_crate::*;

#[test]
fn test_report_validation() {
    let mut smart_house = smart_house::SmartHouse::new("House_1");

    smart_house.add_room("Room_1");
    smart_house.add_room("Room_2");

    let my_socket_1 = socket::SmartSocket::new("Socket_1");
    let my_socket_2 = socket::SmartSocket::new("Socket_2");
    let my_thermometer_1 = thermometer::SmartThermometer::new("Thermometer_1");
    let my_thermometer_2 = thermometer::SmartThermometer::new("Thermometer_2");

    smart_house.link_device_with_room("Room_1", &my_socket_1.name);
    smart_house.link_device_with_room("Room_2", &my_socket_2.name);
    smart_house.link_device_with_room("Room_2", &my_thermometer_2.name);

    let custom_order_1 = info_providers::template_1::SocketReport::new(&my_socket_1, &my_socket_2);

    let custom_order_2 = info_providers::template_2::MixedDevicesReport::new(
        &my_thermometer_1,
        &my_thermometer_2,
        &my_socket_1,
    );

    let report_1 = smart_house.create_report(&custom_order_1);
    let report_2 = smart_house.create_report(&custom_order_2);

    let use_case = report_1.contains("Socket_1 is located in Room_1");
    assert_eq!(
        use_case, true,
        "Expected report_1 to contain 'Socket_1 is located in Room_1'"
    );

    let use_case = report_1.contains("Socket_2 is located in Room_2");
    assert_eq!(
        use_case, true,
        "Expected report_1 to contain 'Socket_2 is located in Room_2'"
    );

    let use_case = report_2.contains("It seems Thermometer_1 isn't registered in House_1");
    assert_eq!(
        use_case, true,
        "Expected report_2 to contain 'It seems Thermometer_1 isn't registered in House_1'"
    );

    let use_case = report_2.contains("Thermometer_2 is located in Room_2");
    assert_eq!(
        use_case, true,
        "Expected report_2 to contain 'Thermometer_2 is located in Room_2'"
    );

    let use_case = report_2.contains("Socket_1 is located in Room_1");
    assert_eq!(
        use_case, true,
        "Expected report_2 to contain 'Socket_1 is located in Room_1'"
    );
}
