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
    if socket1
        .set_power_state(SmartDevicePowerState::Enabled)
        .is_ok()
    {
        println!("Socket1 is enabled");
    }

    // Создание комнат (Первый способ)
    let mut living_room = Room::new("LivingRoom", 7);
    let mut kitchen = Room::new("Kitchen", 5);

    // Добавление умных девайсов в комнаты
    if living_room.add_device(Box::new(socket1)).is_err() {
        println!("Failed to add Socket1 to a {}!", living_room.name);
    }
    if living_room.add_device(Box::new(thermometer1)).is_err() {
        println!("Failed to add Thermometer1 to a {}!", living_room.name);
    }
    if kitchen.add_device(Box::new(socket2)).is_err() {
        println!("Failed to add Socket2 to a {}!", kitchen.name);
    }

    if kitchen.add_device(Box::new(thermometer2)).is_err() {
        println!("Failed to add Thermometer2 to a {}!", kitchen.name);
    }

    // Добавление комнат в дом
    if my_house.add_room(living_room).is_err() {
        println!("Failed to add a LivingRoom to a {}!", my_house.name);
    }

    if my_house.add_room(kitchen).is_err() {
        println!("Failed to add a Kitchen to a {}!", my_house.name);
    }

    // Создание комнат (Второй способ)
    if my_house.create_new_empty_room("Bedroom", 7).is_err() {
        println!("Failed to create a new room!");
    } else {
        // Получение ссылки на инстанс комнаты
        if let Some(room) = my_house.get_room("Bedroom") {
            // Добавление умных девайсов в комнату
            if room.add_device(Box::new(socket3)).is_err() {
                println!("Failed to add Socket3 to a {}!", room.name);
            }

            if room.add_device(Box::new(thermometer3)).is_err() {
                println!("Failed to add Thermometer3 to a {}!", room.name);
            }
        } else {
            println!("Failed to get a room!");
        }
    }

    // Создание отчета о состоянии дома
    let report = my_house.create_report();
    println!("{}", report);
}
