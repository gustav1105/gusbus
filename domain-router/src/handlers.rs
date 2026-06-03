use axum::{extract::Request, response::Html};

use crate::context::Context;

pub async fn location_page() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}

pub async fn dump(req: Request) -> Html<String> {
    let ctx = req.extensions().get::<Context>().unwrap();

    let mut body = String::new();

    body.push_str(&format!("<h1>Context</h1>"));
    body.push_str(&format!("<pre>{:#?}</pre>", ctx));

    Html(body)
}
