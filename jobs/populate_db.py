import polars as pl
from datetime import datetime

old_db = "postgresql://lucas:Cars2389@0.0.0.0:5432/prod"
new_db = "postgresql://lucas:Cars2389@0.0.0.0:5432/rust_backend"


def transform_countries():
    return [
        pl.col("id"),
        pl.col("name").alias("name"),
        pl.col("iso").alias("iso"),
        pl.col("alpha_2_code").alias("alpha_2_code"),
        pl.col("alpha_3_code").alias("alpha_3_code"),
        pl.lit(datetime.now()).alias("created_at"),
        pl.lit(datetime.now()).alias("updated_at"),
    ]


def transform_companies():
    return [
        pl.col("id"),
        pl.col("ticker").alias("ticker"),
        pl.col("name").alias("name"),
        pl.col("website").alias("website"),
        pl.col("state").alias("state"),
        pl.col("ceo").alias("ceo"),
        pl.col("image").alias("image"),
        pl.col("city").alias("city"),
        pl.col("employees").alias("employees"),
        pl.col("address").alias("address"),
        pl.col("zip_code").alias("zip_code"),
        pl.col("cik").alias("cik"),
        pl.col("cusip").alias("cusip"),
        pl.col("isin").alias("isin"),
        pl.col("description").alias("description"),
        pl.col("ipoDate")
        .str.to_date(format="%Y-%m-%d", strict=False)
        .alias("ipo_date"),
        pl.col("country_id").alias("country_id"),
        pl.col("exchange_id").alias("exchange_id"),
        pl.col("industry_id").alias("industry_id"),
        pl.col("sector_id").alias("sector_id"),
        pl.col("is_adr").alias("is_adr"),
        pl.col("is_fund").alias("is_fund"),
        pl.lit(datetime.now()).alias("created_at"),
        pl.lit(datetime.now()).alias("updated_at"),
    ]


def run_changes():
    for old_table, selection in (
        ("assets_countries", transform_countries()),
        (
            "assets_currencies",
            (
                pl.col("currency").alias("alphabetic_code"),
                pl.col("iso").alias("numeric_code"),
                pl.all().exclude("currency", "iso", "spanish_name", "accronym"),
            ),
        ),
        ("assets_sectors", (pl.col("id"), pl.col("sector").alias("name"))),
        ("assets_industries", (pl.col("id"), pl.col("industry").alias("name"))),
        (
            "assets_exchanges",
            (
                pl.col("exchange_ticker").alias("ticker"),
                pl.col("id"),
                pl.col("country_id"),
                pl.col("exchange").alias("name"),
                pl.lit("").alias("image"),
            ),
        ),
        ("assets_companies", transform_companies()),
    ):
        (
            pl.read_database_uri(f"SELECT * FROM {old_table}", old_db)
            .select(selection)
            .fill_null("")
            .write_database(
                table_name=old_table.replace("assets_", ""),
                connection=new_db,
                if_table_exists="append",
            )
        )


run_changes()
