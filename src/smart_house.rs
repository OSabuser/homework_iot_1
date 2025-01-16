//! Модуль, содержащий реализацию типа-контейнера "Умный дом"
//!
//! > Умный дом представляет собой контейнер для хранения информации о помещениях и устройствах, зарегистрированных в них.
//! > В доме можно добавлять и удалять помещения, а также привязывать и отвязывать устройства к конкретным помещениям. При этом
//! > имеются ограничения на максимальное число помещений в доме и устройств в каждом помещении. Имена помещений и устройств должны быть уникальными.

use super::info_providers::{SmartDeviceInfoProvider, SmartDeviceRegistrationState};
use std::collections::HashMap;

///
/// Тип описывающий характеристики "Умного дома"
///
pub struct SmartHouse {
    /// Пользовательский псевдоним для дома
    pub name: String,

    /// Список комнат (хэш таблица, где key - уникальное имя комнаты, value - конкретный экземпляр комнаты с именем key)
    smart_rooms: HashMap<String, Vec<String>>,

    /// Максимальное допустимое число комнат в доме
    room_limit: usize,

    /// Максимальное допустимое число устройств в комнате
    device_limit: usize,
}

/// Результат set/get операций с содержимым умного дома
pub enum SmartHouseManagementStatus {
    /// Успех
    OperationSucceded,

    /// Ошибки set/get
    OperationFailed(ErrorReason),
}

/// Перечисление возможных ошибок set/get операций с содержимым умного дома
pub enum ErrorReason {
    /// Количество комнат в доме превышает допустимое значение
    RoomLimitExceeded,

    /// Количество устройств в комнате превышает допустимое значение
    DeviceLimitExceeded,

    /// В доме уже существует комната с таким именем
    RoomAlreadyPresented,

    /// В комнате уже существует устройство с таким именем
    DeviceAlreadyPresented,

    /// Комната с таким именем не существует
    RoomDoesntExist,

    /// Устройства с таким именем не существует
    DeviceDoesntExist,
}

impl SmartHouse {
    /// Создание экземпляра умного дома с псевдонимом `name`
    ///
    /// По умолчанию лимит комнат - *5*, помещений в каждой комнате- *10*
    ///
    /// ## Пример
    /// ```ignore
    /// let my_house = SmartHouse::new("CountryHouse");
    /// ```
    ///
    pub fn new(smart_house_name: &str) -> Self {
        Self {
            name: String::from(smart_house_name),
            smart_rooms: HashMap::with_capacity(5),
            room_limit: 5,
            device_limit: 10,
        }
    }

