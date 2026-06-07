CREATE TABLE IF NOT EXISTS sync_state (
    chain_id BIGINT PRIMARY KEY,
    last_synced_block BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);