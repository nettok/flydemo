mod api;

use axum::Router;
use axum::response::Html;
use axum::routing::{get, post};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(handle_index)).nest(
        "/api/v1",
        Router::new()
            .route("/inventory", get(api::get_inventory))
            .route("/inventory", post(api::post_inventory_item)),
    )
}

async fn handle_index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::Item;
    use axum::{
        body::Body,
        http,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use tower::ServiceExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn index() {
        let app = app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"<h1>Hello, World!</h1>");
    }

    #[tokio::test]
    async fn get_inventory() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/v1/inventory")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn post_inventory_item() {
        let app = app();

        let id = Some(Uuid::now_v7());

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/v1/inventory")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&json!(Item {
                            id,
                            name: "Cow".to_owned(),
                            stock: 1
                        }))
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            json!(Item {
                id,
                name: "Cow".to_owned(),
                stock: 1
            })
        );
    }
}
