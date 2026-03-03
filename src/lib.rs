use serde::Deserialize;
use std::str::FromStr;
use worker::*;

#[derive(Deserialize)]
struct APIResult<T> {
    result: T,
}

#[derive(Deserialize)]
struct Package {
    resources: Vec<Resource>,
}

#[derive(Deserialize)]
struct Resource {
    identifier: String,
    url: String,
}

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/packages/:package/resources/:resource", handle_package)
        .run(_req, _env)
        .await
}

async fn handle_package(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let package_id = _ctx.param("package").ok_or("Missing package id")?;
    let resource = _ctx.param("resource").ok_or("Missing resource name")?;
    let token = _ctx.secret("OPENTRANSPORTDATA_API_KEY")?;


    let headers = Headers::new();
    headers.append("Authorization", &format!("Bearer {}", token))?;
    let mut init = RequestInit::new();
    init.with_headers(headers);

    let request = Request::new_with_init(
        &format!("https://api.opentransportdata.swiss/ckan-api/package_show?id={}", package_id),
        &init,
    )?;

    let mut response = Fetch::Request(request).send().await?;

    if response.status_code() != 200 {
        let text = response.text().await?;
        return Response::error(
            format!("Request failed: {} {}", text, response.status_code()),
            500,
        );
    }

    let api_result: APIResult<Package> = response.json().await?;

    let item = api_result
        .result
        .resources
        .iter()
        .find(|r| r.identifier == resource.to_string())
        .ok_or("Could not find resource")?;

    Response::redirect(Url::from_str(&item.url)?)
}
