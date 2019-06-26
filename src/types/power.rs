#[derive(Debug)]
pub enum Type {
    AC,
    UPS,
    BATTERY,
}

#[derive(Debug)]
pub struct PowerSources {
    pub sources: Option<Vec<Battery>>,
    pub power_type: Type,
    pub adapter: Option<Adapter>,
}

#[derive(Debug)]
pub struct Battery {
    pub present: bool,
    pub charged: bool,
    pub state: String,
    pub charging: bool,
    pub current: i64,
    pub finishing_charge: bool,
    pub max_capacity: i64,
    pub design_cycle_count: i64,
    pub provides_time_remaining: bool,
    pub time_remaining: Option<i64>,
    pub capacity: i64,
    pub id: i64,
    pub time_to_charge: i64,
    pub name: String,
    pub serial_number: String,
    pub transport_type: String,
    pub power_type: String,
    pub health: String,
}

#[derive(Debug)]
pub struct Adapter {
    pub id: i64,
    pub serial_number: i64,
    pub source: i64,
    pub family: i64,
    pub watts: i64,
    pub current: i64,
    pub voltage: i64,
}
