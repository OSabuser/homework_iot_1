//! Библиотека "Умный дом" OTUS 2024 - [2]
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;


fn main() {
    // Создание нового экземпляра дома
    let mut smart_house = SmartHouse::new("MyLoungeHome");

    smart_house.get_rooms();

    // Создание нескольких помещений с уникальными именами
    let mut my_room1 = SmartRoom::new("MainRoom1");
    let mut my_room2 = SmartRoom::new("MainRoom2");
    let mut my_room3 = SmartRoom::new("MainRoom3");

    // Создание нескольких умных устройств с уникальными именами
    let my_thermometer1: SmartThermometer = SmartThermometer::new("MyThermo1");
    let my_thermometer2: SmartThermometer = SmartThermometer::new("MyThermo2");
    let my_thermometer3: SmartThermometer = SmartThermometer::new("MyThermo3");
    let my_smart_plug_1: SmartPlug = SmartPlug::new("MyPrettySocket#1");
    let my_smart_plug_2: SmartPlug = SmartPlug::new("MyPrettySocket#2");


    // Регистрация умных устройств в соответствующих помещениях
    my_room1.add_device(&my_thermometer1.name.clone(), Box::new(my_thermometer1));
    my_room2.add_device(&my_thermometer2.name.clone(), Box::new(my_thermometer2));
    my_room2.add_device(&my_smart_plug_1.name.clone(), Box::new(my_smart_plug_1));
    my_room3.add_device(&my_smart_plug_2.name.clone(), Box::new(my_smart_plug_2));
    my_room3.add_device(&my_thermometer3.name.clone(), Box::new(my_thermometer3));


    // Добавление помещений в дом smart_house с передачей владения
    smart_house.add_room(&my_room1.name.clone(), my_room1);
    smart_house.add_room(&my_room2.name.clone(), my_room2);
    smart_house.add_room(&my_room3.name.clone(), my_room3);

    smart_house.get_rooms();


    // Удаление помещения с именем MainRoom1 из дома smart_house
    smart_house.delete_room("MainRoom1");

    // Получение ссылки на экземпляр комнаты с определённым именем
    if let Some(x) = smart_house.get_room("MainRoom2") {
        println!("Получен доступ к помещению {}", x.name);
        x.get_devices();
    } else {
        println!("В доме {} нет помещения с именем {}!", smart_house.name, "MainRoom2");
    }


}



trait SmartDevice {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
}
enum NamedItemOperationStatus {
    OperationSucceded,
    OperationFailed(ErrorReason),
}
enum ErrorReason {
    ItemLimitExceeded,
    ItemAlreadyPresented,
    ItemDoesntExist,
}

///
/// Низкоуровневый интерфейс для работы с контейнерами, содержащими именнованные элементы
/// Максимальное количество элементов в контейнере определяется реализацией трейта HasLimitedCapacity
/// В качестве контейнера можно использовать, например, Vec, HashMap...
///
trait NamedItemContainer {
    /// Тип элемента, который будет содержаться в контейнере
    type ItemType;

    const MAX_SIZE: usize;

    /// Добавление элемента с уникальным именем name в контейнер
    fn add_element<ItemType>(
        &mut self,
        name: &str,
        element: Self::ItemType,
    ) -> NamedItemOperationStatus;

    /// Удаление элемента, имеющего уникальное имя name из контейнера
    fn remove_element(&mut self, name: &str) -> NamedItemOperationStatus;

    /// Получение списка уникальных имён элементов, содержащихся в контейнере
    fn get_element_names(&self) -> Option<Vec<String>>;

    /// Получение ссылки на экземпляр элемента, содержащегося в контейнере
    fn get_element_by_name(&self, name: &str) -> Option<&Self::ItemType>;

    /// Проверка: содержится ли элемент с именем name в контейнере
    fn is_element_exist(&self, name: &str) -> bool;
}

///
/// 1. Умный дом
/// В текущей версии может содержать до 5 комнат с различными умными устройствами
///
/// ## Параметры
///
/// * `name` - пользовательский псевдоним для дома
/// * `rooms` - список комнат (хэш таблица, где key - уникальное имя комнаты, value - конкретный экземпляр комнаты с именем key)
///
struct SmartHouse {
    name: String,
    rooms: HashMap<String, <SmartHouse as NamedItemContainer>::ItemType>,
}

