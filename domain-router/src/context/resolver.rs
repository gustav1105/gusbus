use axum::extract::Request;
use std::net::IpAddr;

use super::{IdentityContext, IdentitySource, NetworkContext, RequestContext};

pub fn resolve_request(req: &Request) -> RequestContext {
    let h = req.headers();

    RequestContext {
        host: h
            .get("host")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string(),

        path: req.uri().path().to_string(),

        method: req.method().to_string(),

        scheme: h
            .get("x-forwarded-proto")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("http")
            .to_string(),
    }
}

pub fn resolve_network(req: &Request) -> NetworkContext {
    let h = req.headers();

    let client_ip = h
        .get("cf-connecting-ip")
        .or_else(|| h.get("x-forwarded-for"))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .and_then(|v| v.trim().parse::<IpAddr>().ok());

    let cf_ray = h
        .get("cf-ray")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    let country = h
        .get("cf-ipcountry")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    NetworkContext {
        client_ip,
        cf_ray,
        country,
    }
}

pub fn resolve_identity(req: &Request) -> IdentityContext {
    let h = req.headers();

    // --- AUTHELIA WINS ---
    if h.get("remote-user").is_some() {
        return IdentityContext {
            source: IdentitySource::Authelia,
            email: h
                .get("remote-email")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            username: h
                .get("remote-user")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            groups: h
                .get("remote-groups")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            authenticated: true,
        };
    }

    // --- CLOUDFLARE ACCESS ---
    if h.get("cf-access-authenticated-user-email").is_some() {
        return IdentityContext {
            source: IdentitySource::CloudflareAccess,
            email: h
                .get("cf-access-authenticated-user-email")
                .and_then(|v| v.to_str().ok())
                .map(String::from),
            username: None,
            groups: vec![],
            authenticated: true,
        };
    }

    // --- FALLBACK ---
    IdentityContext {
        source: IdentitySource::Anonymous,
        email: None,
        username: None,
        groups: vec![],
        authenticated: false,
    }
}
