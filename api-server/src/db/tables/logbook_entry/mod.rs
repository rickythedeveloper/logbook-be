use sqlx::types::{BigDecimal, Uuid, chrono::NaiveDateTime};

#[derive(sqlx::Type, Debug, Copy, Clone)]
#[sqlx(type_name = "operating_capacity_type")]
pub enum OperatingCapacity {
    PIC,
    PUT,
}

#[derive(sqlx::FromRow, Debug)]
pub struct LogbookEntry {
    id: Uuid,
    aircraft_make_model: String,
    aircraft_registration: String,
    captain: String,
    holder_operating_capacity: OperatingCapacity,
    departure_location: String,
    arrival_location: String,
    departure_time: NaiveDateTime,
    arrival_time: NaiveDateTime,
    day_single_engine_in_command: BigDecimal,
    day_single_engine_dual: BigDecimal,
    night_single_engine_in_command: BigDecimal,
    night_single_engine_dual: BigDecimal,
    day_multi_engine_in_command: BigDecimal,
    day_multi_engine_dual: BigDecimal,
    night_multi_engine_in_command: BigDecimal,
    night_multi_engine_dual: BigDecimal,
    actual_instrument: BigDecimal,
    simulated_instrument: BigDecimal,
    day_takeoffs: i32,
    night_takeoffs: i32,
    day_landings: i32,
    night_landings: i32,
    remarks: String,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

impl LogbookEntry {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_aircraft_make_model(&self) -> &str {
        &self.aircraft_make_model
    }

    pub fn get_aircraft_registration(&self) -> &str {
        &self.aircraft_registration
    }

    pub fn get_captain(&self) -> &str {
        &self.captain
    }

    pub fn get_holder_operating_capacity(&self) -> OperatingCapacity {
        self.holder_operating_capacity
    }

    pub fn get_departure_location(&self) -> &str {
        &self.departure_location
    }

    pub fn get_arrival_location(&self) -> &str {
        &self.arrival_location
    }

    pub fn get_departure_time(&self) -> NaiveDateTime {
        self.departure_time
    }

    pub fn get_arrival_time(&self) -> NaiveDateTime {
        self.arrival_time
    }

    pub fn get_day_single_engine_in_command(&self) -> &BigDecimal {
        &self.day_single_engine_in_command
    }

    pub fn get_day_single_engine_dual(&self) -> &BigDecimal {
        &self.day_single_engine_dual
    }

    pub fn get_night_single_engine_in_command(&self) -> &BigDecimal {
        &self.night_single_engine_in_command
    }

    pub fn get_night_single_engine_dual(&self) -> &BigDecimal {
        &self.night_single_engine_dual
    }

    pub fn get_day_multi_engine_in_command(&self) -> &BigDecimal {
        &self.day_multi_engine_in_command
    }

    pub fn get_day_multi_engine_dual(&self) -> &BigDecimal {
        &self.day_multi_engine_dual
    }

    pub fn get_night_multi_engine_in_command(&self) -> &BigDecimal {
        &self.night_multi_engine_in_command
    }

    pub fn get_night_multi_engine_dual(&self) -> &BigDecimal {
        &self.night_multi_engine_dual
    }

    pub fn get_actual_instrument(&self) -> &BigDecimal {
        &self.actual_instrument
    }

    pub fn get_simulated_instrument(&self) -> &BigDecimal {
        &self.simulated_instrument
    }

    pub fn get_day_takeoffs(&self) -> i32 {
        self.day_takeoffs
    }

    pub fn get_night_takeoffs(&self) -> i32 {
        self.night_takeoffs
    }

    pub fn get_day_landings(&self) -> i32 {
        self.day_landings
    }

    pub fn get_night_landings(&self) -> i32 {
        self.night_landings
    }

    pub fn get_remarks(&self) -> &str {
        &self.remarks
    }

    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn get_updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
