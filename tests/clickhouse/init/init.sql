-- create database for store obf metrics
CREATE DATABASE IF NOT EXISTS sleipnir;

-- table for store metrics
CREATE TABLE IF NOT EXISTS sleipnir.metrics (
    path String,
    value Float64,
    timestamp Int64,
    time DateTime DEFAULT toDateTime(timestamp),
    date Date DEFAULT toDate(time)
)

-- partitioning and engine
ENGINE = MergeTree
PARTITION BY toYYYYMMDD(date)
ORDER BY (path, date, timestamp)

-- retention
TTL date + INTERVAL 90 DAY;
