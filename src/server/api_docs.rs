use crate::{
    companies::ApiDoc as ApiDocCompanies,
    countries::ApiDoc as ApiDocCountries,
    currencies::ApiDoc as ApiDocCurrencies,
    dictionary::ApiDoc as ApiDocDictionary,
    exchanges::ApiDoc as ApiDocExchanges,
    industries::ApiDoc as ApiDocIndustries,
    sectors::ApiDoc as ApiDocSectors,
    server::ErrorMessage,
    transactions::{ApiDocTransactions},
    users::ApiDoc as ApiDocUsers,
};

use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    security(("jwt" = ["*"])),
    servers(
        (url = "http://{domain}:{port}/api/{version}", description = "Local server",
            variables(
                ("domain" = (default = "127.0.0.1", description = "Default domain for API")),
                ("port" = (default = "8000", enum_values("8000", "5000", "3030"), description = "Supported ports for API")),
                ("version" = (default = "v1", enum_values("v1"), description = "Supported versions for API")),
            )
        )),
    nest(
        (path = "/", api = ApiDocUsers, tags = ["Users"]),
        (path = "/", api = ApiDocCurrencies, tags = ["Currencies"]),
        (path = "/", api = ApiDocDictionary, tags = ["Dictionary"]),
        (path = "/", api = ApiDocCompanies, tags = ["Companies"]),
        (path = "/", api = ApiDocExchanges, tags = ["Exchanges"]),
        (path = "/", api = ApiDocIndustries, tags = ["Industries"]),
        (path = "/", api = ApiDocSectors, tags = ["Sectors"]),
        (path = "/", api = ApiDocCountries, tags = ["Countries"]),
        (path = "/", api = ApiDocTransactions, tags = ["Transactions"]),
    ),
    components(
        schemas(ErrorMessage),
        responses(ErrorMessage)
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