impl NamedItemContainer for SmartHouse {
    type ItemType = SmartRoom;

    const MAX_SIZE: usize = 5;

    fn get_element_by_name(&self, name: &str) -> Option<&Self::ItemType> {
        self.rooms.get(name)
    }

    fn add_element<ItemType>(
        &mut self,
        name: &str,
        element: Self::ItemType,
    ) -> NamedItemOperationStatus {
        if self.rooms.len() == Self::MAX_SIZE {
            return NamedItemOperationStatus::OperationFailed(ErrorReason::ItemLimitExceeded);
        }

        if self.is_element_exist(name) == true {
            return NamedItemOperationStatus::OperationFailed(ErrorReason::ItemAlreadyPresented);
        } else {
            self.rooms.insert(name.to_string(), element);
            return NamedItemOperationStatus::OperationSucceded;
        }
    }

    fn remove_element(&mut self, name: &str) -> NamedItemOperationStatus {
        match self.rooms.remove_entry(name) {
            Some(_) => NamedItemOperationStatus::OperationSucceded,
            None => NamedItemOperationStatus::OperationFailed(ErrorReason::ItemDoesntExist),
        }
    }

    fn get_element_names(&self) -> Option<Vec<String>> {
        if self.rooms.is_empty() == true {
            None
        } else {
            let mut room_names = Vec::with_capacity(self.rooms.len());

            for room in self.rooms.keys() {
                room_names.push(room.clone());
            }

            Some(room_names)
        }
    }

    fn is_element_exist(&self, name: &str) -> bool {
        self.rooms.contains_key(name)
    }
}

impl SmartHouse {
    fn new(smart_house_name: &str) -> Self {
        Self {
            name: String::from(smart_house_name),
            rooms: HashMap::with_capacity(Self::MAX_SIZE),
        }
    }

    /// Регистрация нового помещения в доме (с последующей передачей владения)
    fn add_room(
        &mut self,
        name: &str,
        element: <SmartHouse as NamedItemContainer>::ItemType,
    ) -> () {
        match self.add_element::<<SmartHouse as NamedItemContainer>::ItemType>(name, element) {
            NamedItemOperationStatus::OperationSucceded => {
                println!("В дом {} было добавлено помещение: {:?}", self.name, name)
            }
            NamedItemOperationStatus::OperationFailed(_) => {
                println!("Ошибка регистрации нового помещения!")
            }
        }
    }

    /// Получение ссылки на объект комнаты с именем name
    fn get_room(&self, name: &str) -> Option<&<SmartHouse as NamedItemContainer>::ItemType> {
        self.get_element_by_name(name)
    }

    /// Удаление помещения из дома
    fn delete_room(&mut self, name: &str) -> () {
        match self.remove_element(name) {
            NamedItemOperationStatus::OperationSucceded => {
                println!("Из дома {} было удалено помещение: {:?}", self.name, name)
            }
            NamedItemOperationStatus::OperationFailed(_) => println!("Ошибка удаления помещения!"),
        }
    }

    /// Получение списка зарегистрированных в доме помещений
    fn get_rooms(&self) -> Option<Vec<String>> {
        if let Some(x) = self.get_element_names() {
            println!("В доме {} зарегистрированы помещения: {:?}", self.name, x);
            Some(x)
        } else {
            println!("В доме {} нет зарегистрированных помещений", self.name);
            None
        }
    }

    /// Получение списка зарегистрированных в помещении умных устройств
    fn get_devices(&self, room_name: &str) -> Option<Vec<String>> {
        // Размер возвращаемого массива можно выбрать самостоятельно
        todo!("список устройств в комнате `room`")
    }

