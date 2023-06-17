enum _SmartSocketStatus {
    _ON,
    _OFF,
}
struct _SmartHouse {
    _socket: _SmartSocket,
    _thermometer: _SmartThermometer,
}

struct _SmartSocket {
    _description: String,
    _status: _SmartSocketStatus,
}

struct _SmartThermometer {
    _temperature: f32,
}

impl _SmartHouse {
    fn _new(_socket: _SmartSocket, _thermometer: _SmartThermometer) -> Self {
        todo!()
    }
}

impl _SmartSocket {
    fn _new() -> Self {
        todo!()
    }

    fn _get_description(&self) -> String {
        todo!()
    }

    fn _power_on_of(&self) {
        todo!()
    }
}

impl _SmartThermometer {
    fn _new() -> Self {
        todo!()
    }

    fn _get_current_temperature(&self) -> f32 {
        todo!()
    }
}

fn main() {
    todo!()
}
