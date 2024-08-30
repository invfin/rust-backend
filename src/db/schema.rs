// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int8,
        user_id -> Int8,
        currency_id -> Int8,
        #[max_length = 250]
        name -> Varchar,
        #[max_length = 250]
        category -> Varchar,
        #[max_length = 250]
        company -> Varchar,
        description -> Nullable<Text>,
        amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    assets_details (id) {
        id -> Int8,
        #[max_length = 50]
        category -> Varchar,
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
        accumulated_other_comprehensive_income_and_loss -> Float8,
        accounts_payable -> Float8,
        cash_and_cash_equivalents -> Float8,
        cash_and_short_term_investments -> Float8,
        common_stocks -> Float8,
        deferred_revenue -> Float8,
        deferred_revenue_non_current -> Float8,
        deferred_tax_liabilities_non_current -> Float8,
        goodwill -> Float8,
        goodwill_and_intangible_assets -> Float8,
        intangible_assets -> Float8,
        inventory -> Float8,
        long_term_debt -> Float8,
        long_term_investments -> Float8,
        net_debt -> Float8,
        net_receivables -> Float8,
        other_assets -> Float8,
        other_current_assets -> Float8,
        other_current_liabilities -> Float8,
        other_liabilities -> Float8,
        other_non_current_assets -> Float8,
        other_non_current_liabilities -> Float8,
        other_total_stockholders_equity -> Float8,
        preferred_stocks -> Float8,
        property_plant_and_equipment -> Float8,
        retained_earnings -> Float8,
        short_term_debt -> Float8,
        short_term_investments -> Float8,
        tax_assets -> Float8,
        tax_payables -> Float8,
        total_assets -> Float8,
        total_current_assets -> Float8,
        total_current_liabilities -> Float8,
        total_debt -> Float8,
        total_investments -> Float8,
        total_liabilities -> Float8,
        total_liabilities_and_total_equity -> Float8,
        total_non_current_assets -> Float8,
        total_non_current_liabilities -> Float8,
        total_stockholders_equity -> Float8,
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
        acquisitions_net -> Float8,
        accounts_payable -> Float8,
        accounts_receivable -> Float8,
        capital_expenditures -> Float8,
        cash_beginning_period -> Float8,
        cash_end_period -> Float8,
        change_in_working_capital -> Float8,
        common_stock_issued -> Float8,
        common_stock_repurchased -> Float8,
        debt_repayment -> Float8,
        deferred_income_tax -> Float8,
        depreciation_and_amortization -> Float8,
        dividends_paid -> Float8,
        effect_of_forex_exchange -> Float8,
        financing_activities_cash_flow -> Float8,
        free_cash_flow -> Float8,
        inventory -> Float8,
        investing_activities_cash_flow -> Float8,
        investments_in_property_plant_and_equipment -> Float8,
        net_change_in_cash -> Float8,
        net_income -> Float8,
        operating_activities_cash_flow -> Float8,
        other_financing_activities -> Float8,
        other_investing_activities -> Float8,
        other_non_cash_items -> Float8,
        other_working_capital -> Float8,
        purchases_of_investments -> Float8,
        sales_and_maturities_of_investments -> Float8,
        stock_based_compensation -> Float8,
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
        capital_expenditure_growth -> Float8,
        cost_of_revenue_growth -> Float8,
        earnings_per_share_growth -> Float8,
        free_cash_flow_growth -> Float8,
        net_income_growth -> Float8,
        operating_expenses_growth -> Float8,
        owners_earnings_growth -> Float8,
        research_and_development_expenses_growth -> Float8,
        revenue_growth -> Float8,
        shares_buyback -> Float8,
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
    dashboard (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        author_id -> Int8,
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
        accounts_payable_turnover -> Float8,
        asset_turnover -> Float8,
        cash_conversion_cycle -> Float8,
        cash_conversion_ratio -> Float8,
        days_inventory_outstanding -> Float8,
        days_payables_outstanding -> Float8,
        days_sales_outstanding -> Float8,
        fixed_asset_turnover -> Float8,
        free_cash_flow_to_operating_cash_flow -> Float8,
        inventory_turnover -> Float8,
        operating_cycle -> Float8,
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
        company_equity_multiplier -> Float8,
        enterprise_value -> Float8,
        enterprise_value_to_free_cash_flow -> Float8,
        enterprise_value_to_operating_cash_flow -> Float8,
        enterprise_value_to_sales -> Float8,
        enterprise_value_multiple -> Float8,
        market_capitalization -> Float8,
    }
}

