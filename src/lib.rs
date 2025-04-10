mod tests;

use rand::prelude::*;
use std::ops::{Index, IndexMut};

/// Тип описывающий устройство "Умный термометр"
#[derive(Debug)]
pub struct SmartThermometer {}

impl SmartThermometer {
    pub fn new() -> Self {
        Self {}
    }

    /// Получение значения температуры с датчика
    ///
    /// допустимый диапазон значения от 1.0 до 40.0
    ///
    /// # Пример использования
    ///
    /// ```
    /// use smart_home::SmartThermometer;
    ///
    /// let thermometer = SmartThermometer::new();
    ///
    /// let value = thermometer.get_temperature();
    ///
    /// assert_eq!(1.0 < value && value < 40.0, true,
    /// "Температура датчика вне допустимых пределов[1.0, 40.0],  {}", value);
    ///
    /// ```
    pub fn get_temperature(&self) -> f32 {
        rand::rng().random_range(1.0..40.0)
    }
}

impl Default for SmartThermometer {
    fn default() -> Self {
        Self::new()
    }
}

/// Тип описывающий устройство "Умная розетка"
#[derive(Debug)]
pub struct SmartSocket {
    state: bool,
}

impl SmartSocket {
    pub fn new(state: bool) -> Self {
        Self { state }
    }

    /// Включение розетки
    ///
    /// # Пример использования
    ///
    /// ```
    ///  use smart_home::SmartSocket;
    ///
    ///  let mut socket = SmartSocket::new(false);
    ///  socket.turn_on();
    ///  assert_eq!(socket.get_state(), true, "Розетка должна быть включена");
    /// ```
    pub fn turn_on(&mut self) {
        self.state = true
    }

    /// Выключение розетки
    ///
    /// # Пример использования
    /// ```
    ///  use smart_home::SmartSocket;
    ///
    ///  let mut socket = SmartSocket::new(true);
    ///  socket.turn_off();
    ///  assert_eq!(socket.get_state(), false, "Розетка должна быть выключена");
    /// ```
    pub fn turn_off(&mut self) {
        self.state = false
    }

    /// Получение статуса розетки
    ///
    /// допустимые значения true/false
    ///
    /// # Пример использования
    ///
    /// ```
    ///  use smart_home::SmartSocket;
    ///
    ///  let socket = SmartSocket::new(true);
    ///  let value = socket.get_state();
    ///  assert_eq!(value, true, "Розетка должна быть включена");
    /// ```
    pub fn get_state(&self) -> bool {
        self.state
    }

    /// Получение значения температуры с датчика
    ///
    /// допустимый диапазон значения от 1.0 до 40.0
    ///
    /// # Пример использования
    ///
    /// ```
    ///  use smart_home::SmartSocket;
    ///
    ///  let socket = SmartSocket::new(true);
    ///  let value = socket.get_power();
    ///  assert_eq!(2.0 <= value && value <= 10.0, true,
    ///  "Мощность включенной розетки должна быть в диапазоне [2.0, 10.0], текущее значение {}", value);
    ///
    /// ```
    pub fn get_power(&self) -> f32 {
        match self.state {
            true => rand::rng().random_range(2.0..10.0),
            false => 0.0,
        }
    }
}

pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}

impl SmartDevice {
    /// Печатает текущий статус устройства.
    pub fn print_status(&self) {
        match self {
            SmartDevice::Thermometer(thermometer) => {
                println!(
                    "Thermometer:\tCurrent temperature: {:.2}°C",
                    thermometer.get_temperature()
                );
            }
            SmartDevice::Socket(socket) => match socket.get_state() {
                true => {
                    println!(
                        "Socket:\tCurrent sate: ON; Current power: {:.2}W",
                        socket.get_power()
                    )
                }
                false => {
                    println!("Socket:\tCurrent sate: OFF")
                }
            },
        }
    }
}

pub struct Room {
    devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        Self { devices }
    }

    pub fn print_report(&self) {
        println!("Devices:");
        for (index, device) in self.devices.iter().enumerate() {
            print!("\t{}: \t", index + 1);
            device.print_status()
        }
    }
}

impl Index<usize> for Room {
    type Output = SmartDevice;

    fn index(&self, index: usize) -> &Self::Output {
        &self.devices[index]
    }
}

impl IndexMut<usize> for Room {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.devices[index]
    }
}

pub struct SmartHome {
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    pub fn print_report(&self) {
        println!("Smart home report:");
        for (index, room) in self.rooms.iter().enumerate() {
            println!("Room {}:", index + 1,);
            room.print_report()
        }
    }
}

impl Index<usize> for SmartHome {
    type Output = Room;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rooms[index]
    }
}

impl IndexMut<usize> for SmartHome {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rooms[index]
    }
}
