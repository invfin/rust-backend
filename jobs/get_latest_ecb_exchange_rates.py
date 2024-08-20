import logging
from pathlib import Path
import polars as pl
import subprocess
import datetime
import aiohttp
import asyncio
from tqdm.asyncio import tqdm

logger = logging.getLogger(__name__)
logging.basicConfig(filename=f"{__name__}.log", level=logging.INFO)

TOKEN = ""


async def make_request(
    base: str,
    target: str,
    date: str,
    conversion_rate: str,
    precision: int,
    scale: int,
    client: aiohttp.ClientSession,
):
    data = {
        "base": base,
        "target": target,
        "conversion_rate": conversion_rate,
        "date": date,
        "source": "ECB",
        "precision": precision,
        "scale": scale,
    }
    response = await client.post("/api/v1/exchange_rates", json=data)
    try:
        response.raise_for_status()
    except aiohttp.ClientResponseError as e:
        logger.error("%s %s", e, data)
    else:
        d = await response.json()
        logger.info("%s", d)


async def insert_exchange_rates(file_name: str):
    rows = (
        pl.read_csv(file_name, ignore_errors=True)
        .filter(pl.col("FREQ").eq("D"))
        .select(
            "CURRENCY",
            "CURRENCY_DENOM",
            "TIME_PERIOD",
            pl.col("OBS_VALUE").cast(pl.Utf8),
            pl.col("OBS_VALUE").cast(pl.Utf8).str.split(".").alias("precision_scale"),
        )
        .select(
            "CURRENCY",
            "CURRENCY_DENOM",
            "TIME_PERIOD",
            "OBS_VALUE",
            (
                pl.col("precision_scale").list.first().str.len_chars()
                + pl.col("precision_scale").list.last().str.len_chars()
            ).alias("precision"),
            pl.col("precision_scale").list.last().str.len_chars().alias("scale"),
        )
        .iter_rows()
    )
    async with aiohttp.ClientSession(
        base_url="http://127.0.0.1:8000/",
        headers={
            "Authorization": f"Bearer {TOKEN}",
            "content-type": "application/json",
        },
    ) as client:
        tasks = list(make_request(*row, client=client) for row in rows)
        await tqdm.gather(*tasks, total=len(tasks))


def main():
    today = datetime.date.today()
    yesterday = (today - datetime.timedelta(1)).strftime("%Y-%m-%d")
    today_str = today.strftime("%Y-%m-%d")
    url = f"https://data-api.ecb.europa.eu/service/data/EXR/?startPeriod={yesterday}&endPeriod={today_str}&format=csvdata"
    file_name = f"data/exchange_rates/ecb_{yesterday}_{today_str}.csv"
    if not Path(file_name).exists():
        subprocess.Popen(("curl", "-k", "-o", file_name, url)).wait()

    asyncio.run(insert_exchange_rates(file_name))
    asyncio.run(insert_exchange_rates("data.csv"))


if __name__ == "__main__":
    main()
