//! Модуль, содержащий реализацию устройства "Умный термометр"
//!
//! Краткое описание сущности реализации
//!
use super::smart_device::{SmartDevice, SmartDevicePowerState, SmartDeviceStatus};

///
/// Тип описывающий характеристики и поведение девайса "Умный термометр"
///
pub struct SmartThermometer {
    /// Пользовательский псевдоним для термометра
    pub name: String,

    /// Текущая температура окружающей среды(°С)
    temperature: f32,

    // Cтатус работы (ВКЛ,ВЫКЛ/ОШИБКА)
    status: SmartDeviceStatus<<SmartThermometer as SmartDevice>::ErrorType>,
}

impl SmartThermometer {
    /// Создание экземпляра термометра с псевдонимом `name`
    ///
    /// По умолчанию термометр выключен, температура окружающей среды - 0.0 °С
    ///
    /// ## Пример
    /// ```ignore
    /// let my_plug = SmartThermometer::new("ThatFamousThing_0");
    /// ```
    ///
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            temperature: 0.0,
            status: SmartDeviceStatus::PowerState(
                super::smart_device::SmartDevicePowerState::Disabled,
            ),
        }
    }
}

// Перечисление возможных ошибок в работе термометра
#[derive(Clone)]
pub enum SmartThermometerЕrrorCode {
    /// Ошибка: слишком низкая температура
    Underheat,

    /// Ошибка: перегрев
    Overheat,
}

impl SmartDevice for SmartThermometer {
    type ErrorType = SmartThermometerЕrrorCode;

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
            "{}: current temperature is {}, status: {}",
            self.name, self.temperature, self.status
        )
    }
}

use std::fmt::{self, Display};
impl Display for SmartThermometerЕrrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Underheat => write!(f, "Underheat error."),
            Self::Overheat => write!(f, "Overheat error."),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stupid_test() -> () {
        let mut my_socket = SmartThermometer::new("Thermometer_1");

        let is_device_in_normal_state =
            match my_socket.set_power_state(SmartDevicePowerState::Enabled) {
                Ok(_) => true,
                _ => false,
            };

        assert_eq!(
            is_device_in_normal_state, true,
            "Device must be in an enabled state!"
        );

        let is_device_enabled = match my_socket.get_device_status() {
            SmartDeviceStatus::PowerState(SmartDevicePowerState::Enabled) => true,
            _ => false,
        };

        assert_eq!(
            is_device_enabled, true,
            "Device must be in an enabled state!"
        );
    }
}
