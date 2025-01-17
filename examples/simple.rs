
/// # Пример использования библиотеки для создания умного дома
/// 1. Показаны два способа регистрации комнат в доме
/// 2. Приведён пример добавления умных девайсов в комнаты
/// 3. Показан пример управления подачей питания к умному устройству
/// 4. Пример создания отчета о состоянии дома
use iot_crate::house::House;
use iot_crate::room::Room;
use iot_crate::smart_device::{SmartDevice, SmartDevicePowerState};
use iot_crate::socket::SmartSocket;
use iot_crate::thermometer::SmartThermometer;
use iot_crate::containers::SmartContainerManagementStatus;

fn main() {
    // Создание инстанса умного дома
    let mut my_house = House::new("MyLoungeHouse", 10);

    //  Создание инстансов умных девайсов
    let mut socket1 = SmartSocket::new("Socket1");
    let socket2 = SmartSocket::new("Socket2");
    let socket3 = SmartSocket::new("Socket3");
    let thermometer1 = SmartThermometer::new("Thermometer1");
    let thermometer2 = SmartThermometer::new("Thermometer2");
    let thermometer3 = SmartThermometer::new("Thermometer3");

    // Включение розетки
    if let Ok(()) = socket1.set_power_state(SmartDevicePowerState::Enabled) {
        println!("Socket1 is enabled");
    }

    // Создание комнат (Первый способ)
    let mut living_room = Room::new("LivingRoom", 7);
    let mut kitchen = Room::new("Kitchen", 5);

    // Добавление умных девайсов в комнаты
    living_room.add_device(Box::new(socket1));
    living_room.add_device(Box::new(thermometer1));
    kitchen.add_device(Box::new(socket2));
    kitchen.add_device(Box::new(thermometer2));


    // Добавление комнат в дом
    my_house.add_room(living_room);
    my_house.add_room(kitchen);

    // Создание комнат (Второй способ)
    if let SmartContainerManagementStatus::OperationFailed(_) =
        my_house.create_new_empty_room("Bedroom", 7)
    {
        println!("Failed to create a new room!");
    } else {
        // Получение ссылки на инстанс комнаты
        if let Some(room) = my_house.get_room("Bedroom") {
            // Добавление умных девайсов в комнату
            room.add_device(Box::new(socket3));
            room.add_device(Box::new(thermometer3));
        } else {
            println!("Failed to get a room!");
        }
    }

    // Создание отчета о состоянии дома
    let report = my_house.create_report();
    println!("{}", report);

}
