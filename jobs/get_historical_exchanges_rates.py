from collections.abc import Iterator
from concurrent.futures import ThreadPoolExecutor, as_completed
import datetime
from pandas import read_html, date_range
import polars as pl
import os
import logging

import tqdm

logger = logging.getLogger(__name__)
logging.basicConfig(filename="jobs.log", level=logging.INFO)


def download_file(code: str, dates: list[str]):
    code = code.strip()
    for date in dates:
        url = f"https://www.x-rates.com/historical/?from={code}&amount=1&date={date}"
        try:
            _, df = read_html(url)
            os.makedirs(f"data/exchange_rates/{code}", exist_ok=True)
            df.to_parquet(f"data/exchange_rates/{code}/{date}.parquet")
        except Exception as e:
            logger.error("%s %s", e, url)
            return


def get_product_dates_codes():
    new_db = "postgresql://lucas:Cars2389@0.0.0.0:5432/rust_backend"
    dates = list(
        d.date().strftime("%Y-%m-%d")
        for d in date_range("2014-01-01", datetime.date.today())
    )
    alphabetic_codes_rows: Iterator[tuple[str, ...]] = (
        pl.read_database_uri("SELECT alphabetic_code, name FROM currencies", new_db)
        .filter(pl.col("name").str.len_chars() != 0)
        .select("alphabetic_code")
        .iter_rows()
    )
    for (code,) in alphabetic_codes_rows:
        yield code, dates


def _main():
    # TODO: we got locked out
    with ThreadPoolExecutor() as pool:
        futures = (
            pool.submit(download_file, code, dates)
            for code, dates in get_product_dates_codes()
        )
        list(tqdm.tqdm(as_completed(futures), total=645624))


if __name__ == "__main__":
    _main()
