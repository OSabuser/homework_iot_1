//! Библиотека "Умный дом" OTUS 2024 - [2]
use std::collections::HashMap;

fn main() {
    // Создание нового экземпляра дома
    let mut smart_house = SmartHouse::new("MyLoungeHome");

    // Создание нескольких помещений в доме
    smart_house.add_room("Room_1");
    smart_house.add_room("Room_2");
    smart_house.add_room("Room_3");

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
        name: "UnrealThermometer".to_string(),
    };

    // Привязка девайсов к конкретным помещениям
    smart_house.link_device_with_room("Room_1", &my_socket_1.name);
    smart_house.link_device_with_room("Room_2", &my_socket_2.name);
    smart_house.link_device_with_room("Room_3", &my_socket_3.name);
    smart_house.link_device_with_room("Room_1", &my_thermometer_1.name);
    //smart_house.link_device_with_room("Room_2", &my_thermometer_2.name);

    // Создание требований для отчёта
    let sockets_info = SmartSocketInfoProvider {
        most_wanted_socket_1: &my_socket_1,
        most_wanted_socket_2: &my_socket_3,
    };
    let thermometers_info = MyCustomInfoProvider1 {
        thermometer_instance_1: &my_thermometer_1,
        thermometer_instance_2: &my_thermometer_2,
        smart_socket_1: &my_socket_2,
    };

    // Составление отчётов
    let report_1 = smart_house.create_report(&sockets_info);
    println!("Report 1:\n{}", report_1);
    let report_2 = smart_house.create_report(&thermometers_info);
    println!("Report 2:\n{}", report_2);
    let smart_home_info = smart_house.get_smart_house_status();
    println!("Smart home status:\n{}", smart_home_info);
}

#[allow(dead_code)]
enum SmartDeviceScanningStatus {
    Registered(String),
    NotFound(String),
}
trait SmartDeviceInfoProvider {
    /// Получение статуса устройства
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceScanningStatus;

    /// Получение списка имён устройств, для которых необходимо составить отчёт
    fn get_device_names(&self) -> Vec<&str>;
}

// Пользовательские устройства:
struct SmartSocket {
    name: String,
}
struct SmartThermometer {
    name: String,
}

// Пользовательские поставщики информации об устройствах
struct SmartSocketInfoProvider<'sockets_lifetime> {
    most_wanted_socket_1: &'sockets_lifetime SmartSocket,
    most_wanted_socket_2: &'sockets_lifetime SmartSocket,
}
impl SmartSocketInfoProvider<'_> {
    fn get_smart_device_names(&self) -> [&str; 2] {
        [
            &self.most_wanted_socket_1.name,
            &self.most_wanted_socket_2.name,
        ]
    }
}
impl SmartDeviceInfoProvider for SmartSocketInfoProvider<'_> {
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceScanningStatus {
        if smart_house_instance.is_device_presented_in_room(room_name, device_name) {
            SmartDeviceScanningStatus::Registered(format!(
                "{} is located in {}\n",
                device_name, room_name
            ))
        } else {
            SmartDeviceScanningStatus::NotFound("Not found!".to_string())
        }
    }
    fn get_device_names(&self) -> Vec<&str> {
        let mut device_names = Vec::with_capacity(self.get_smart_device_names().len());

        for device_name in self.get_smart_device_names() {
            device_names.push(device_name);
        }
        device_names
    }
}

struct MyCustomInfoProvider1<'devices_lifetime> {
    thermometer_instance_1: &'devices_lifetime SmartThermometer,
    thermometer_instance_2: &'devices_lifetime SmartThermometer,
    smart_socket_1: &'devices_lifetime SmartSocket,
}
impl MyCustomInfoProvider1<'_> {
    fn get_smart_device_names(&self) -> [&str; 3] {
        [
            &self.thermometer_instance_1.name,
            &self.thermometer_instance_2.name,
            &self.smart_socket_1.name,
        ]
    }
}
impl SmartDeviceInfoProvider for MyCustomInfoProvider1<'_> {
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceScanningStatus {
        if smart_house_instance.is_device_presented_in_room(room_name, device_name) {
            SmartDeviceScanningStatus::Registered(format!(
                "{} is located in {}\n",
                device_name, room_name
            ))
        } else {
            SmartDeviceScanningStatus::NotFound("Not found!".to_string())
        }
    }

    fn get_device_names(&self) -> Vec<&str> {
        let mut device_names = Vec::with_capacity(self.get_smart_device_names().len());

        for device_name in self.get_smart_device_names() {
            device_names.push(device_name);
        }
        device_names
    }
}

