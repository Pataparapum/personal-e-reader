/// Estado del sistema que muestra la barra superior.
/// Hoy son datos de prueba; después los alimentará un módulo `system/`,
#[derive(Debug, Clone)]
pub struct Status {
    pub battery: Battery,
    pub time: String,
    pub wifi: Wifi,
    pub usb: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Battery {
    pub percent: u8,
    pub charging: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Wifi {
    Off,
    /// Intensidad de 1 a 3,
    On(u8)
}

impl Status {
    pub fn mock() -> Self {
        Self {
            battery: Battery { percent: 82, charging: false },
            time: "09:41".to_string(),
            wifi: Wifi::On(3),
            usb:true
        }
    }
}