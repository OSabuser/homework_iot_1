/// Модуль, определяющий поведение устройств в системе "Умных дом"
/// Также модуль содержит в себе модули, описывающие конкретные устройства
pub mod devices;
pub mod info_providers;
pub mod smart_house;

pub use devices::smart_device;
pub use devices::socket;
pub use devices::thermometer;
