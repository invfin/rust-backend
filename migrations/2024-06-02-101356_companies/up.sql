CREATE TABLE income_statements (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    link VARCHAR(255) NOT NULL DEFAULT '',
    final_link VARCHAR(255) NOT NULL DEFAULT '',
    date DATE NOT NULL,
    cost_and_expenses double precision NOT NULL,
    cost_of_revenue double precision NOT NULL,
    depreciation_and_amortization double precision NOT NULL,
    earnings_before_interest_taxes_depreciation_and_amortization double precision NOT NULL,
    general_and_administrative_expenses double precision NOT NULL,
    gross_profit double precision NOT NULL,
    income_before_tax double precision NOT NULL,
    income_tax_expenses double precision NOT NULL,
    interest_expense double precision NOT NULL,
    net_income double precision NOT NULL,
    net_total_other_income_and_expenses double precision NOT NULL,
    operating_expenses double precision NOT NULL,
    operating_income double precision NOT NULL,
    other_expenses double precision NOT NULL,
    research_and_development_expenses double precision NOT NULL,
    revenue double precision NOT NULL,
    selling_and_marketing_expenses double precision NOT NULL,
    selling_general_and_administrative_expenses double precision NOT NULL,
    weighted_average_diluted_shares_outstanding double precision NOT NULL,
    weighted_average_shares_outstanding double precision NOT NULL
);

CREATE TABLE balance_sheet_statements (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    link VARCHAR(255) NOT NULL DEFAULT '',
    final_link VARCHAR(255) NOT NULL DEFAULT '',
    date DATE NOT NULL,
    accumulated_other_comprehensive_income_and_loss double precision NOT NULL,
    accounts_payable double precision NOT NULL,
    cash_and_cash_equivalents double precision NOT NULL,
    cash_and_short_term_investments double precision NOT NULL,
    common_stocks double precision NOT NULL,
    deferred_revenue double precision NOT NULL,
    deferred_revenue_non_current double precision NOT NULL,
    deferred_tax_liabilities_non_current double precision NOT NULL,
    goodwill double precision NOT NULL,
    goodwill_and_intangible_assets double precision NOT NULL,
    intangible_assets double precision NOT NULL,
    inventory double precision NOT NULL,
    long_term_debt double precision NOT NULL,
    long_term_investments double precision NOT NULL,
    net_debt double precision NOT NULL,
    net_receivables double precision NOT NULL,
    other_assets double precision NOT NULL,
    other_current_assets double precision NOT NULL,
    other_current_liabilities double precision NOT NULL,
    other_liabilities double precision NOT NULL,
    other_non_current_assets double precision NOT NULL,
    other_non_current_liabilities double precision NOT NULL,
    other_total_stockholders_equity double precision NOT NULL,
    preferred_stocks double precision NOT NULL,
    property_plant_and_equipment double precision NOT NULL,
    retained_earnings double precision NOT NULL,
    short_term_debt double precision NOT NULL,
    short_term_investments double precision NOT NULL,
    tax_assets double precision NOT NULL,
    tax_payables double precision NOT NULL,
    total_assets double precision NOT NULL,
    total_current_assets double precision NOT NULL,
    total_current_liabilities double precision NOT NULL,
    total_debt double precision NOT NULL,
    total_investments double precision NOT NULL,
    total_liabilities double precision NOT NULL,
    total_liabilities_and_total_equity double precision NOT NULL,
    total_non_current_assets double precision NOT NULL,
    total_non_current_liabilities double precision NOT NULL,
    total_stockholders_equity double precision NOT NULL
);

CREATE TABLE cashflow_statements (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    link VARCHAR(255) NOT NULL DEFAULT '',
    final_link VARCHAR(255) NOT NULL DEFAULT '',
    date DATE NOT NULL,
    acquisitions_net double precision NOT NULL,
    accounts_payable double precision NOT NULL,
    accounts_receivable double precision NOT NULL,
    capital_expenditures double precision NOT NULL,
    cash_beginning_period double precision NOT NULL,
    cash_end_period double precision NOT NULL,
    change_in_working_capital double precision NOT NULL,
    common_stock_issued double precision NOT NULL,
    common_stock_repurchased double precision NOT NULL,
    debt_repayment double precision NOT NULL,
    deferred_income_tax double precision NOT NULL,
    depreciation_and_amortization double precision NOT NULL,
    dividends_paid double precision NOT NULL,
    effect_of_forex_exchange double precision NOT NULL,
    financing_activities_cash_flow double precision NOT NULL,
    free_cash_flow double precision NOT NULL,
    inventory double precision NOT NULL,
    investing_activities_cash_flow double precision NOT NULL,
    investments_in_property_plant_and_equipment double precision NOT NULL,
    net_change_in_cash double precision NOT NULL,
    net_income double precision NOT NULL,
    operating_activities_cash_flow double precision NOT NULL,
    other_financing_activities double precision NOT NULL,
    other_investing_activities double precision NOT NULL,
    other_non_cash_items double precision NOT NULL,
    other_working_capital double precision NOT NULL,
    purchases_of_investments double precision NOT NULL,
    sales_and_maturities_of_investments double precision NOT NULL,
    stock_based_compensation double precision NOT NULL
);

CREATE TABLE rentability_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    nopat_roic double precision NOT NULL,
    return_on_assets double precision NOT NULL,
    return_on_capital double precision NOT NULL,
    return_on_common_equity double precision NOT NULL,
    return_on_equity double precision NOT NULL,
    return_on_invested_capital double precision NOT NULL,
    return_on_tangible_assets double precision NOT NULL,
    return_on_total_assets double precision NOT NULL,
    rogic double precision NOT NULL
);

