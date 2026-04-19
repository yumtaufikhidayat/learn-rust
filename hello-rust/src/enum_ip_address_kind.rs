pub enum IpAddressKind {
    V4, V6
}

pub struct IpAddress {
    pub kind: IpAddressKind,
    pub address: String,
}

pub enum IpAddresses {
    V4(String),
    V6(String),
}

pub enum IpAddressCustom {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}