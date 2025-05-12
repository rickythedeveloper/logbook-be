use sqlx::types::{chrono::NaiveDateTime, BigDecimal, Uuid};

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "operating_capacity_type")]
enum OperatingCapacity {
    PIC,
    PUT
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