//! Модуль, содержащий описание
//!
//! Краткое описание сущности реализации
//!

/// Перечисление возможных состояний работы умного устройства
/// Тип T - перечисление ошибок, присущих конкретному устройству
#[derive(Clone)]
pub enum SmartDeviceStatus<T> {
    /// Состояние питания умного устройства
    PowerState(SmartDevicePowerState),
    /// Возможные ошибки в работе умного устройства
    Malfunction(T),
}

/// Перечисление возможных состояний питания умного устройства
#[derive(Clone)]
pub enum SmartDevicePowerState {
    /// Устройство включено
    Enabled,
    /// Устройство выключено
    Disabled,
}

/// SmartDevice trait, определяющий общий функционал для "Умных" устройств
pub trait SmartDevice {
    /// Возможные ошибки в работе устройства
    type ErrorType;

    /// Получение текущего статуса работы устройства
    fn get_device_status(&self) -> &SmartDeviceStatus<Self::ErrorType>;

    /// Попытка включения/выключения устройства  
    ///
    /// В случае, если во время работы устройства возникла ошибка, происходит возврат соответствующего кода 
    fn set_power_state(&mut self, state: SmartDevicePowerState) -> Result<(), Self::ErrorType>;

    /// Получение текстовой информации о состоянии устройства
    fn get_text_report(&self) -> String;
}

use std::fmt::{self, Display};

impl Display for SmartDevicePowerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "Disabled."),
            Self::Enabled => write!(f, "Enabled."),
        }
    }
}
