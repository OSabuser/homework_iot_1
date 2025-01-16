//! Модуль, описывающий сущности, отвечающие за предоставление информации о умных устройствах
//!
//! В данном модуле описаны:
//! - Два пользовательских шаблона для отчётов о состоянии умных устройств в доме
//! - Trait SmartDeviceInfoProvider, предоставляющий методы для получения информации о умных устройствах
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
    /// Получение статуса регистрации и работы устройства в smart_house_instance
    fn get_device_status_info(
        &self,
        smart_house_instance: &SmartHouse,
        room_name: &str,
        device_name: &str,
    ) -> SmartDeviceRegistrationState;

    /// Получение списка имён устройств, для которых необходимо составить отчёт
    fn get_device_names(&self) -> Vec<&str>;
}

//TODO:- сделать общий тип для всех отчётов, содержащий в себе Vec<Box<dyn SmartDevice>>
pub mod template_1 {
    //! Шаблон для отчёта о состоянии двух умных розеток с заданными именами
    //!
    use super::SmartHouse;
    use super::{SmartDeviceInfoProvider, SmartDeviceRegistrationState};
    use crate::devices::socket::SmartSocket;

    /// Тип представляющий собой форму отчёта о состоянии двух умных розеток
    ///
    /// Должен быть проинициализирован ссылками на два, интересующих пользователя, умных устройства типа SmartSocket
    pub struct SocketReport<'sockets_lifetime> {
        most_wanted_socket_1: &'sockets_lifetime SmartSocket,
        most_wanted_socket_2: &'sockets_lifetime SmartSocket,
    }

    impl<'sockets_lifetime> SocketReport<'sockets_lifetime> {
        /// Создание пользовательской формы отчёта о состоянии двух умных розеток
        ///
        /// ## Пример
        /// ```ignore
        /// let my_socket_1 = SmartSocket::new("QuitePrettySocket");
        /// let my_socket_2 = SmartSocket::new("ThatDamnedSocket");
        /// let my_report = SocketReport::new(&my_socket_1, &my_socket_2);
        /// ```
        pub fn new(
            socket_1: &'sockets_lifetime SmartSocket,
            socket_2: &'sockets_lifetime SmartSocket,
        ) -> Self {
            Self {
                most_wanted_socket_1: socket_1,
                most_wanted_socket_2: socket_2,
            }
        }
    }

    impl SmartDeviceInfoProvider for SocketReport<'_> {
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
            let device_names: [&str; 2] = [
                &self.most_wanted_socket_1.name,
                &self.most_wanted_socket_2.name,
            ];
            let mut result = Vec::with_capacity(device_names.len());

            for name in device_names {
                result.push(name);
            }
            result
        }
    }
}

//TODO:- сделать общий тип для всех отчётов, содержащий в себе Vec<Box<dyn SmartDevice>>
pub mod template_2 {
    //! Шаблон для отчёта о состоянии двух термометров и одной розетки
    //! с заданными именами
    use super::SmartHouse;
    use super::{SmartDeviceInfoProvider, SmartDeviceRegistrationState};
    use crate::devices::socket::SmartSocket;
    use crate::devices::thermometer::SmartThermometer;

    /// Тип представляющий собой форму отчёта о состоянии двух термометров и одной розетки
    ///
    /// Должен быть проинициализирован ссылками на устройства типа SmartThermometer и SmartSocket
    pub struct MixedDevicesReport<'devices_lifetime> {
        thermometer_instance_1: &'devices_lifetime SmartThermometer,
        thermometer_instance_2: &'devices_lifetime SmartThermometer,
        smart_socket_1: &'devices_lifetime SmartSocket,
    }
    impl<'devices_lifetime> MixedDevicesReport<'devices_lifetime> {
        /// Создание пользовательской формы отчёта о состоянии двух термометров и одной розетки
        ///
        /// ## Пример
        /// ```ignore
        /// let my_thermometer_1 = SmartThermometer::new("TheGreatThermometer");
        /// let my_socket_1 = SmartSocket::new("QuitePrettySocket");
        /// let my_thermometer_2 = SmartThermometer::new("UnrealThermometer");
        /// let my_report = MixedDevicesReport::new(&my_thermometer_1, &my_thermometer_2, &my_socket_1);
        /// ```
        ///   
        pub fn new(
            thermo_1: &'devices_lifetime SmartThermometer,
            thermo_2: &'devices_lifetime SmartThermometer,
            socket_1: &'devices_lifetime SmartSocket,
        ) -> Self {
            Self {
                thermometer_instance_1: thermo_1,
                thermometer_instance_2: thermo_2,
                smart_socket_1: socket_1,
            }
        }
    }
    impl SmartDeviceInfoProvider for MixedDevicesReport<'_> {
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
        /// Получение списка имен устройств, для которых необходимо составить отчёт
        fn get_device_names(&self) -> Vec<&str> {
            let device_names: [&str; 3] = [
                &self.thermometer_instance_1.name,
                &self.thermometer_instance_2.name,
                &self.smart_socket_1.name,
            ];
            let mut result = Vec::with_capacity(device_names.len());

            for name in device_names {
                result.push(name);
            }
            result
        }
    }
}
