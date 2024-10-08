CREATE TABLE assets_details (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    category VARCHAR(50) NOT NULL,
    name VARCHAR(250) NOT NULL,
    company_id BIGINT REFERENCES companies(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE investment_details (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    quantity double precision NOT NULL,
    cost numeric(15,4) NOT NULL,
    asset_id BIGINT NOT NULL REFERENCES assets_details(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE transactions_details (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    description TEXT,
    comment TEXT,
    file VARCHAR(250),
    investment_details_id BIGINT REFERENCES investment_details(id) ON DELETE CASCADE,
    fee numeric(15,4) NOT NULL,
    original_amount numeric(15,4) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE accounts (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    currency_id BIGINT NOT NULL REFERENCES currencies(id) ON DELETE CASCADE,
    name VARCHAR(250) NOT NULL,
    category VARCHAR(250) NOT NULL,
    company VARCHAR(250) NOT NULL,
    description TEXT,
    amount numeric(15,4) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE fees (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    description TEXT,
    active bool NOT NULL DEFAULT true,
    percentage bool NOT NULL,
    account_id BIGINT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    recurrence VARCHAR(250) NOT NULL,
    amount numeric(15,4) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE rates_return (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    description TEXT,
    active bool NOT NULL DEFAULT true,
    percentage bool NOT NULL,
    account_id BIGINT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    recurrence VARCHAR(250) NOT NULL,
    amount numeric(15,4) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE transactions (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    details_id BIGINT NOT NULL REFERENCES transactions_details(id) ON DELETE CASCADE,
    account_id BIGINT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    exchange_rate_id BIGINT REFERENCES exchange_rates(id) ON DELETE CASCADE,
    "date" DATE NOT NULL,
    amount numeric(15,4) NOT NULL,
    category VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
