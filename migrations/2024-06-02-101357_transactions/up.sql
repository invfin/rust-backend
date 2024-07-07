CREATE TABLE transactions_files (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE assets_details (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    type VARCHAR(50) NOT NULL,
    name VARCHAR(250) NOT NULL,
    company_id BIGINT REFERENCES companies(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE investment_details (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    fee numeric NOT NULL,
    quantity numeric NOT NULL,
    cost numeric NOT NULL,
    amount numeric NOT NULL,
    date TIMESTAMP NOT NULL,
    currency_id BIGINT NOT NULL REFERENCES currencies(id) ON DELETE CASCADE,
    amount_converted numeric NOT NULL,
    asset_id BIGINT NOT NULL REFERENCES assets_details(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE transactions_details (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    date TIMESTAMP NOT NULL,
    description TEXT,
    comment TEXT,
    file_id BIGINT REFERENCES transactions_files(id) ON DELETE CASCADE,
    currency_id BIGINT NOT NULL REFERENCES currencies(id) ON DELETE CASCADE,
    investment_details_id BIGINT REFERENCES investment_details(id) ON DELETE CASCADE,
    original_amount numeric NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE transactions (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    details_id BIGINT NOT NULL REFERENCES transactions_details(id) ON DELETE CASCADE,
    exchange_rate_id BIGINT REFERENCES exchange_rates(id) ON DELETE CASCADE,
    date TIMESTAMP NOT NULL,
    amount numeric NOT NULL,
    type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
