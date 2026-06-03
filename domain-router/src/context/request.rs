#[derive(Debug, Clone)]
pub struct RequestContext {
    pub host: String,
    pub path: String,
    pub method: String,
    pub scheme: String,
}
