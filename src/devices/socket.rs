use super::{SmartDevice, SmartDeviceStatus, SmartDevicePowerState};

///
/// Тип описывающий характеристики и поведение девайса "Умная розетка"
///
struct SmartSocket {
    /// Пользовательский псевдоним для розетки
    name: String,

    /// Текущая мощность (Вт), потребляемая подключёнными к розетке устройствами
    power_consumption: f32,

    // Cтатус работы (ВКЛ,ВЫКЛ/ОШИБКА)
    status: SmartDeviceStatus<<SmartSocket as SmartDevice>::ErrorType>,
}


impl SmartSocket {
    /// Создание экземпляра умной розетки с псевдонимом `name`
    /// 
    /// По умолчанию розетка выключена, потребление - `0.0 Вт`
    ///
    /// ## Пример
    /// ```ignore
    /// let my_plug = SmartSocket::new("MyPreciousPlug_1");
    /// ```
    /// 
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            power_consumption: 0.0,
            status: SmartDeviceStatus::PowerState(super::SmartDevicePowerState::Disabled),
        }
    }
}


// Перечисление возможных ошибок в работе умной розетки
#[derive(Clone)]
enum SmartSocketErrorCode {
    /// Ошибка: перегрузка по току
    Overcurrent,

    /// Ошибка: перегрузка по напряжению
    Overvoltage,

    /// Ошибка: перегрев
    Overheat,
}


impl SmartDevice for SmartSocket {
    type ErrorType = SmartSocketErrorCode;

    fn set_power_state(&mut self, state: SmartDevicePowerState) -> Result<(), Self::ErrorType> {
        match &self.status {
            SmartDeviceStatus::PowerState(_) => {
                self.status = SmartDeviceStatus::PowerState(state);
                Ok(())
            }
            SmartDeviceStatus::Malfunction(y) => {
                println!("Cannot perform the operation due to: {}", y);
                Err((*y).clone())
            }
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


use std::fmt::{self, Display};

impl Display for SmartSocketErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Overcurrent => write!(f, "Overcurrent error."),
            Self::Overheat => write!(f, "Overheat error."),
            Self::Overvoltage => write!(f, "Overvoltage error."),
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