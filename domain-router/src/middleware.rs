use axum::{body::Body, http::Request, middleware::Next, response::Response};

use crate::context::{Context, ContextBuilder};

pub async fn context_middleware(mut req: Request<Body>, next: Next) -> Response {
    // 1. Build context from request
    let ctx: Context = ContextBuilder::build(&req);

    // 2. Inject into request extensions
    req.extensions_mut().insert(ctx);

    // 3. Continue pipeline
    next.run(req).await
}
