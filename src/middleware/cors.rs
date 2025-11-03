use ntex::web::middleware::DefaultHeaders;
use ntex::http::header;

pub fn cors_middleware() -> DefaultHeaders {
    DefaultHeaders::new()
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, DELETE, OPTIONS")
        .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization")
        .header(header::ACCESS_CONTROL_MAX_AGE, "3600")
}