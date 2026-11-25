CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    pass_hash TEXT NOT NULL,
    role user_role NOT NULL
);

CREATE TABLE nodes (
    id UUID PRIMARY KEY,
    continent TEXT NOT NULL,
    country TEXT NOT NULL,
    region TEXT NOT NULL,
    city TEXT NOT NULL,
    district TEXT NOT NULL,
    lat FLOAT8 NOT NULL,
    lon FLOAT8 NOT NULL,
    alt_m FLOAT4 NOT NULL,
    last_heartbeat TIMESTAMPTZ
);

CREATE TABLE logs (
    temp FLOAT4 NOT NULL,
    hum FlOAT4 NOT NULL,
    air_q FlOAT4 NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    node_id UUID NOT NULL REFERENCES nodes(id),
    PRIMARY KEY (node_id, timestamp)
);

CREATE INDEX logs_idx ON logs(timestamp DESC);
