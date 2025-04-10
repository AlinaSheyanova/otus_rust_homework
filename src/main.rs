use smart_home::SmartDevice::{Socket, Thermometer};
use smart_home::{Room, SmartHome, SmartSocket, SmartThermometer};

fn main() {
    let mut home: SmartHome = SmartHome::new(vec![
        Room::new(vec![
            Thermometer(SmartThermometer::new()),
            Thermometer(SmartThermometer::new()),
            Socket(SmartSocket::new(false)),
        ]),
        Room::new(vec![
            Thermometer(SmartThermometer::new()),
            Socket(SmartSocket::new(false)),
            Socket(SmartSocket::new(true)),
            Socket(SmartSocket::new(false)),
        ]),
        Room::new(vec![
            Thermometer(SmartThermometer::new()),
            Thermometer(SmartThermometer::new()),
            Socket(SmartSocket::new(false)),
            Socket(SmartSocket::new(true)),
        ]),
    ]);

    home.print_report();
    if let Socket(socket) = &mut home[1][1] {
        socket.turn_on()
    }
    home.print_report();
}
