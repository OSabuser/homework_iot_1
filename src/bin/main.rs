//! Библиотека "Умный дом" OTUS 2024 - [2]
use std::collections::HashMap;
use std::fmt::{self, format};

fn main() {
    // Создание нового экземпляра дома
    let mut smart_house = SmartHouse::new("MyLoungeHome");

    // Создание нескольких помещений в доме
    smart_house.add_room("Room_1");
    smart_house.add_room("Room_2");
    smart_house.add_room("Room_3");

    // Вывод зарегистрированных в умном доме помещений
    if let Some(room_list) = smart_house.get_rooms() {
        println!("Зарегистрированы комнаты: {:?}", room_list);
    }

    // Создание нескольких умных девайсов
    let my_socket_1 = SmartSocket {
        name: "QuitePrettySocket".to_string(),
    };
    let my_socket_2 = SmartSocket {
        name: "ThatDamnedSocket".to_string(),
    };
    let my_socket_3 = SmartSocket {
        name: "MyFavoriteSocket".to_string(),
    };
    let my_thermometer_1 = SmartThermometer {
        name: "TheGreatThermometer".to_string(),
    };
    let my_thermometer_2 = SmartThermometer {
        name: "RegularThermometer".to_string(),
    };

    // Привязка девайсов к конкретным помещениям
    smart_house.link_device_with_room("Room_1", &my_socket_1.name);
    smart_house.link_device_with_room("Room_2", &my_socket_2.name);
    smart_house.link_device_with_room("Room_3", &my_socket_3.name);
    smart_house.link_device_with_room("Room_1", &my_thermometer_1.name);
    smart_house.link_device_with_room("Room_2", &my_thermometer_2.name);

    // Создание требований для отчёта
    let sockets_info = SmartSocketInfoProvider {
        most_wanted_socket_1: &my_socket_1,
        most_wanted_socket_2: &my_socket_3,
    };
    let thermometers_info = MyCustomInfoProvider_1 {
        thermometer_instance_1: &my_thermometer_1,
        thermometer_instance_2: &my_thermometer_2,
        smart_socket_1: &my_socket_2,
    };

    // Составление отчётов
    let report_1 = smart_house.create_report(&sockets_info);
    let report_2 = smart_house.create_report(&thermometers_info);
}

enum SmartDeviceStatus {
    Registered(String),
    NotFound(String),
}
trait SmartDeviceInfoProvider {
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceStatus;
}

// Пользовательские устройства:
struct SmartSocket {
    name: String,
}
struct SmartThermometer {
    name: String,
}

struct SmartSocketInfoProvider<'sockets_lifetime> {
    most_wanted_socket_1: &'sockets_lifetime SmartSocket,
    most_wanted_socket_2: &'sockets_lifetime SmartSocket,
}
impl SmartDeviceInfoProvider for SmartSocketInfoProvider<'_> {
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceStatus {
        if smart_house_instance.is_device_presented_in_room(room_name, device_name) {
            SmartDeviceStatus::Registered(format!(
                "Note: {} is located in {}",
                device_name, room_name
            ))
        } else {
            SmartDeviceStatus::NotFound(format!(
                "Error: {} wasn't found in {}",
                device_name, room_name
            ))
        }
    }
}

struct MyCustomInfoProvider_1<'devices_lifetime> {
    thermometer_instance_1: &'devices_lifetime SmartThermometer,
    thermometer_instance_2: &'devices_lifetime SmartThermometer,
    smart_socket_1: &'devices_lifetime SmartSocket,
}
impl SmartDeviceInfoProvider for MyCustomInfoProvider_1<'_> {
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceStatus {
        if smart_house_instance.is_device_presented_in_room(room_name, device_name) {
            SmartDeviceStatus::Registered(format!(
                "Note: {} is located in {}",
                device_name, room_name
            ))
        } else {
            SmartDeviceStatus::NotFound(format!(
                "Error: {} wasn't found in {}",
                device_name, room_name
            ))
        }
    }
}

enum SmartHouseManagementStatus {
    OperationSucceded,
    OperationFailed(ErrorReason),
}
enum ErrorReason {
    RoomLimitExceeded,
    DeviceLimitExceeded,
    RoomAlreadyPresented,
    DeviceAlreadyPresented,
    RoomDoesntExist,
    DeviceDoesntExist,
}

