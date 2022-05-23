use regex::Regex;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |req, _| {
            let re = Regex::new(r"/dp/\w*?/").unwrap();
            let url = &req.url().unwrap().to_string();
            console_log!("url is {}", url);
            if let Some(caps) = re.captures(url) {
                let amazon_url = format!("https://amazon.co.jp/{}", caps.get(0).unwrap().as_str());
                // Response::from_html(format!("<html><body><h1>Amazon URL shorter</h1><p>Location: {}</p></body></html>", amazon_url))
                Response::redirect(Url::parse(&amazon_url).unwrap())
            } else {
                // Response::from_html("<html><body><h1>Amazon URL Shorter</h1><form action=\"/\"><input type=value name=q placeholder\"string like /?https://amazon.co.jp/***/dp/{id}/***</\" /></form></body></html>")
                Response::ok("test")
            }
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
