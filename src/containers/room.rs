use super::{SmartContainerManagementStatus, ErrorReason};
use crate::smart_device::SmartDevice;
use std::collections::HashMap;

/// Тип, описывающий комнату в доме
///
///
pub struct Room {
    /// Название комнаты
    pub name: String,

    /// Список умных устройств в комнате
    devices: HashMap<String, Box<dyn SmartDevice>>,

    /// Максимальное количество умных устройств в комнате
    device_limit: usize,
}

impl Room {
    /// Создание новой комнаты с именем `name`, в которой может быть не более `limit` умных устройств
    pub fn new(name: &str, limit: usize) -> Self {
        Self {
            name: name.to_string(),
            devices: HashMap::with_capacity(limit),
            device_limit: limit,
        }
    }

    /// Добавление умного устройства в комнату
    pub fn add_device(&mut self, device: Box<dyn SmartDevice>) -> SmartContainerManagementStatus {
        if self.devices.len() >= self.device_limit {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemLimitExceeded);
        }

        let device_name = device.get_name();

        if self.devices.contains_key(device_name) {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemAlreadyPresented);
        }

        self.devices.insert(device_name.to_string(), device);
        SmartContainerManagementStatus::OperationSucceded
    }

    /// Удаление умного устройства из комнаты
    pub fn remove_device(&mut self, device_name: &str) -> SmartContainerManagementStatus {
        if !self.devices.contains_key(device_name) {
            return SmartContainerManagementStatus::OperationFailed(ErrorReason::ItemDoesntExist);
        }

        self.devices.remove(device_name);
        SmartContainerManagementStatus::OperationSucceded
    }

    /// Получение умного устройства по имени
    pub fn get_device(&self, device_name: &str) -> Option<&Box<dyn SmartDevice>> {
        self.devices.get(device_name)
    }

    /// Получение списка умных устройств в комнате
    pub fn get_device_list(&self) -> Vec<String> {
        self.devices.keys().cloned().collect()
    }
}