    /// Вывод текстовой информации о состоянии дома
    ///
    /// Выводятся имена комнат и зарегистрированных в них устройствах
    pub fn get_smart_house_status(&self) -> String {
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
    pub fn add_room(&mut self, name: &str) -> SmartHouseManagementStatus {
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
    pub fn delete_room(&mut self, name: &str) -> SmartHouseManagementStatus {
        match self.smart_rooms.remove_entry(name) {
            Some(_) => SmartHouseManagementStatus::OperationSucceded,
            None => SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomDoesntExist),
        }
    }

    /// Привязка умного устройства к комнате
    pub fn link_device_with_room(
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
    pub fn unlink_device_from_room(
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

    /// Проверка существования комнаты с именем name в доме
    pub fn is_room_already_exist(&self, name: &str) -> bool {
        self.smart_rooms.contains_key(name)
    }

    /// Проверка сущестования устройства с именем device_name в комнате с именем room_name
    pub fn is_device_presented_in_room(&self, room_name: &str, device_name: &str) -> bool {
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

    /// Создание отчёта для в соответствии с типом поставщика данных
    pub fn create_report(&self, requested_order: &dyn SmartDeviceInfoProvider) -> String {
        let mut report: Vec<String> = Vec::new();

        // Получение списка добавленных комнат
        if let Some(registered_rooms) = self.get_rooms() {
            let desired_devices = requested_order.get_device_names();

            let mut has_device_been_found = false;

            // Перебор всех комнат в умном доме
            'device_enumeration: for device in &desired_devices {
                'rooms_enumeration: for room in &registered_rooms {
                    match requested_order.get_device_status_info(self, room, device) {
                        SmartDeviceRegistrationState::Registered(x) => {
                            has_device_been_found = true;
                            report.push(x);
                            break 'rooms_enumeration;
                        }
                        SmartDeviceRegistrationState::NotFound(_) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn house_io_test() -> () {
        let mut my_house = SmartHouse::new("CountryHouse");

        let is_room_added = match my_house.add_room("LivingRoom") {
            SmartHouseManagementStatus::OperationSucceded => true,
            _ => false,
        };

        assert_eq!(is_room_added, true, "Room must be added!");

        let is_device_linked = match my_house.link_device_with_room("LivingRoom", "SmartSocket_1") {
            SmartHouseManagementStatus::OperationSucceded => true,
            _ => false,
        };

        assert_eq!(is_device_linked, true, "Device must be linked!");

        let is_device_unlinked =
            match my_house.unlink_device_from_room("LivingRoom", "SmartSocket_1") {
                SmartHouseManagementStatus::OperationSucceded => true,
                _ => false,
            };

        assert_eq!(is_device_unlinked, true, "Device must be unlinked!");

        let is_room_deleted = match my_house.delete_room("LivingRoom") {
            SmartHouseManagementStatus::OperationSucceded => true,
            _ => false,
        };

        assert_eq!(is_room_deleted, true, "Room must be deleted!");
    }

    #[test]
    fn house_overflow_test() -> () {
        let mut my_house = SmartHouse::new("CountryHouse");

        // Регистрация максимально возможного числа комнат
        for i in 0..=4 {
            let is_room_added = match my_house.add_room(&format!("LivingRoom_{}", i)) {
                SmartHouseManagementStatus::OperationSucceded => true,
                _ => false,
            };

            assert_eq!(
                is_room_added, true,
                "Room must be added! Corrupted index is:{}",
                i
            );
        }

        // Попытка добавить комнату, когда лимит превышен
        let is_adding_failed = match my_house.add_room("LivingRoom_5") {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::RoomLimitExceeded) => true,
            _ => false,
        };

        assert_eq!(is_adding_failed, true, "Room must not be added! ");

        // Регистрация максимально возможного числа устройств в произвольной комнате
        for i in 0..=9 {
            let is_device_linked = match my_house
                .link_device_with_room("LivingRoom_0", &format!("SmartSocket_{}", i))
            {
                SmartHouseManagementStatus::OperationSucceded => true,
                _ => false,
            };

            assert_ne!(
                is_device_linked, false,
                "Device must be linked! Corrupted index is:{}",
                i
            );
        }

        // Попытка добавить устройство, когда лимит превышен
        let is_adding_failed = match my_house
            .link_device_with_room("LivingRoom_0", "SmartSocket_10")
        {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::DeviceLimitExceeded) => true,
            _ => false,
        };

        assert_eq!(is_adding_failed, true, "Device must not be linked!");
    }

    #[test]
    fn house_similar_name_test() {
        let mut my_house = SmartHouse::new("CountryHouse");

        let is_room_added = match my_house.add_room("LivingRoom") {
            SmartHouseManagementStatus::OperationSucceded => true,
            _ => false,
        };
        assert_eq!(is_room_added, true, "Room must be added!");

        let is_room_added = match my_house.add_room("LivingRoom") {
            SmartHouseManagementStatus::OperationSucceded => true,
            _ => false,
        };
        assert_eq!(is_room_added, false, "Room must not be added!");

        let is_device_linked = match my_house.link_device_with_room("LivingRoom", "SmartSocket_1") {
            SmartHouseManagementStatus::OperationSucceded => true,
            _ => false,
        };
        assert_eq!(is_device_linked, true, "Device must be linked!");

        let is_device_linked = match my_house.link_device_with_room("LivingRoom", "SmartSocket_1") {
            SmartHouseManagementStatus::OperationFailed(ErrorReason::DeviceAlreadyPresented) => {
                true
            }
            _ => false,
        };

        assert_eq!(is_device_linked, true, "Device must not be linked!");
    }
}