CREATE TABLE liquidity_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    cash_ratio double precision NOT NULL,
    current_ratio double precision NOT NULL,
    debt_to_equity_ratio double precision NOT NULL,
    operating_cash_flow_ratio double precision NOT NULL,
    quick_ratio double precision NOT NULL
);
CREATE TABLE margin_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    free_cash_flow_equity_to_net_income double precision NOT NULL,
    free_cash_flow_margin double precision NOT NULL,
    gross_margin double precision NOT NULL,
    net_income_margin double precision NOT NULL,
    owners_earnings_to_net_income double precision NOT NULL,
    unlevered_free_cash_flow_to_net_income double precision NOT NULL,
    unlevered_free_cash_flow_to_operating_income double precision NOT NULL,
    unlevered_free_cash_flow_ebit_to_net_income double precision NOT NULL
);
CREATE TABLE free_cashflow_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    free_cash_flow double precision NOT NULL,
    free_cash_flow_equity double precision NOT NULL,
    unlevered_free_cash_flow double precision NOT NULL,
    unlevered_free_cash_flow_ebit double precision NOT NULL,
    owners_earnings double precision NOT NULL
);

CREATE TABLE per_share_values (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    book_value_per_share double precision NOT NULL,
    capital_expenditure_per_share double precision NOT NULL,
    cash_per_share double precision NOT NULL,
    earnings_per_share double precision NOT NULL,
    free_cash_flow_per_share double precision NOT NULL,
    operating_cash_flow_per_share double precision NOT NULL,
    sales_per_share double precision NOT NULL,
    tangible_book_value_per_share double precision NOT NULL,
    total_assets_per_share double precision NOT NULL
);
CREATE TABLE non_gaap_figures (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    average_accounts_payable double precision NOT NULL,
    average_inventory double precision NOT NULL,
    dividend_yield double precision NOT NULL,
    earnings_yield double precision NOT NULL,
    effective_tax_rate double precision NOT NULL,
    free_cash_flow_yield double precision NOT NULL,
    income_quality double precision NOT NULL,
    invested_capital double precision NOT NULL,
    market_capitalization double precision NOT NULL,
    net_current_asset_value double precision NOT NULL,
    net_operating_profit_after_tax double precision NOT NULL,
    normalized_income double precision NOT NULL,
    payout_ratio double precision NOT NULL,
    retention_ratio double precision NOT NULL,
    tangible_assets double precision NOT NULL
);
CREATE TABLE operation_risk_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    asset_coverage_ratio double precision NOT NULL,
    cash_coverage double precision NOT NULL,
    cash_flow_coverage_ratios double precision NOT NULL,
    debt_ratio double precision NOT NULL,
    debt_service_coverage double precision NOT NULL,
    interest_coverage double precision NOT NULL,
    long_term_debt_to_capitalization double precision NOT NULL,
    operating_cash_flow_ratio double precision NOT NULL,
    total_debt_to_capitalization double precision NOT NULL
);
CREATE TABLE enterprise_value_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    company_equity_multiplier double precision NOT NULL,
    enterprise_value double precision NOT NULL,
    enterprise_value_to_free_cash_flow double precision NOT NULL,
    enterprise_value_to_operating_cash_flow double precision NOT NULL,
    enterprise_value_to_sales double precision NOT NULL,
    enterprise_value_multiple double precision NOT NULL,
    market_capitalization double precision NOT NULL
);
CREATE TABLE company_growth (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    capital_expenditure_growth double precision NOT NULL,
    cost_of_revenue_growth double precision NOT NULL,
    earnings_per_share_growth double precision NOT NULL,
    free_cash_flow_growth double precision NOT NULL,
    net_income_growth double precision NOT NULL,
    operating_expenses_growth double precision NOT NULL,
    owners_earnings_growth double precision NOT NULL,
    research_and_development_expenses_growth double precision NOT NULL,
    revenue_growth double precision NOT NULL,
    shares_buyback double precision NOT NULL
);
CREATE TABLE efficiency_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    accounts_payable_turnover double precision NOT NULL,
    asset_turnover double precision NOT NULL,
    cash_conversion_cycle double precision NOT NULL,
    cash_conversion_ratio double precision NOT NULL,
    days_inventory_outstanding double precision NOT NULL,
    days_payables_outstanding double precision NOT NULL,
    days_sales_outstanding double precision NOT NULL,
    fixed_asset_turnover double precision NOT NULL,
    free_cash_flow_to_operating_cash_flow double precision NOT NULL,
    inventory_turnover double precision NOT NULL,
    operating_cycle double precision NOT NULL
);
CREATE TABLE price_to_ratios (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
    reported_currency_id BIGINT REFERENCES currencies(id) ON DELETE SET NULL,
    period_id BIGINT REFERENCES periods(id) ON DELETE SET NULL,
    is_ttm BOOLEAN NOT NULL DEFAULT FALSE,
    from_average BOOLEAN NOT NULL DEFAULT FALSE,
    date DATE NOT NULL,
    price_to_book_value double precision NOT NULL,
    price_to_cash_flow double precision NOT NULL,
    price_to_earnings double precision NOT NULL,
    price_to_earnings_growth double precision NOT NULL,
    price_to_free_cash_flow double precision NOT NULL,
    price_to_operating_cash_flow double precision NOT NULL,
    price_to_sales double precision NOT NULL,
    price_to_tangible_assets double precision NOT NULL,
    price_to_total_assets double precision NOT NULL
);
