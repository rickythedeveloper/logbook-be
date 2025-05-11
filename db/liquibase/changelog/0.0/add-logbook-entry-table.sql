--liquibase formatted sql

--changeset ricky:add-logbook-entry-table
CREATE TYPE operating_capacity_type AS ENUM (
    'Captain',
    'First Officer',
    'Relief Pilot',
    'Flight Instructor',
    'Check Airman',
    'Student Pilot',
    'Observer'
);

CREATE TABLE logbook_entry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aircraft_make_model TEXT NOT NULL,
    aircraft_registration TEXT NOT NULL,
    captain TEXT NOT NULL,
    holder_operating_capacity operating_capacity_type NOT NULL,
    departure_location TEXT NOT NULL,
    arrival_location TEXT NOT NULL,
    departure_time TIMESTAMP NOT NULL,
    arrival_time TIMESTAMP NOT NULL,

    -- Single Engine Time
    day_single_engine_in_command NUMERIC(5,1) NOT NULL,
    day_single_engine_dual NUMERIC(5,1) NOT NULL,
    night_single_engine_in_command NUMERIC(5,1) NOT NULL,
    night_single_engine_dual NUMERIC(5,1) NOT NULL,

    -- Multi Engine Time
    day_multi_engine_in_command NUMERIC(5,1) NOT NULL,
    day_multi_engine_dual NUMERIC(5,1) NOT NULL,
    night_multi_engine_in_command NUMERIC(5,1) NOT NULL,
    night_multi_engine_dual NUMERIC(5,1) NOT NULL,

    -- instrument
    actual_instrument NUMERIC(5,1) NOT NULL,
    simulated_instrument NUMERIC(5,1) NOT NULL,

    -- takeoff / landing counts
    day_takeoffs INT NOT NULL,
    night_takeoffs INT NOT NULL,
    day_landings INT NOT NULL,
    night_landings INT NOT NULL,

    remarks TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
RETURN NEW;
END
$$ language 'plpgsql';

CREATE TRIGGER update_pilot_logbook_timestamp
BEFORE UPDATE ON logbook_entry
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

--rollback DROP TRIGGER update_pilot_logbook_timestamp ON logbook_entry;
--rollback DROP FUNCTION update_updated_at_column;
--rollback DROP TABLE logbook_entry;
--rollback DROP TYPE operating_capacity_type;