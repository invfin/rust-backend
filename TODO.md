# TODO
- [X] Create accounts
- [ ] Create transactions for account
- [ ] Save custom dashboards (db table, model, endpoint), load dashboard
- [ ] Create endpoints for the dashboards to get data


Checkout the [diesel webpage](https://diesel.rs) for
longer guides about diesel
Checkout the [crates.io source code](https://github.com/rust-lang/crates.io/)
for a real world application using axum and diesel

## Get historical exchanges rates from:
### USA
https://api.stlouisfed.org/fred/category/series?category_id=94&api_key=&file_type=json
get each series_id from the previous url
https://api.stlouisfed.org/fred/series/observations?series_id=DEXMXUS&api_key=&file_type=json

### EUR
https://data.ecb.europa.eu/help/data-examples
Free but only for now date:
https://happyapi.fr/api/devises
Same as above but only 1000 req/month:
https://openexchangerates.org/account?nope=already_signed_up