use super::{ContainerIOHistory, ContainerName, ErrorReason};
use crate::smart_device::SmartDevice;
use std::collections::HashMap;

//TODO: SmartContainerManagementStatus -> Result<>

/// Тип, описывающий комнату в доме
///
///
pub struct Room {
    /// Название комнаты
    pub name: ContainerName,

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
    pub fn add_device(
        &mut self,
        device: Box<dyn SmartDevice>,
    ) -> Result<ContainerIOHistory, ErrorReason> {
        if self.devices.len() >= self.device_limit {
            return Err(ErrorReason::ItemLimitExceeded);
        }

        let device_name = device.get_name();

        if self.devices.contains_key(device_name) {
            return Err(ErrorReason::ItemAlreadyPresented);
        }

        let status = format!(
            "Device {} has been registered in room {}",
            device_name, self.name
        );
        self.devices.insert(device_name.to_string(), device);
        Ok(status)
    }

    /// Удаление умного устройства из комнаты
    pub fn remove_device(&mut self, device_name: &str) -> Result<ContainerIOHistory, ErrorReason> {
        if !self.devices.contains_key(device_name) {
            return Err(ErrorReason::ItemDoesntExist);
        }

        let status = format!(
            "Device {} has been removed from room {}",
            device_name, self.name
        );

        self.devices.remove(device_name);
        Ok(status)
    }

    /// Получение умного устройства по имени
    pub fn get_device(&self, device_name: &str) -> Option<&Box<dyn SmartDevice>> {
        self.devices.get(device_name)
    }

    /// Получение списка умных устройств в комнате
    pub fn get_device_list(&self) -> Vec<ContainerName> {
        self.devices.keys().cloned().collect()
    }
}
