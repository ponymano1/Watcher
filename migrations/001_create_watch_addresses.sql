CREATE TABLE IF NOT EXISTS watch_addresses (
    id BIGSERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    chain_id BIGINT NOT NULL,
    label TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(address, chain_id)
);