diesel::table! {
    exchange_rates (id) {
        id -> Int8,
        base_id -> Int8,
        target_id -> Int8,
        #[max_length = 255]
        conversion_rate -> Varchar,
        precision -> Int4,
        scale -> Int4,
        date -> Date,
        #[max_length = 255]
        source -> Nullable<Varchar>,
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
    fees (id) {
        id -> Int8,
        description -> Nullable<Text>,
        active -> Bool,
        percentage -> Bool,
        account_id -> Int8,
        #[max_length = 250]
        recurrence -> Varchar,
        amount -> Numeric,
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
        free_cash_flow -> Float8,
        free_cash_flow_equity -> Float8,
        unlevered_free_cash_flow -> Float8,
        unlevered_free_cash_flow_ebit -> Float8,
        owners_earnings -> Float8,
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
        cost_and_expenses -> Float8,
        cost_of_revenue -> Float8,
        depreciation_and_amortization -> Float8,
        earnings_before_interest_taxes_depreciation_and_amortization -> Float8,
        general_and_administrative_expenses -> Float8,
        gross_profit -> Float8,
        income_before_tax -> Float8,
        income_tax_expenses -> Float8,
        interest_expense -> Float8,
        net_income -> Float8,
        net_total_other_income_and_expenses -> Float8,
        operating_expenses -> Float8,
        operating_income -> Float8,
        other_expenses -> Float8,
        research_and_development_expenses -> Float8,
        revenue -> Float8,
        selling_and_marketing_expenses -> Float8,
        selling_general_and_administrative_expenses -> Float8,
        weighted_average_diluted_shares_outstanding -> Float8,
        weighted_average_shares_outstanding -> Float8,
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
        date -> Date,
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
        cash_ratio -> Float8,
        current_ratio -> Float8,
        debt_to_equity_ratio -> Float8,
        operating_cash_flow_ratio -> Float8,
        quick_ratio -> Float8,
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
        free_cash_flow_equity_to_net_income -> Float8,
        free_cash_flow_margin -> Float8,
        gross_margin -> Float8,
        net_income_margin -> Float8,
        owners_earnings_to_net_income -> Float8,
        unlevered_free_cash_flow_to_net_income -> Float8,
        unlevered_free_cash_flow_to_operating_income -> Float8,
        unlevered_free_cash_flow_ebit_to_net_income -> Float8,
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
        average_accounts_payable -> Float8,
        average_inventory -> Float8,
        dividend_yield -> Float8,
        earnings_yield -> Float8,
        effective_tax_rate -> Float8,
        free_cash_flow_yield -> Float8,
        income_quality -> Float8,
        invested_capital -> Float8,
        market_capitalization -> Float8,
        net_current_asset_value -> Float8,
        net_operating_profit_after_tax -> Float8,
        normalized_income -> Float8,
        payout_ratio -> Float8,
        retention_ratio -> Float8,
        tangible_assets -> Float8,
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
        asset_coverage_ratio -> Float8,
        cash_coverage -> Float8,
        cash_flow_coverage_ratios -> Float8,
        debt_ratio -> Float8,
        debt_service_coverage -> Float8,
        interest_coverage -> Float8,
        long_term_debt_to_capitalization -> Float8,
        operating_cash_flow_ratio -> Float8,
        total_debt_to_capitalization -> Float8,
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
        book_value_per_share -> Float8,
        capital_expenditure_per_share -> Float8,
        cash_per_share -> Float8,
        earnings_per_share -> Float8,
        free_cash_flow_per_share -> Float8,
        operating_cash_flow_per_share -> Float8,
        sales_per_share -> Float8,
        tangible_book_value_per_share -> Float8,
        total_assets_per_share -> Float8,
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
        price_to_book_value -> Float8,
        price_to_cash_flow -> Float8,
        price_to_earnings -> Float8,
        price_to_earnings_growth -> Float8,
        price_to_free_cash_flow -> Float8,
        price_to_operating_cash_flow -> Float8,
        price_to_sales -> Float8,
        price_to_tangible_assets -> Float8,
        price_to_total_assets -> Float8,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int8,
        user_id -> Int8,
        currency_id -> Int8,
        country_id -> Int8,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rates_return (id) {
        id -> Int8,
        description -> Nullable<Text>,
        active -> Bool,
        percentage -> Bool,
        account_id -> Int8,
        #[max_length = 250]
        recurrence -> Varchar,
        amount -> Numeric,
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
        nopat_roic -> Float8,
        return_on_assets -> Float8,
        return_on_capital -> Float8,
        return_on_common_equity -> Float8,
        return_on_equity -> Float8,
        return_on_invested_capital -> Float8,
        return_on_tangible_assets -> Float8,
        return_on_total_assets -> Float8,
        rogic -> Float8,
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
        account_id -> Int8,
        exchange_rate_id -> Nullable<Int8>,
        date -> Date,
        amount -> Numeric,
        #[max_length = 50]
        category -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions_details (id) {
        id -> Int8,
        description -> Nullable<Text>,
        comment -> Nullable<Text>,
        #[max_length = 250]
        file -> Nullable<Varchar>,
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

diesel::table! {
    widget (id) {
        id -> Int8,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 255]
        component -> Varchar,
        #[max_length = 255]
        source -> Varchar,
        author_id -> Int8,
        dashboard_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> currencies (currency_id));
diesel::joinable!(accounts -> users (user_id));
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
diesel::joinable!(dashboard -> users (author_id));
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
diesel::joinable!(fees -> accounts (account_id));
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
diesel::joinable!(profiles -> countries (country_id));
diesel::joinable!(profiles -> currencies (currency_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(rates_return -> accounts (account_id));
diesel::joinable!(rentability_ratios -> companies (company_id));
diesel::joinable!(rentability_ratios -> currencies (reported_currency_id));
diesel::joinable!(rentability_ratios -> periods (period_id));
diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> exchange_rates (exchange_rate_id));
diesel::joinable!(transactions -> transactions_details (details_id));
diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(transactions_details -> currencies (currency_id));
diesel::joinable!(transactions_details -> investment_details (investment_details_id));
diesel::joinable!(transactions_files -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    assets_details,
    balance_sheet_statements,
    cashflow_statements,
    companies,
    company_growth,
    countries,
    currencies,
    currencies_countries_m2m,
    dashboard,
    definitions,
    definitions_categories,
    definitions_categories_m2m,
    definitions_content,
    efficiency_ratios,
    enterprise_value_ratios,
    exchange_rates,
    exchanges,
    fees,
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
    rates_return,
    rentability_ratios,
    sectors,
    transactions,
    transactions_details,
    transactions_files,
    users,
    widget,
);
