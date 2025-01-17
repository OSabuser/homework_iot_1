pub mod containers;
/// Модуль, определяющий поведение устройств в системе "Умных дом"
/// Также модуль содержит в себе модули, описывающие конкретные устройства
pub mod devices;

pub use containers::house;
pub use containers::room;
pub use devices::smart_device;
pub use devices::socket;
pub use devices::thermometer;
