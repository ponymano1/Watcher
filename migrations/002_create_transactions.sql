CREATE TABLE IF NOT EXISTS transactions (
    id BIGSERIAL PRIMARY KEY,
    chain_id BIGINT NOT NULL,
    tx_hash TEXT NOT NULL,
    block_number BIGINT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT,
    value_wei TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(chain_id, tx_hash)
);