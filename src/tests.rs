#[cfg(test)]
mod tests {
    use crate::{SmartSocket, SmartThermometer};

    #[test]
    fn test_smart_thermometer_get_temperature() {
        let thermometer = SmartThermometer::new();
        let value = thermometer.get_temperature();
        assert_eq!(
            1.0 < value && value < 40.0,
            true,
            "Температура датчика вне допустимых пределов[1.0, 40.0],  {}",
            value
        );
    }

    #[test]
    fn test_smart_socket_turn_on_off() {
        let mut socket = SmartSocket::new(false);
        socket.turn_on();
        assert_eq!(socket.get_state(), true, "Розетка должна быть включена");
    }

    #[test]
    fn test_smart_socket_turn_on_on() {
        let mut socket = SmartSocket::new(true);
        socket.turn_on();
        assert_eq!(socket.get_state(), true, "Розетка должна быть включена");
    }

    #[test]
    fn test_smart_socket_turn_off_off() {
        let mut socket = SmartSocket::new(false);
        socket.turn_off();
        assert_eq!(socket.get_state(), false, "Розетка должна быть выключена");
    }

    #[test]
    fn test_smart_socket_turn_off_on() {
        let mut socket = SmartSocket::new(true);
        socket.turn_off();
        assert_eq!(socket.get_state(), false, "Розетка должна быть выключена");
    }

    #[test]
    fn test_smart_socket_get_state_off() {
        let socket = SmartSocket::new(false);
        let value = socket.get_state();
        assert_eq!(value, false, "Розетка должна быть выключена");
    }

    #[test]
    fn test_smart_socket_get_state_on() {
        let socket = SmartSocket::new(true);
        let value = socket.get_state();
        assert_eq!(value, true, "Розетка должна быть включена");
    }

    #[test]
    fn test_smart_socket_get_power_off() {
        let socket = SmartSocket::new(false);
        let value = socket.get_power();
        assert_eq!(
            value, 0.0,
            "Мощность выключенной розетки должна быть 0!, а не {}",
            value
        );
    }

    #[test]
    fn test_smart_socket_get_power_on() {
        let socket = SmartSocket::new(true);
        let value = socket.get_power();
        assert_eq!(
            2.0 <= value && value <= 10.0,
            true,
            "Мощность включенной розетки должна быть в диапазоне [2.0, 10.0], текущее значение {}",
            value
        );
    }
}
