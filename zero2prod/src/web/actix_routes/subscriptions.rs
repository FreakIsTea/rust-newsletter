use super::*;

pub async fn subscribe(_form: web::Form<SubscriptionRequest>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
