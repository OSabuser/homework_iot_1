use super::{SmartContainerManagementStatus, ErrorReason};
use crate::containers::room::Room;
/// Smart house
///
///
///
use std::collections::HashMap;

//TODO: SmartContainerManagementStatus -> Result<>

/// Тип, описывающий дом
pub struct House {
    /// Название дома
    pub name: String,

    /// Список комнат в доме
    rooms: HashMap<String, Room>,

    /// Максимальное количество комнат в доме
    room_limit: usize,
}

impl House {
    /// Создание нового дома с именем `name`, в котором может быть не более `limit` комнат
    ///
    /// ## Пример
    /// ```ignore
    /// let my_house = House::new("MyHouse");
    /// ```
    pub fn new(name: &str, limit: usize) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::with_capacity(limit),
            room_limit: limit,
        }
    }

    /// Создание новой пустой комнаты в доме
    pub fn create_new_empty_room(
        &mut self,
        room_name: &str,
        device_limit: usize,
    ) -> SmartContainerManagementStatus {
        if self.rooms.len() >= self.room_limit {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemLimitExceeded);
        }

        if self.rooms.contains_key(room_name) {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemAlreadyPresented);
        }

        let new_room = Room::new(room_name, device_limit);
        self.add_room(new_room)
    }

    /// Добавление комнаты в дом
    pub fn add_room(&mut self, room: Room) -> SmartContainerManagementStatus {
        if self.rooms.len() >= self.room_limit {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemLimitExceeded);
        }

        if self.rooms.contains_key(&room.name) {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemAlreadyPresented);
        }

        self.rooms.insert(room.name.to_string(), room);
        SmartContainerManagementStatus::OperationSucceded
    }

    /// Удаление комнаты из дома
    pub fn remove_room_by_name(&mut self, room_name: &str) -> SmartContainerManagementStatus {
        if !self.rooms.contains_key(room_name) {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemDoesntExist);
        }

        self.rooms.remove(room_name);
        SmartContainerManagementStatus::OperationSucceded
    }

    /// Получение комнаты по имени
    pub fn get_room(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(room_name)
    }

    /// Получение списка комнат в доме
    pub fn get_room_list(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }

    /// Создание отчёта для в соответствии с типом поставщика данных
    pub fn create_report(&self) -> String {
        let mut report: Vec<String> = Vec::new();

        report.push(format!("Smart house instance: {}.\n", self.name));

        if self.rooms.is_empty() {
            report.push("There are no registered rooms and devices.\n".to_string());
            return report.join("\n");
        }

        for (room_name, room) in self.rooms.iter() {
            report.push(format!("Room: {}\n", room_name));

            if room.get_device_list().is_empty() {
                report.push("There are no devices in this room".to_string());
            } else {
                for device_name in room.get_device_list() {
                    report.push(format!("Device: {}: ", device_name));

                    // Получение текстового отчёта о состоянии умного устройства
                    match room.get_device(&device_name) {
                        Some(device) => {
                            report.push(device.get_text_report());
                        }
                        None => {
                            report.push("Connection was refused!".to_string());
                        }
                    }
                }
                report.push("\n".to_string());
            }
        }
        report.join(" ")
    }
}
