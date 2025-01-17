pub mod house;
pub mod room;

/// Результат set/get операций с содержимым умного дома/комнаты
pub enum SmartContainerManagementStatus {
    /// Успех
    OperationSucceded,

    /// Ошибки set/get
    OperationFailed(ErrorReason),
}

/// Перечисление возможных ошибок set/get операций с содержимым умного дома/комнаты
pub enum ErrorReason {
    /// Превышено максимальное количество элементов в контейнере
    ItemLimitExceeded,

    /// В контейнере уже существует элемент с таким именем
    ItemAlreadyPresented,

    /// В контейнере отсутствует элемент с таким именем
    ItemDoesntExist,
}
