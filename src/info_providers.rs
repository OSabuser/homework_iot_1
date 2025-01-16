//! Модуль, содержащий реализацию типа-контейнера "Умный дом"
//!
//! Краткое описание сущности реализации
//!

use super::smart_house::SmartHouse;


/// Статус регистрации умного устройства в доме
pub enum SmartDeviceRegistrationState {
    /// Устройство зарегистрировано
    Registered(String),

    /// Устройство не найдено
    NotFound(String),
}


pub trait SmartDeviceInfoProvider {
    /// Получение статуса устройства
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceRegistrationState;

    /// Получение списка имён устройств, для которых необходимо составить отчёт
    fn get_device_names(&self) -> Vec<&str>;
}



pub mod two_sockets {
    //! Модуль, содержащий реализацию типа-контейнера "Умный дом"
    //!
    //! Краткое описание сущности реализации
    //!

    use crate::SmartSocket;
    use crate::smart_house::SmartHouse;
    use super::{SmartDeviceInfoProvider, SmartDeviceRegistrationState};

    pub struct SmartSocketInfoProvider<'sockets_lifetime> {
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
        ) -> SmartDeviceRegistrationState {
            if smart_house_instance.is_device_presented_in_room(room_name, device_name) {
                SmartDeviceRegistrationState::Registered(format!(
                    "{} is located in {}\n",
                    device_name, room_name
                ))
            } else {
                SmartDeviceRegistrationState::NotFound("Not found!".to_string())
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
}

pub mod mixed_devices {
    //! Модуль, содержащий реализацию типа-контейнера "Умный дом"
    //!
    //! Краткое описание сущности реализации
    //!
    //! 
    use crate::SmartSocket;
    use crate::SmartThermometer;
    use crate::smart_house::SmartHouse;
    use super::{SmartDeviceInfoProvider, SmartDeviceRegistrationState};

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
        ) -> SmartDeviceRegistrationState {
            if smart_house_instance.is_device_presented_in_room(room_name, device_name) {
                SmartDeviceRegistrationState::Registered(format!(
                    "{} is located in {}\n",
                    device_name, room_name
                ))
            } else {
                SmartDeviceRegistrationState::NotFound("Not found!".to_string())
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
}