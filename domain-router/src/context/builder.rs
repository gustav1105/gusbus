use axum::extract::Request;

use super::{
    Context,
    resolver::{resolve_identity, resolve_network, resolve_request},
};

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build(req: &Request) -> Context {
        Context {
            request: resolve_request(req),
            network: resolve_network(req),
            identity: resolve_identity(req),
        }
    }
}