#[allow(dead_code)]
enum SmartHouseManagementStatus {
    OperationSucceded,
    OperationFailed(ErrorReason),
}
#[allow(dead_code)]
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
///     В текущей версии может содержать до 5 комнат с 10 умными устройствами в каждой
///
/// ## Параметры
///
/// * `name` - пользовательский псевдоним для дома
/// * `smart_rooms` - список комнат (хэш таблица, где key - уникальное имя комнаты, value - конкретный экземпляр комнаты с именем key)
/// * `room_limit` - максимальное допустимое число комнат в доме
/// * `device_limit` - максимальное допустимое число умных устройств в комнате 
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

    /// Вывод текстовой информации о состоянии дома
    fn get_smart_house_status(&self) -> String {
        let mut report: Vec<String> = Vec::new();

        if let Some(room_list) = self.get_rooms() {
            for room in room_list {
                if let Some(devices_list) = self.get_devices(&room) {
                    report.push(format!(
                        "К комнате {} привязаны устройства: {}\n",
                        room,
                        devices_list.join(", ")
                    ));
                } else {
                    report.push(format!(
                        "С комнатой {} не связано ни одно устройство!\n",
                        room
                    ));
                }
            }
        } else {
            report.push(format!(
                "В доме {} не зарегистрировано ни одной комнаты!\n",
                self.name
            ));
        }
        report.concat()
    }

    /// Регистрация нового помещения в доме
    fn add_room(&mut self, name: &str) -> SmartHouseManagementStatus {
        if self.smart_rooms.len() == self.room_limit {
            return SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomLimitExceeded);
        }

        if self.is_room_already_exist(name) {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomAlreadyPresented)
        } else {
            self.smart_rooms
                .insert(name.to_string(), Vec::with_capacity(10));
            SmartHouseManagementStatus::OperationSucceded
        }
    }

    /// Удаление помещения из дома
    fn _delete_room(&mut self, name: &str) -> SmartHouseManagementStatus {
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
                    SmartHouseManagementStatus::OperationSucceded
                }
            } else {
                SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
            }
        } else {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
        }
    }

    /// Удаление умного устройства из комнаты
    fn _unlink_device_from_room(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> SmartHouseManagementStatus {
        if self.is_room_already_exist(room_name) {
            if self.is_device_presented_in_room(room_name, device_name) {
                if let Some(x) = self.smart_rooms.get_mut(room_name) {
                    x.retain(|value| *value != device_name);
                    SmartHouseManagementStatus::OperationSucceded
                } else {
                    SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist)
                }
            } else {
                SmartHouseManagementStatus::OperationFailed(ErrorReason::DeviceDoesntExist)
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
                if !device_list.is_empty() {
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

    /// Создание отчёта для конкретного поставщика информации
    fn create_report(&self, requested_order: &dyn SmartDeviceInfoProvider) -> String {
        let mut report: Vec<String> = Vec::new();

        // Получение списка добавленных комнат
        if let Some(registered_rooms) = self.get_rooms() {
            let desired_devices = requested_order.get_device_names();

            let mut has_device_been_found = false;

            // Перебор всех комнат в умном доме
            'device_enumeration: for device in &desired_devices {
                'rooms_enumeration: for room in &registered_rooms {
                    match requested_order.get_device_status_info(self, room, device) {
                        SmartDeviceScanningStatus::Registered(x) => {
                            has_device_been_found = true;
                            report.push(x);
                            break 'rooms_enumeration;
                        }
                        SmartDeviceScanningStatus::NotFound(_) => {
                            has_device_been_found = false;
                        }
                    }
                }

                if !has_device_been_found {
                    report.push(format!(
                        "It seems {} isn't registered in {}\n",
                        device, self.name
                    ));
                    continue 'device_enumeration;
                }
                has_device_been_found = false;
            }

            report.concat()
        } else {
            report.push(format!(
                "You should initialize the {} properly!\n",
                self.name
            ));
            report.concat()
        }
    }
}
