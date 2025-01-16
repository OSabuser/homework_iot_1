/// Объявление используемых в крейте дочерних модулей
/// Описание работы всей библиотеки
/// Author title, license, etc..
///
///
///
///
mod devices;
mod info_providers;
mod smart_house;

pub use devices::smart_device::{SmartDevice, SmartDevicePowerState, SmartDeviceStatus};
pub use devices::socket::SmartSocket;
pub use devices::thermometer::SmartThermometer;
pub use smart_house::{SmartHouse, SmartHouseManagementStatus, ErrorReason};