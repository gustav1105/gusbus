pub mod builder;
pub mod identity;
pub mod network;
pub mod request;
pub mod resolver;

pub use builder::ContextBuilder;
pub use identity::{IdentityContext, IdentitySource};
pub use network::NetworkContext;
pub use request::RequestContext;

#[derive(Debug, Clone)]
pub struct Context {
    pub request: RequestContext,
    pub network: NetworkContext,
    pub identity: IdentityContext,
}