    fn create_report(
        &self,
        /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    ) -> String {
        todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

///
/// 2. Умная комната
/// В текущей версии может содержать до 10 уникальных умных устройств
///
/// ## Параметры
///
/// * `name` - пользовательский псевдоним для дома
/// * `rooms` - список комнат
///
struct SmartRoom {
    name: String,
    devices: HashMap<String, <SmartRoom as NamedItemContainer>::ItemType>,
}

impl SmartRoom {
    fn new(smart_room_name: &str) -> Self {
        Self {
            name: String::from(smart_room_name),
            devices: HashMap::with_capacity(Self::MAX_SIZE),
        }
    }

    /// Регистрация нового умного устройства в помещении (с последующей передачей владения)
    fn add_device(
        &mut self,
        name: &str,
        element: <SmartRoom as NamedItemContainer>::ItemType,
    ) -> () {
        match self.add_element::<<SmartRoom as NamedItemContainer>::ItemType>(name, element) {
            NamedItemOperationStatus::OperationSucceded => {
                println!(
                    "В помещение {} было добавлено умное устройство: {:?}",
                    self.name, name
                )
            }
            NamedItemOperationStatus::OperationFailed(_) => {
                println!("Ошибка регистрации нового умного устройства!")
            }
        }
    }

    /// Получение ссылки на объект умного устройства с именем name
    fn get_device(&self, name: &str) -> Option<&<SmartRoom as NamedItemContainer>::ItemType> {
        self.get_element_by_name(name)
    }


    /// Удаление умного устройства с именем name из помещения
    fn delete_device(&mut self, name: &str) -> () {
        match self.remove_element(name) {
            NamedItemOperationStatus::OperationSucceded => {
                println!(
                    "Из помещения {} было удалено умное устройство: {:?}",
                    self.name, name
                )
            }
            NamedItemOperationStatus::OperationFailed(_) => {
                println!("Ошибка удаления умного устройства!")
            }
        }
    }

    /// Получение списка зарегистрированных в помещении умных устройств
    fn get_devices(&self) -> Option<Vec<String>> {
        if let Some(x) = self.get_element_names() {
            println!(
                "В помещении {} зарегистрированы умные устройства: {:?}",
                self.name, x
            );
            Some(x)
        } else {
            println!(
                "В помещении {} нет зарегистрированных умных устройств",
                self.name
            );
            None
        }
    }
}

impl NamedItemContainer for SmartRoom {
    type ItemType = Box<dyn SmartDevice>;

    const MAX_SIZE: usize = 10;

    fn add_element<ItemType>(
        &mut self,
        name: &str,
        element: Self::ItemType,
    ) -> NamedItemOperationStatus {
        if self.devices.len() == Self::MAX_SIZE {
            return NamedItemOperationStatus::OperationFailed(ErrorReason::ItemLimitExceeded);
        }

        if self.is_element_exist(name) == true {
            return NamedItemOperationStatus::OperationFailed(ErrorReason::ItemAlreadyPresented);
        } else {
            self.devices.insert(name.to_string(), element);
            return NamedItemOperationStatus::OperationSucceded;
        }
    }

    fn get_element_by_name(&self, name: &str) -> Option<&Self::ItemType> {
        self.devices.get(name)
    }

    fn remove_element(&mut self, name: &str) -> NamedItemOperationStatus {
        match self.devices.remove_entry(name) {
            Some(_) => NamedItemOperationStatus::OperationSucceded,
            None => NamedItemOperationStatus::OperationFailed(ErrorReason::ItemDoesntExist),
        }
    }

    fn get_element_names(&self) -> Option<Vec<String>> {
        if self.devices.is_empty() == true {
            None
        } else {
            let mut device_names = Vec::with_capacity(self.devices.len());

            for device in self.devices.keys() {
                device_names.push(device.clone());
            }

            Some(device_names)
        }
    }

    fn is_element_exist(&self, name: &str) -> bool {
        self.devices.contains_key(name)
    }
}


///
/// 3. Умный термометр
///
/// ## Параметры
///
/// * `name` - пользовательский псевдоним для термометра 
/// * `current_temperature` - текущая температура в градусах Цельсия
///
pub struct SmartThermometer {
    name: String,
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
    pub fn new(device_name: &str) -> Self {
        Self {
            name: String::from(device_name),
            current_temperature: 0.0,
        }
    }
    /// Получение значения текущей температуры конкретного экземпляра термометра
    pub fn _get_current_temperature(&self) -> f32 {
        self.current_temperature
    }
}

impl SmartDevice for SmartThermometer {}

///
/// 4. Умная розетка
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

impl SmartDevice for SmartPlug {}


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