///
/// 1. Умный дом
/// В текущей версии может содержать до 5 комнат с различными умными устройствами
///
/// ## Параметры
///
/// * `name` - пользовательский псевдоним для дома
/// * `rooms` - список комнат (хэш таблица, где key - уникальное имя комнаты, value - конкретный экземпляр комнаты с именем key)
///
struct SmartHouse {
    name: String,
    smart_rooms: HashMap<String, Vec<String>>,
    room_limit: usize,
    device_limit: usize,
}

impl SmartHouse {
    fn new(smart_house_name: &str) -> Self {
        Self {
            name: String::from(smart_house_name),
            smart_rooms: HashMap::with_capacity(5),
            room_limit: 5,
            device_limit: 10,
        }
    }

    /// Регистрация нового помещения в доме
    fn add_room(&mut self, name: &str) -> SmartHouseManagementStatus {
        if self.smart_rooms.len() == self.room_limit {
            return SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomLimitExceeded);
        }

        if self.is_room_already_exist(name) {
            return SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomAlreadyPresented);
        } else {
            self.smart_rooms
                .insert(name.to_string(), Vec::with_capacity(10));
            return SmartHouseManagementStatus::OperationSucceded;
        }
    }

    /// Удаление помещения из дома
    fn delete_room(&mut self, name: &str) -> SmartHouseManagementStatus {
        match self.smart_rooms.remove_entry(name) {
            Some(_) => SmartHouseManagementStatus::OperationSucceded,
            None => SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist),
        }
    }

    /// Привязка умного устройства к комнате
    fn link_device_with_room(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> SmartHouseManagementStatus {
        if self.is_room_already_exist(room_name) {
            if self.is_device_presented_in_room(room_name, device_name) {
                return SmartHouseManagementStatus::OperationFailed(
                    ErrorReason::DeviceAlreadyPresented,
                );
            }

            if let Some(x) = self.smart_rooms.get_mut(room_name) {
                if x.len() == self.device_limit {
                    SmartHouseManagementStatus::OperationFailed(ErrorReason::DeviceLimitExceeded)
                } else {
                    x.push(device_name.to_string());
                    return SmartHouseManagementStatus::OperationSucceded;
                }
            } else {
                SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
            }
        } else {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
        }
    }

    /// Удаление умного устройства из комнаты
    fn unlink_device_from_room(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> SmartHouseManagementStatus {
        if self.is_room_already_exist(room_name) {
            if self.is_device_presented_in_room(room_name, device_name) {
                if let Some(x) = self.smart_rooms.get_mut(room_name) {
                    x.retain(|value| *value != device_name);
                    return SmartHouseManagementStatus::OperationSucceded;
                } else {
                    SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
                }
            } else {
                return SmartHouseManagementStatus::OperationFailed(ErrorReason::DeviceDoesntExist);
            }
        } else {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
        }
    }

    /// Проверка сущестования комнаты с именем name в текущем инстансе дома
    fn is_room_already_exist(&self, name: &str) -> bool {
        self.smart_rooms.contains_key(name)
    }

    /// Проверка сущестования устройства device_name в room_name
    fn is_device_presented_in_room(&self, room_name: &str, device_name: &str) -> bool {
        if self.is_room_already_exist(room_name) {
            if let Some(x) = self.smart_rooms.get(room_name) {
                x.contains(&device_name.to_string())
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Получение списка зарегистрированных в доме помещений
    fn get_rooms(&self) -> Option<Vec<String>> {
        if self.smart_rooms.is_empty() {
            None
        } else {
            let mut room_names = Vec::with_capacity(self.smart_rooms.len());

            for room in self.smart_rooms.keys() {
                room_names.push(room.clone());
            }

            Some(room_names)
        }
    }

    /// Получение списка зарегистрированных в помещении умных устройств
    fn get_devices(&self, room_name: &str) -> Option<Vec<String>> {
        if self.is_room_already_exist(room_name) {
            if let Some(device_list) = self.smart_rooms.get(room_name) {
                if device_list.len() > 0 {
                    let mut device_names = Vec::with_capacity(self.smart_rooms.len());

                    for device in device_list {
                        device_names.push(device.clone());
                    }

                    Some(device_names)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn create_report(&self, requested_order: &dyn SmartDeviceInfoProvider) -> String {
        if let Some(rooms_list) = self.get_rooms() {
            let mut report = String::new();
            format!("There are no created rooms in the {} !", self.name)
        } else {
            format!("There are no created rooms in the {} !", self.name)
        }
    }
}
