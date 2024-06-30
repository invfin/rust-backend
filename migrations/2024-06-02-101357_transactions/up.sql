CREATE TABLE incomes (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount numeric NOT NULL,
    date TIMESTAMP NOT NULL,
    description TEXT,
    comment TEXT,
    currency_id BIGINT NOT NULL REFERENCES currencies(id) ON DELETE CASCADE,
    amount_converted numeric NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE expenses (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount numeric NOT NULL,
    date TIMESTAMP NOT NULL,
    description TEXT,
    comment TEXT,
    currency_id BIGINT NOT NULL REFERENCES currencies(id) ON DELETE CASCADE,
    amount_converted numeric NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE investments (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    fee numeric NOT NULL,
    quantity numeric NOT NULL,
    cost numeric NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount numeric NOT NULL,
    date TIMESTAMP NOT NULL,
    description TEXT,
    comment TEXT,
    currency_id BIGINT NOT NULL REFERENCES currencies(id) ON DELETE CASCADE,
    amount_converted numeric NOT NULL,
    asset_type VARCHAR(255) NOT NULL,
    asset_id BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
