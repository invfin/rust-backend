// @generated automatically by Diesel CLI.

diesel::table! {
    assets_details (id) {
        id -> Int8,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 250]
        name -> Varchar,
        company_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    balance_sheet_statements (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        #[max_length = 255]
        link -> Varchar,
        #[max_length = 255]
        final_link -> Varchar,
        date -> Date,
        accumulated_other_comprehensive_income_and_loss -> Nullable<Numeric>,
        accounts_payable -> Nullable<Numeric>,
        cash_and_cash_equivalents -> Nullable<Numeric>,
        cash_and_short_term_investments -> Nullable<Numeric>,
        common_stocks -> Nullable<Numeric>,
        deferred_revenue -> Nullable<Numeric>,
        deferred_revenue_non_current -> Nullable<Numeric>,
        deferred_tax_liabilities_non_current -> Nullable<Numeric>,
        goodwill -> Nullable<Numeric>,
        goodwill_and_intangible_assets -> Nullable<Numeric>,
        intangible_assets -> Nullable<Numeric>,
        inventory -> Nullable<Numeric>,
        long_term_debt -> Nullable<Numeric>,
        long_term_investments -> Nullable<Numeric>,
        net_debt -> Nullable<Numeric>,
        net_receivables -> Nullable<Numeric>,
        other_assets -> Nullable<Numeric>,
        other_current_assets -> Nullable<Numeric>,
        other_current_liabilities -> Nullable<Numeric>,
        other_liabilities -> Nullable<Numeric>,
        other_non_current_assets -> Nullable<Numeric>,
        other_non_current_liabilities -> Nullable<Numeric>,
        other_total_stockholders_equity -> Nullable<Numeric>,
        preferred_stocks -> Nullable<Numeric>,
        property_plant_and_equipment -> Nullable<Numeric>,
        retained_earnings -> Nullable<Numeric>,
        short_term_debt -> Nullable<Numeric>,
        short_term_investments -> Nullable<Numeric>,
        tax_assets -> Nullable<Numeric>,
        tax_payables -> Nullable<Numeric>,
        total_assets -> Nullable<Numeric>,
        total_current_assets -> Nullable<Numeric>,
        total_current_liabilities -> Nullable<Numeric>,
        total_debt -> Nullable<Numeric>,
        total_investments -> Nullable<Numeric>,
        total_liabilities -> Nullable<Numeric>,
        total_liabilities_and_total_equity -> Nullable<Numeric>,
        total_non_current_assets -> Nullable<Numeric>,
        total_non_current_liabilities -> Nullable<Numeric>,
        total_stockholders_equity -> Nullable<Numeric>,
    }
}

diesel::table! {
    cashflow_statements (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        #[max_length = 255]
        link -> Varchar,
        #[max_length = 255]
        final_link -> Varchar,
        date -> Date,
        acquisitions_net -> Nullable<Numeric>,
        accounts_payable -> Nullable<Numeric>,
        accounts_receivable -> Nullable<Numeric>,
        capital_expenditures -> Nullable<Numeric>,
        cash_beginning_period -> Nullable<Numeric>,
        cash_end_period -> Nullable<Numeric>,
        change_in_working_capital -> Nullable<Numeric>,
        common_stock_issued -> Nullable<Numeric>,
        common_stock_repurchased -> Nullable<Numeric>,
        debt_repayment -> Nullable<Numeric>,
        deferred_income_tax -> Nullable<Numeric>,
        depreciation_and_amortization -> Nullable<Numeric>,
        dividends_paid -> Nullable<Numeric>,
        effect_of_forex_exchange -> Nullable<Numeric>,
        financing_activities_cash_flow -> Nullable<Numeric>,
        free_cash_flow -> Nullable<Numeric>,
        inventory -> Nullable<Numeric>,
        investing_activities_cash_flow -> Nullable<Numeric>,
        investments_in_property_plant_and_equipment -> Nullable<Numeric>,
        net_change_in_cash -> Nullable<Numeric>,
        net_income -> Nullable<Numeric>,
        operating_activities_cash_flow -> Nullable<Numeric>,
        other_financing_activities -> Nullable<Numeric>,
        other_investing_activities -> Nullable<Numeric>,
        other_non_cash_items -> Nullable<Numeric>,
        other_working_capital -> Nullable<Numeric>,
        purchases_of_investments -> Nullable<Numeric>,
        sales_and_maturities_of_investments -> Nullable<Numeric>,
        stock_based_compensation -> Nullable<Numeric>,
    }
}

