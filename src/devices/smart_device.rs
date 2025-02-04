//! Модуль содержит описание общего функционала для "Умных" устройств
//! и перечисления возможных состояний работы умного устройства
//!

/// Перечисление возможных состояний работы умного устройства
/// Тип T - перечисление ошибок, присущих конкретному устройству

//TODO: SmartDeviceStatus from PowerState & Malfunction

#[derive(Clone)]
pub enum SmartDeviceStatus {
    /// Состояние питания умного устройства
    PowerState(SmartDevicePowerState),
    /// Возможные ошибки в работе умного устройства
    Malfunction(SmartDeviceErrorCode),
}
#[derive(Clone)]
pub enum SmartDeviceErrorCode {
    /// Ошибка: перегрузка по току
    Overcurrent,

    /// Ошибка: перегрузка по напряжению
    Overvoltage,

    /// Ошибка: перегрев
    Overheat,

    /// Ошибка: слишком низкая температура
    Underheat,
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
    /// Получение текущего статуса работы устройства
    fn get_device_status(&self) -> SmartDeviceStatus;

    /// Попытка включения/выключения устройства  
    ///
    /// В случае, если во время работы устройства возникла ошибка, происходит возврат соответствующего кода
    fn set_power_state(&mut self, state: SmartDevicePowerState)
        -> Result<(), SmartDeviceErrorCode>;

    /// Получение текстовой информации о состоянии устройства
    fn get_text_report(&self) -> String;

    /// Получение имени устройства
    fn get_name(&self) -> &str;
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
impl Display for SmartDeviceErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Underheat => write!(f, "Underheat error."),
            Self::Overcurrent => write!(f, "Overcurrent error."),
            Self::Overheat => write!(f, "Overheat error."),
            Self::Overvoltage => write!(f, "Overvoltage error."),
        }
    }
}
impl Display for SmartDeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malfunction(x) => write!(f, "{}", x),
            Self::PowerState(y) => write!(f, "{}", y),
        }
    }
}
