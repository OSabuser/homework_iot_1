//! Модуль, содержащий реализацию устройства "Умная розетка"
//!
//! > Умная розетка - это устройство, которое позволяет управлять подключёнными к ней устройствами
//! > с помощью команды "Включить" и "Выключить". Также умная розетка может сообщать о текущем
//! > потреблении электроэнергии подключёнными устройствами.
//! > В случае возникновения ошибки (перегрузка по току, перегрев и т.д.) умная розетка переходит
//! > в состояние "Ошибка" и перестаёт выполнять команды на включение/выключение.
//!
//!
use super::smart_device::{
    SmartDevice, SmartDeviceErrorCode, SmartDevicePowerState, SmartDeviceStatus,
};

///
/// Тип описывающий характеристики и поведение девайса "Умная розетка"
///
pub struct SmartSocket {
    /// Пользовательский псевдоним для розетки
    pub name: String,

    /// Текущая мощность (Вт), потребляемая подключёнными к розетке устройствами
    power_consumption: f32,

    // Cтатус работы (ВКЛ,ВЫКЛ/ОШИБКА)
    status: SmartDeviceStatus,
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
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            power_consumption: 0.0,
            status: SmartDeviceStatus::PowerState(
                super::smart_device::SmartDevicePowerState::Disabled,
            ),
        }
    }
}

impl SmartDevice for SmartSocket {
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
            "Current power consumption is {}, status: {} \n",
            self.power_consumption, self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stupid_test() {
        let mut my_socket = SmartSocket::new("Socket_1");

        let is_device_in_normal_state = my_socket
            .set_power_state(SmartDevicePowerState::Enabled)
            .is_ok();

        assert!(
            is_device_in_normal_state,
            "Device must be in an enabled state!"
        );

        let is_device_enabled = matches!(
            my_socket.get_device_status(),
            SmartDeviceStatus::PowerState(SmartDevicePowerState::Enabled)
        );

        assert!(is_device_enabled, "Device must be in an enabled state!");
    }
}
