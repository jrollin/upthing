use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_health_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = health_check(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
