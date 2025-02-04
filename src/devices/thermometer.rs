//! Модуль, содержащий реализацию устройства "Умный термометр"
//!
//! > Умный термометр - это устройство, которое измеряет температуру окружающей среды
//! > и может сообщить о ней пользователю.
//! > В случае, если температура окружающей среды выходит за пределы нормы, умный термометр переходит в состояние ошибки.

use super::smart_device::{
    SmartDevice, SmartDeviceErrorCode, SmartDevicePowerState, SmartDeviceStatus,
};

///
/// Тип описывающий характеристики и поведение девайса "Умный термометр"
///
pub struct SmartThermometer {
    /// Пользовательский псевдоним для термометра
    pub name: String,

    /// Текущая температура окружающей среды(°С)
    temperature: f32,

    // Cтатус работы (ВКЛ,ВЫКЛ/ОШИБКА)
    status: SmartDeviceStatus,
}

impl SmartThermometer {
    /// Создание экземпляра термометра с псевдонимом `name`
    ///
    /// По умолчанию термометр выключен, температура окружающей среды - `0.0 °С`
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

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_power_state(
        &mut self,
        state: SmartDevicePowerState,
    ) -> Result<(), SmartDeviceErrorCode> {
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

    fn get_device_status(&self) -> SmartDeviceStatus {
        self.status.clone()
    }

    fn get_text_report(&self) -> String {
        format!(
            "Current temperature is {}, status: {}\n",
            self.temperature, self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stupid_test() {
        let mut my_thermo_1 = SmartThermometer::new("Thermometer_1");

        let is_device_in_normal_state = my_thermo_1
            .set_power_state(SmartDevicePowerState::Enabled)
            .is_ok();

        assert!(
            is_device_in_normal_state,
            "Device must be in an enabled state!"
        );

        let is_device_enabled = matches!(
            my_thermo_1.get_device_status(),
            SmartDeviceStatus::PowerState(SmartDevicePowerState::Enabled)
        );

        assert!(is_device_enabled, "Device must be in an enabled state!");
    }
}
