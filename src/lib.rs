//! Библиотека "Умный дом" OTUS 2024
use std::fmt;
use std::fmt::Display;

///
/// 1. Умный термометр
///
/// ## Параметры
///
/// * `current_temperature` - текущая температура в градусах Цельсия
///
pub struct SmartThermometer {
    current_temperature: f32,
}

impl SmartThermometer {
    /// Создание экземпляра умного термометра
    /// *По умолчанию текущая температура равна 0.0*
    ///
    /// ## Пример
    /// ```
    /// let my_thermometer = _SmartThermometer::new();
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            current_temperature: 0.0,
        }
    }
    /// Получение значения текущей температуры конкретного экземпляра термометра
    pub fn _get_current_temperature(&self) -> f32 {
        self.current_temperature
    }
}

///
/// 2. Умная розетка
///
/// ## Параметры
///
/// * `name` - пользовательский псевдоним для розетки
/// * `status` - статус работы (ВКЛ/ВЫКЛ/ОШИБКА)
/// * `power_consumption` - текущая мощность [Вт], потребляемая подключёнными к розетке устройствами
///
pub struct SmartPlug {
    name: String,
    status: SmartPlugStatus,
    power_consumption: f32,
}
impl SmartPlug {
    /// Создание экземпляра умной розетки с псевдонимом [name]
    /// *По умолчанию розетка выключена, потребление - 0.0 W*
    ///
    /// ## Пример
    /// ```
    /// let my_plug = _SmartPlug::new("MyPreciousPlug_1");
    /// ```
    ///
    pub fn new(device_name: &str) -> Self {
        Self {
            name: String::from(device_name),
            status: SmartPlugStatus::PowerState(SmartPlugPowerState::Disabled),
            power_consumption: 0.0,
        }
    }
    /// Получение значения текущей потребляемой мощности для конкретного экземпляра розетки
    pub fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }
    /// Получение текстового описания конкретного экземпляра розетки (псевдоним, потребляемая мощность, статус работы)
    pub fn get_status(&self) {
        println!("#1 Device name: {}", self.name);
        println!(
            "#2 Current power consumption is: {} W",
            self.get_power_consumption()
        );
        println!("#3 Device status: {}", self.status);
    }
    /// Включение/выключение конкретного экземпляра розетки
    /// *Если розетка работает некорректно, в консоль будет выведено сообщение с соответствующей ошибкой*
    ///
    /// ## Пример
    /// ```
    /// //Создание нового экземпляра розетки с его последующим включением
    /// let my_plug = _SmartPlug::new("MyPreciousPlug_1");
    /// my_plug._set_power_state(_SmartPlugPowerState::Enabled);
    /// ```
    ///
    pub fn set_power_state(&mut self, power_state: SmartPlugPowerState) {
        if let SmartPlugStatus::Malfunction(error_code) = &self.status {
            println!("Cannot perform the operation due to: {}", error_code);
        } else {
            self.status = SmartPlugStatus::PowerState(power_state);
        }
    }
}

/// Перечисление возможных рабочих состояний умной розетки
enum SmartPlugStatus {
    /// Состояние питания розетки
    PowerState(SmartPlugPowerState),
    /// Возможные ошибки в работе розетки
    Malfunction(SmartPlugErrorCode),
}
/// Перечисление возможных рабочих состояний умной розетки
pub enum SmartPlugPowerState {
    /// Розетка включена
    Enabled,
    /// Розетка выключена
    Disabled,
}
/// Перечисление возможных рабочих состояний умной розетки
enum SmartPlugErrorCode {
    /// Ошибка: перегрузка по току
    Overcurrent,
    /// Ошибка: перегрузка по напряжению
    Overvoltage,
    /// Ошибка: перегрев
    Overheat,
}

///
/// Реализации трейта Display для SmartPlugPowerState, SmartPlugErrorCode, SmartPlugStatus
/// для возможности вывода состояния работы розетки в консоль
///
impl Display for SmartPlugStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malfunction(x) => write!(f, "{}", x),
            Self::PowerState(y) => write!(f, "{}", y),
        }
    }
}
impl Display for SmartPlugPowerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Disabled => write!(f, "Disabled."),
            Self::Enabled => write!(f, "Enabled."),
        }
    }
}
impl Display for SmartPlugErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Overcurrent => write!(f, "Overcurrent error!"),
            Self::Overheat => write!(f, "Overheat error!"),
            Self::Overvoltage => write!(f, "Overvoltage error!"),
        }
    }
}
