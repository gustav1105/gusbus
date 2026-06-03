#[derive(Debug, Clone)]
pub enum IdentitySource {
    CloudflareAccess,
    Authelia,
    Anonymous,
}

#[derive(Debug, Clone)]
pub struct IdentityContext {
    pub source: IdentitySource,
    pub email: Option<String>,
    pub username: Option<String>,
    pub groups: Vec<String>,
    pub authenticated: bool,
}
