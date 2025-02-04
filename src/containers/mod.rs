pub mod house;
pub mod room;

/// Перечисление возможных ошибок set/get операций с содержимым умного дома/комнаты
pub enum ErrorReason {
    /// Превышено максимальное количество элементов в контейнере
    ItemLimitExceeded,

    /// В контейнере уже существует элемент с таким именем
    ItemAlreadyPresented,

    /// В контейнере отсутствует элемент с таким именем
    ItemDoesntExist,
}

/// Alias для названия контейнера (умного дома, комнаты)
pub type ContainerName = String;
pub type ContainerIOHistory = String;
