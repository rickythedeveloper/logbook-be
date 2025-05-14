use sqlx::types::{BigDecimal, Uuid, chrono::NaiveDateTime};
use sqlx::{Pool, Postgres};
use std::any::Any;

mod pool_utils;
mod tables;

pub trait Database {
    async fn add_logbook_entries(
        &self,
        entries: Vec<tables::logbook_entry::LogbookEntry>,
    ) -> Result<(), ()>;
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {
    async fn add_logbook_entries(
        &self,
        entries: Vec<tables::logbook_entry::LogbookEntry>,
    ) -> Result<(), ()> {
        if entries.is_empty() {
            return Ok(());
        }

        let pool = pool_utils::get_connection_pool().await;

        // Create vectors for each column using getters
        let ids: Vec<Uuid> = entries.iter().map(|e| e.get_id()).collect();
        let aircraft_make_models: Vec<String> = entries
            .iter()
            .map(|e| e.get_aircraft_make_model().to_string())
            .collect();
        let aircraft_registrations: Vec<String> = entries
            .iter()
            .map(|e| e.get_aircraft_registration().to_string())
            .collect();
        let captains: Vec<String> = entries
            .iter()
            .map(|e| e.get_captain().to_string())
            .collect();
        let holder_operating_capacities: Vec<tables::logbook_entry::OperatingCapacity> = entries
            .iter()
            .map(|e| e.get_holder_operating_capacity())
            .collect();
        let departure_locations: Vec<String> = entries
            .iter()
            .map(|e| e.get_departure_location().to_string())
            .collect();
        let arrival_locations: Vec<String> = entries
            .iter()
            .map(|e| e.get_arrival_location().to_string())
            .collect();
        let departure_times: Vec<NaiveDateTime> =
            entries.iter().map(|e| e.get_departure_time()).collect();
        let arrival_times: Vec<NaiveDateTime> =
            entries.iter().map(|e| e.get_arrival_time()).collect();
        let day_single_engine_in_commands: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_day_single_engine_in_command().clone())
            .collect();
        let day_single_engine_duals: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_day_single_engine_dual().clone())
            .collect();
        let night_single_engine_in_commands: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_night_single_engine_in_command().clone())
            .collect();
        let night_single_engine_duals: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_night_single_engine_dual().clone())
            .collect();
        let day_multi_engine_in_commands: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_day_multi_engine_in_command().clone())
            .collect();
        let day_multi_engine_duals: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_day_multi_engine_dual().clone())
            .collect();
        let night_multi_engine_in_commands: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_night_multi_engine_in_command().clone())
            .collect();
        let night_multi_engine_duals: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_night_multi_engine_dual().clone())
            .collect();
        let actual_instruments: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_actual_instrument().clone())
            .collect();
        let simulated_instruments: Vec<BigDecimal> = entries
            .iter()
            .map(|e| e.get_simulated_instrument().clone())
            .collect();
        let day_takeoffs: Vec<i32> = entries.iter().map(|e| e.get_day_takeoffs()).collect();
        let night_takeoffs: Vec<i32> = entries.iter().map(|e| e.get_night_takeoffs()).collect();
        let day_landings: Vec<i32> = entries.iter().map(|e| e.get_day_landings()).collect();
        let night_landings: Vec<i32> = entries.iter().map(|e| e.get_night_landings()).collect();
        let remarks: Vec<String> = entries
            .iter()
            .map(|e| e.get_remarks().to_string())
            .collect();
        let created_ats: Vec<NaiveDateTime> = entries.iter().map(|e| e.get_created_at()).collect();
        let updated_ats: Vec<Option<NaiveDateTime>> =
            entries.iter().map(|e| e.get_updated_at()).collect();

        // Execute the query with UNNEST
        let query_result = sqlx::query(
            "INSERT INTO logbook_entry (
            id, aircraft_make_model, aircraft_registration, captain,
            holder_operating_capacity, departure_location, arrival_location,
            departure_time, arrival_time, day_single_engine_in_command,
            day_single_engine_dual, night_single_engine_in_command,
            night_single_engine_dual, day_multi_engine_in_command,
            day_multi_engine_dual, night_multi_engine_in_command,
            night_multi_engine_dual, actual_instrument, simulated_instrument,
            day_takeoffs, night_takeoffs, day_landings, night_landings,
            remarks, created_at, updated_at
        )
        SELECT * FROM UNNEST(
            $1::uuid[], $2::text[], $3::text[], $4::text[],
            $5::operating_capacity[], $6::text[], $7::text[],
            $8::timestamp[], $9::timestamp[], $10::numeric[],
            $11::numeric[], $12::numeric[], $13::numeric[],
            $14::numeric[], $15::numeric[], $16::numeric[],
            $17::numeric[], $18::numeric[], $19::numeric[],
            $20::int4[], $21::int4[], $22::int4[], $23::int4[],
            $24::text[], $25::timestamp[], $26::timestamp[]
        )",
        )
        .bind(ids)
        .bind(aircraft_make_models)
        .bind(aircraft_registrations)
        .bind(captains)
        .bind(holder_operating_capacities) // Assuming the OperatingCapacity type can be bound directly
        .bind(departure_locations)
        .bind(arrival_locations)
        .bind(departure_times)
        .bind(arrival_times)
        .bind(day_single_engine_in_commands)
        .bind(day_single_engine_duals)
        .bind(night_single_engine_in_commands)
        .bind(night_single_engine_duals)
        .bind(day_multi_engine_in_commands)
        .bind(day_multi_engine_duals)
        .bind(night_multi_engine_in_commands)
        .bind(night_multi_engine_duals)
        .bind(actual_instruments)
        .bind(simulated_instruments)
        .bind(day_takeoffs)
        .bind(night_takeoffs)
        .bind(day_landings)
        .bind(night_landings)
        .bind(remarks)
        .bind(created_ats)
        .bind(updated_ats)
        .execute(&pool)
        .await;

        match query_result {
            Ok(result) => Ok(()),
            Err(err) => Err(()),
        }
    }
}

pub async fn get_one_logbook_entry() -> Result<(), sqlx::Error> {
    let pool = pool_utils::get_connection_pool().await;

    let row =
        sqlx::query_as::<_, tables::logbook_entry::LogbookEntry>("select * from logbook_entry")
            .fetch_one(&pool)
            .await;

    match row {
        Ok(row) => println!("{:?}", row),
        Err(e) => match e {
            sqlx::Error::RowNotFound => println!("No row found"),
            sqlx::Error::Database(db_err) => println!("Database error: {:?}", db_err),
            _ => println!("Error: {:?}", e),
        },
    }

    Ok(())
}
