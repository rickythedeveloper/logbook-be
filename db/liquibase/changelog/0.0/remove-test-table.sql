--liquibase formatted sql

--changeset ricky:remove-test-table
DROP TABLE IF EXISTS test_table ;