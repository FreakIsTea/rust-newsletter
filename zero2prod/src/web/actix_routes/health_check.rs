use super::*;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