diesel::table! {
    companies (id) {
        id -> Int8,
        #[max_length = 255]
        ticker -> Varchar,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        website -> Nullable<Varchar>,
        #[max_length = 255]
        state -> Nullable<Varchar>,
        #[max_length = 255]
        ceo -> Nullable<Varchar>,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        employees -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        zip_code -> Nullable<Varchar>,
        #[max_length = 255]
        cik -> Nullable<Varchar>,
        #[max_length = 255]
        cusip -> Nullable<Varchar>,
        #[max_length = 255]
        isin -> Nullable<Varchar>,
        description -> Nullable<Text>,
        ipo_date -> Nullable<Date>,
        country_id -> Nullable<Int8>,
        exchange_id -> Nullable<Int8>,
        industry_id -> Nullable<Int8>,
        sector_id -> Nullable<Int8>,
        is_adr -> Bool,
        is_fund -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    company_growth (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        capital_expenditure_growth -> Numeric,
        cost_of_revenue_growth -> Numeric,
        earnings_per_share_growth -> Numeric,
        free_cash_flow_growth -> Numeric,
        net_income_growth -> Numeric,
        operating_expenses_growth -> Numeric,
        owners_earnings_growth -> Numeric,
        research_and_development_expenses_growth -> Numeric,
        revenue_growth -> Numeric,
        shares_buyback -> Numeric,
    }
}

diesel::table! {
    countries (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 10]
        iso -> Varchar,
        #[max_length = 10]
        alpha_2_code -> Varchar,
        #[max_length = 10]
        alpha_3_code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 10]
        alphabetic_code -> Varchar,
        #[max_length = 10]
        numeric_code -> Varchar,
        #[max_length = 10]
        symbol -> Varchar,
        decimals -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    currencies_countries_m2m (id) {
        id -> Int8,
        currency_id -> Int8,
        country_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    definitions (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        resume -> Text,
        #[max_length = 255]
        slug -> Varchar,
        created_at -> Timestamp,
        total_views -> Int8,
        updated_at -> Timestamp,
        published_at -> Timestamp,
        total_votes -> Int8,
        author_id -> Int8,
        #[max_length = 255]
        thumbnail -> Nullable<Varchar>,
    }
}

diesel::table! {
    definitions_categories (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    definitions_categories_m2m (id) {
        id -> Int8,
        definition_id -> Int8,
        category_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    definitions_content (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Timestamp,
        author_id -> Int8,
        definition_id -> Int8,
        order -> Int8,
        content -> Text,
    }
}

diesel::table! {
    efficiency_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        accounts_payable_turnover -> Numeric,
        asset_turnover -> Numeric,
        cash_conversion_cycle -> Numeric,
        cash_conversion_ratio -> Numeric,
        days_inventory_outstanding -> Numeric,
        days_payables_outstanding -> Numeric,
        days_sales_outstanding -> Numeric,
        fixed_asset_turnover -> Numeric,
        free_cash_flow_to_operating_cash_flow -> Numeric,
        inventory_turnover -> Numeric,
        operating_cycle -> Numeric,
    }
}

diesel::table! {
    enterprise_value_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        company_equity_multiplier -> Numeric,
        enterprise_value -> Numeric,
        enterprise_value_to_free_cash_flow -> Numeric,
        enterprise_value_to_operating_cash_flow -> Numeric,
        enterprise_value_to_sales -> Numeric,
        enterprise_value_multiple -> Numeric,
        market_capitalization -> Numeric,
    }
}

diesel::table! {
    exchange_rates (id) {
        id -> Int8,
        base_id -> Int8,
        target_id -> Int8,
        conversion_rate -> Numeric,
        date -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exchanges (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        ticker -> Varchar,
        country_id -> Nullable<Int8>,
        #[max_length = 255]
        image -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    free_cashflow_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        free_cash_flow -> Numeric,
        free_cash_flow_equity -> Numeric,
        unlevered_free_cash_flow -> Numeric,
        unlevered_free_cash_flow_ebit -> Numeric,
        owners_earnings -> Numeric,
    }
}

diesel::table! {
    income_statements (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        #[max_length = 255]
        link -> Varchar,
        #[max_length = 255]
        final_link -> Varchar,
        date -> Date,
        cost_and_expenses -> Nullable<Numeric>,
        cost_of_revenue -> Nullable<Numeric>,
        depreciation_and_amortization -> Nullable<Numeric>,
        earnings_before_interest_taxes_depreciation_and_amortization -> Nullable<Numeric>,
        general_and_administrative_expenses -> Nullable<Numeric>,
        gross_profit -> Nullable<Numeric>,
        income_before_tax -> Nullable<Numeric>,
        income_tax_expenses -> Nullable<Numeric>,
        interest_expense -> Nullable<Numeric>,
        net_income -> Nullable<Numeric>,
        net_total_other_income_and_expenses -> Nullable<Numeric>,
        operating_expenses -> Nullable<Numeric>,
        operating_income -> Nullable<Numeric>,
        other_expenses -> Nullable<Numeric>,
        research_and_development_expenses -> Nullable<Numeric>,
        revenue -> Nullable<Numeric>,
        selling_and_marketing_expenses -> Nullable<Numeric>,
        selling_general_and_administrative_expenses -> Nullable<Numeric>,
        weighted_average_diluted_shares_outstanding -> Nullable<Numeric>,
        weighted_average_shares_outstanding -> Nullable<Numeric>,
    }
}

diesel::table! {
    industries (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    investment_details (id) {
        id -> Int8,
        fee -> Numeric,
        quantity -> Numeric,
        cost -> Numeric,
        amount -> Numeric,
        date -> Timestamp,
        currency_id -> Int8,
        amount_converted -> Numeric,
        asset_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    liquidity_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        cash_ratio -> Numeric,
        current_ratio -> Numeric,
        debt_to_equity_ratio -> Numeric,
        operating_cash_flow_ratio -> Numeric,
        quick_ratio -> Numeric,
    }
}

diesel::table! {
    margin_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        free_cash_flow_equity_to_net_income -> Numeric,
        free_cash_flow_margin -> Numeric,
        gross_margin -> Numeric,
        net_income_margin -> Numeric,
        owners_earnings_to_net_income -> Numeric,
        unlevered_free_cash_flow_to_net_income -> Numeric,
        unlevered_free_cash_flow_to_operating_income -> Numeric,
        unlevered_free_cash_flow_ebit_to_net_income -> Numeric,
    }
}

diesel::table! {
    non_gaap_figures (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        average_accounts_payable -> Numeric,
        average_inventory -> Numeric,
        dividend_yield -> Numeric,
        earnings_yield -> Numeric,
        effective_tax_rate -> Numeric,
        free_cash_flow_yield -> Numeric,
        income_quality -> Numeric,
        invested_capital -> Numeric,
        market_capitalization -> Numeric,
        net_current_asset_value -> Numeric,
        net_operating_profit_after_tax -> Numeric,
        normalized_income -> Numeric,
        payout_ratio -> Numeric,
        retention_ratio -> Numeric,
        tangible_assets -> Numeric,
    }
}

diesel::table! {
    operation_risk_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        asset_coverage_ratio -> Numeric,
        cash_coverage -> Numeric,
        cash_flow_coverage_ratios -> Numeric,
        debt_ratio -> Numeric,
        debt_service_coverage -> Numeric,
        interest_coverage -> Numeric,
        long_term_debt_to_capitalization -> Numeric,
        operating_cash_flow_ratio -> Numeric,
        total_debt_to_capitalization -> Numeric,
    }
}

diesel::table! {
    per_share_values (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        book_value_per_share -> Numeric,
        capital_expenditure_per_share -> Numeric,
        cash_per_share -> Numeric,
        earnings_per_share -> Numeric,
        free_cash_flow_per_share -> Numeric,
        operating_cash_flow_per_share -> Numeric,
        sales_per_share -> Numeric,
        tangible_book_value_per_share -> Numeric,
        total_assets_per_share -> Numeric,
    }
}

diesel::table! {
    periods (id) {
        id -> Int8,
        year -> Int4,
        period -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    price_to_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        price_to_book_value -> Numeric,
        price_to_cash_flow -> Numeric,
        price_to_earnings -> Numeric,
        price_to_earnings_growth -> Numeric,
        price_to_free_cash_flow -> Numeric,
        price_to_operating_cash_flow -> Nullable<Numeric>,
        price_to_sales -> Numeric,
        price_to_tangible_assets -> Numeric,
        price_to_total_assets -> Numeric,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int8,
        user_id -> Int8,
        first_name -> Varchar,
        last_name -> Varchar,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rentability_ratios (id) {
        id -> Int8,
        company_id -> Int8,
        reported_currency_id -> Nullable<Int8>,
        period_id -> Nullable<Int8>,
        is_ttm -> Bool,
        from_average -> Bool,
        date -> Date,
        nopat_roic -> Numeric,
        return_on_assets -> Numeric,
        return_on_capital -> Numeric,
        return_on_common_equity -> Numeric,
        return_on_equity -> Numeric,
        return_on_invested_capital -> Numeric,
        return_on_tangible_assets -> Numeric,
        return_on_total_assets -> Numeric,
        rogic -> Numeric,
    }
}

diesel::table! {
    sectors (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int8,
        user_id -> Int8,
        details_id -> Int8,
        exchange_rate_id -> Nullable<Int8>,
        date -> Timestamp,
        amount -> Numeric,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions_details (id) {
        id -> Int8,
        date -> Timestamp,
        description -> Nullable<Text>,
        comment -> Nullable<Text>,
        file_id -> Nullable<Int8>,
        currency_id -> Int8,
        investment_details_id -> Nullable<Int8>,
        original_amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions_files (id) {
        id -> Int8,
        user_id -> Int8,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        path -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Varchar,
        is_active -> Bool,
        is_superuser -> Bool,
        is_staff -> Bool,
        is_test -> Bool,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(assets_details -> companies (company_id));
diesel::joinable!(balance_sheet_statements -> companies (company_id));
diesel::joinable!(balance_sheet_statements -> currencies (reported_currency_id));
diesel::joinable!(balance_sheet_statements -> periods (period_id));
diesel::joinable!(cashflow_statements -> companies (company_id));
diesel::joinable!(cashflow_statements -> currencies (reported_currency_id));
diesel::joinable!(cashflow_statements -> periods (period_id));
diesel::joinable!(companies -> countries (country_id));
diesel::joinable!(companies -> exchanges (exchange_id));
diesel::joinable!(companies -> industries (industry_id));
diesel::joinable!(companies -> sectors (sector_id));
diesel::joinable!(company_growth -> companies (company_id));
diesel::joinable!(company_growth -> currencies (reported_currency_id));
diesel::joinable!(company_growth -> periods (period_id));
diesel::joinable!(currencies_countries_m2m -> countries (country_id));
diesel::joinable!(currencies_countries_m2m -> currencies (currency_id));
diesel::joinable!(definitions -> users (author_id));
diesel::joinable!(definitions_categories_m2m -> definitions (definition_id));
diesel::joinable!(definitions_categories_m2m -> definitions_categories (category_id));
diesel::joinable!(definitions_content -> definitions (definition_id));
diesel::joinable!(definitions_content -> users (author_id));
diesel::joinable!(efficiency_ratios -> companies (company_id));
diesel::joinable!(efficiency_ratios -> currencies (reported_currency_id));
diesel::joinable!(efficiency_ratios -> periods (period_id));
diesel::joinable!(enterprise_value_ratios -> companies (company_id));
diesel::joinable!(enterprise_value_ratios -> currencies (reported_currency_id));
diesel::joinable!(enterprise_value_ratios -> periods (period_id));
diesel::joinable!(exchanges -> countries (country_id));
diesel::joinable!(free_cashflow_ratios -> companies (company_id));
diesel::joinable!(free_cashflow_ratios -> currencies (reported_currency_id));
diesel::joinable!(free_cashflow_ratios -> periods (period_id));
diesel::joinable!(income_statements -> companies (company_id));
diesel::joinable!(income_statements -> currencies (reported_currency_id));
diesel::joinable!(income_statements -> periods (period_id));
diesel::joinable!(investment_details -> assets_details (asset_id));
diesel::joinable!(investment_details -> currencies (currency_id));
diesel::joinable!(liquidity_ratios -> companies (company_id));
diesel::joinable!(liquidity_ratios -> currencies (reported_currency_id));
diesel::joinable!(liquidity_ratios -> periods (period_id));
diesel::joinable!(margin_ratios -> companies (company_id));
diesel::joinable!(margin_ratios -> currencies (reported_currency_id));
diesel::joinable!(margin_ratios -> periods (period_id));
diesel::joinable!(non_gaap_figures -> companies (company_id));
diesel::joinable!(non_gaap_figures -> currencies (reported_currency_id));
diesel::joinable!(non_gaap_figures -> periods (period_id));
diesel::joinable!(operation_risk_ratios -> companies (company_id));
diesel::joinable!(operation_risk_ratios -> currencies (reported_currency_id));
diesel::joinable!(operation_risk_ratios -> periods (period_id));
diesel::joinable!(per_share_values -> companies (company_id));
diesel::joinable!(per_share_values -> currencies (reported_currency_id));
diesel::joinable!(per_share_values -> periods (period_id));
diesel::joinable!(price_to_ratios -> companies (company_id));
diesel::joinable!(price_to_ratios -> currencies (reported_currency_id));
diesel::joinable!(price_to_ratios -> periods (period_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(rentability_ratios -> companies (company_id));
diesel::joinable!(rentability_ratios -> currencies (reported_currency_id));
diesel::joinable!(rentability_ratios -> periods (period_id));
diesel::joinable!(transactions -> exchange_rates (exchange_rate_id));
diesel::joinable!(transactions -> transactions_details (details_id));
diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(transactions_details -> currencies (currency_id));
diesel::joinable!(transactions_details -> investment_details (investment_details_id));
diesel::joinable!(transactions_details -> transactions_files (file_id));
diesel::joinable!(transactions_files -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    assets_details,
    balance_sheet_statements,
    cashflow_statements,
    companies,
    company_growth,
    countries,
    currencies,
    currencies_countries_m2m,
    definitions,
    definitions_categories,
    definitions_categories_m2m,
    definitions_content,
    efficiency_ratios,
    enterprise_value_ratios,
    exchange_rates,
    exchanges,
    free_cashflow_ratios,
    income_statements,
    industries,
    investment_details,
    liquidity_ratios,
    margin_ratios,
    non_gaap_figures,
    operation_risk_ratios,
    per_share_values,
    periods,
    price_to_ratios,
    profiles,
    rentability_ratios,
    sectors,
    transactions,
    transactions_details,
    transactions_files,
    users,
);
