//! Библиотека "Умный дом" OTUS 2024 - [2]
use std::{
    collections::HashMap,
    fmt::{self, format, Display},
};

fn main() {
    let mut my_socket = SmartSocket {
        name: "MySocket_1".to_string(),
        power_consumption: 0.0,
        status: SmartDeviceStatus::PowerState(SmartDevicePowerState::Disabled),
    };

    let mut my_thermometer = SmartThermometer {
        name: "MyThermometer_1".to_string(),
        temperature: 25.5,
        status: SmartDeviceStatus::PowerState(SmartDevicePowerState::Disabled),
    };

    my_socket.set_power_state(SmartDevicePowerState::Enabled);
    my_thermometer.set_power_state(SmartDevicePowerState::Enabled);

    let report_1 = my_socket.get_text_report();
    let report_2 = my_thermometer.get_text_report();

    println!("Report1: {}", report_1);
    println!("Report2: {}", report_2);
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
struct SmartHouse<'a> {
    name: String,
    smart_rooms: HashMap<String, Vec<Box<&'a dyn SmartDevice>>>,
    room_limit: usize,
    device_limit: usize,
}





enum SmartDeviceStatus<T> {
    /// Состояние питания умного устройства
    PowerState(SmartDevicePowerState),
    /// Возможные ошибки в работе умного устройства
    Malfunction(T),
}
/// Перечисление возможных рабочих состояний умного устройства
enum SmartDevicePowerState {
    /// Устройство включено
    Enabled,
    /// Устройство выключено
    Disabled,
}

// Перечисление возможных ошибок в работе умной розетки
enum SmartSocketErrorCode {
    /// Ошибка: перегрузка по току
    Overcurrent,
    /// Ошибка: перегрузка по напряжению
    Overvoltage,
    /// Ошибка: перегрев
    Overheat,
}
trait SmartDevice {
    type ErrorType;
    fn get_device_status(&self) -> &SmartDeviceStatus<Self::ErrorType>;
    fn set_power_state(&mut self, state: SmartDevicePowerState);
    fn get_text_report(&self) -> String;
}

struct SmartSocket {
    name: String,
    power_consumption: f32,
    status: SmartDeviceStatus<<SmartSocket as SmartDevice>::ErrorType>,
}

impl SmartDevice for SmartSocket {
    type ErrorType = SmartSocketErrorCode;

    fn set_power_state(&mut self, state: SmartDevicePowerState) {
        if let SmartDeviceStatus::Malfunction(x) = &self.status {
            println!("Cannot perform the operation due to: {:?}", "error");
        } else {
            self.status = SmartDeviceStatus::PowerState(state);
        }
    }

    fn get_device_status(&self) -> &SmartDeviceStatus<Self::ErrorType> {
        &self.status
    }

    fn get_text_report(&self) -> String {
        format!(
            "Current power consumption of {} is {}, status: {}",
            self.name, self.power_consumption, self.status
        )
    }
}

// Перечисление возможных ошибок в работе умной розетки
enum SmartThermometerЕrrorCode {
    /// Ошибка: слишком низкая температура
    Underheat,
    /// Ошибка: перегрев
    Overheat,
}

struct SmartThermometer {
    name: String,
    temperature: f32,
    status: SmartDeviceStatus<<SmartThermometer as SmartDevice>::ErrorType>,
}

impl SmartDevice for SmartThermometer {
    type ErrorType = SmartThermometerЕrrorCode;

    fn set_power_state(&mut self, state: SmartDevicePowerState) {
        if let SmartDeviceStatus::Malfunction(x) = &self.status {
            println!("Cannot perform the operation due to: {:?}", "error");
        } else {
            self.status = SmartDeviceStatus::PowerState(state);
        }
    }

    fn get_device_status(&self) -> &SmartDeviceStatus<Self::ErrorType> {
        &self.status
    }

    fn get_text_report(&self) -> String {
        format!(
            "{}: current temperature is {}, status: {}",
            self.name, self.temperature, self.status
        )
    }
}

///
/// Реализации трейта Display для SmartDeviceStatus
/// для возможности вывода состояния работы розетки в консоль
///
impl Display for SmartSocketErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Overcurrent => write!(f, "Overcurrent error."),
            Self::Overheat => write!(f, "Overheat error."),
            Self::Overvoltage => write!(f, "Overvoltage error."),
        }
    }
}
impl Display for SmartThermometerЕrrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Underheat => write!(f, "Underheat error."),
            Self::Overheat => write!(f, "Overheat error."),
        }
    }
}
impl Display for SmartDevicePowerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "Disabled."),
            Self::Enabled => write!(f, "Enabled."),
        }
    }
}
impl Display for SmartDeviceStatus<SmartSocketErrorCode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malfunction(x) => write!(f, "{}", x),
            Self::PowerState(y) => write!(f, "{}", y),
        }
    }
}
impl Display for SmartDeviceStatus<SmartThermometerЕrrorCode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malfunction(x) => write!(f, "{}", x),
            Self::PowerState(y) => write!(f, "{}", y),
        }
    }
}
