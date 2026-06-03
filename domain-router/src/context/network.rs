use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct NetworkContext {
    pub client_ip: Option<IpAddr>,
    pub cf_ray: Option<String>,
    pub country: Option<String>,
}
