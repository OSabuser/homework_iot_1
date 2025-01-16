/// Пример использования библиотеки для создания умного дома
/// В данном примере создаётся умный дом, в котором есть несколько помещений и умные девайсы
/// (розетки и термометры). Девайсы привязываются к помещениям, после чего с помощью шаблонов-поставщиков информации
/// создаются отчёты о состоянии дома и умных девайсов.
use iot_crate::info_providers::{template_1, template_2};
use iot_crate::smart_house::SmartHouse;
use iot_crate::socket::SmartSocket;
use iot_crate::thermometer::SmartThermometer;

fn main() {
    // Создание нового экземпляра дома
    let mut smart_house = SmartHouse::new("MyLoungeHome");

    // Создание нескольких помещений в доме
    smart_house.add_room("Room_1");
    smart_house.add_room("Room_2");
    smart_house.add_room("Room_3");

    // Создание нескольких умных девайсов
    let my_socket_1 = SmartSocket::new("QuitePrettySocket");
    let my_socket_2 = SmartSocket::new("ThatDamnedSocket");
    let my_socket_3 = SmartSocket::new("MyFavoriteSocket");

    let my_thermometer_1 = SmartThermometer::new("TheGreatThermometer");
    let my_thermometer_2 = SmartThermometer::new("UnrealThermometer");

    // Привязка девайсов к конкретным помещениям
    smart_house.link_device_with_room("Room_1", &my_socket_1.name);
    smart_house.link_device_with_room("Room_2", &my_socket_2.name);
    smart_house.link_device_with_room("Room_3", &my_socket_3.name);
    smart_house.link_device_with_room("Room_1", &my_thermometer_1.name);
    //smart_house.link_device_with_room("Room_2", &my_thermometer_2.name);

    // Заполнение шаблонов для предоставления информации

    let sockets_info = template_1::SocketReport::new(&my_socket_1, &my_socket_3);

    let thermometers_info =
        template_2::MixedDevicesReport::new(&my_thermometer_1, &my_thermometer_2, &my_socket_2);

    // Составление отчётов
    let report_1 = smart_house.create_report(&sockets_info);
    println!("Report 1:\n{}", report_1);
    let report_2 = smart_house.create_report(&thermometers_info);
    println!("Report 2:\n{}", report_2);
    let smart_home_info = smart_house.get_smart_house_status();
    println!("Smart home status:\n{}", smart_home_info);
}
