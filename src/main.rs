use std::{collections::HashMap, env, io};

use axum::{
    body::{self, Body},
    extract::{Path, Query},
    http::{
        header::{HeaderMap, HeaderName, HeaderValue},
        StatusCode, Uri,
    },
    response::{Html, IntoResponse, Response, Sse},
    routing::{get, get_service, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// We've already seen returning &'static str
async fn plain_text() -> &'static str {
    "foo"
}

// String works too and will get a `text/plain; charset=utf-8` content-type
async fn plain_text_string(uri: Uri) -> String {
    format!("Hi from {}", uri.path())
}

// Bytes will get a `application/octet-stream` content-type
async fn bytes() -> Vec<u8> {
    vec![1, 2, 3, 4]
}

// `()` gives an empty response
async fn empty() {}

// `StatusCode` gives an empty response with that status code
async fn empty_with_status() -> StatusCode {
    StatusCode::NOT_FOUND
}

// A tuple of `StatusCode` and something that implements `IntoResponse` can
// be used to override the status code
async fn with_status() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
}

// A tuple of `HeaderMap` and something that implements `IntoResponse` can
// be used to override the headers
async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-foo"),
        HeaderValue::from_static("foo"),
    );
    (headers, "foo")
}

// You can also override both status and headers at the same time
async fn with_headers_and_status() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-foo"),
        HeaderValue::from_static("foo"),
    );
    (StatusCode::INTERNAL_SERVER_ERROR, headers, "foo")
}

// `Headers` makes building the header map easier and `impl Trait` is easier
// so you don't have to write the whole type
// async fn with_easy_headers() -> impl IntoResponse {
//     Headers(vec![("x-foo", "foo")])
// }

// `Html` gives a content-type of `text/html`
async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// `Result<T, E>` where `T` and `E` implement `IntoResponse` is useful for
// returning errors
async fn result() -> Result<&'static str, StatusCode> {
    Ok("all good")
}

// `Response` gives full control
async fn response() -> Response<Body> {
    Response::builder().body(Body::empty()).unwrap()
}

#[derive(Serialize)]
struct Blog {
    title: String,
    author: String,
    summary: String,
}

async fn blog_struct() -> Json<Blog> {
    let blog = Blog {
        title: "axum笔记(2)-response".to_string(),
        author: "菩提树下的杨过".to_string(),
        summary: "response各种示例".to_string(),
    };
    Json(blog)
}

async fn blog_struct_cn() -> (HeaderMap, Json<Blog>) {
    let blog = Blog {
        title: "axum笔记(2)-response".to_string(),
        author: "菩提树下的杨过".to_string(),
        summary: "response各种示例".to_string(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    (headers, Json(blog))
}

struct CustomError {
    msg: String,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let body = body::boxed(body::Full::from(self.msg));
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body)
            .unwrap()
    }
}

async fn custom_error() -> Result<&'static str, CustomError> {
    Err(CustomError {
        msg: "Opps!".to_string(),
    })
}

async fn user_info(Path(id): Path<i32>) -> String {
    format!("user id:{}", id)
}

async fn person(Path((id, age)): Path<(i32, i32)>) -> String {
    format!("id:{},age:{}", id, age)
}
#[derive(Deserialize)]
struct SomeRequest {
    a: Option<String>,
    b: Option<i32>,
    c: Option<String>,
    d: Option<u32>,
}

// eg: path_req/a1/b1/c1/d1
async fn path_req(Path(req): Path<SomeRequest>) -> String {
    format!("a:{:?},b:{:?},c:{:?},d:{:?}", req.a, req.b, req.c, req.d)
}

//eg: query_req/?a=test&b=2&c=abc&d=80
async fn query_req(Query(req): Query<SomeRequest>) -> String {
    format!("a:{:?},b:{:?},c:{:?},d:{:?}", req.a, req.b, req.c, req.d)
}
//eg: query?a=1&b=1.0&c=xxx
async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    for (key, value) in &params {
        println!("key:{},value:{}", key, value);
    }
    format!("{:?}", params)
}

// 表单提交
async fn form_request(Form(model): Form<SomeRequest>) -> String {
    format!(
        "a:{},b:{},c:{},d:{}",
        model.a.unwrap_or_default(),
        model.b.unwrap_or(-1), //b缺省值指定为-1
        model.c.unwrap_or_default(),
        model.d.unwrap_or_default()
    )
}

// json提交
async fn json_request(Json(req): Json<SomeRequest>) -> String {
    format!("a:{:?},b:{:?},c:{:?},d:{:?}", req.a, req.b, req.c, req.d)
}

/**
 * 获取所有请求头
 */
async fn get_all_header(headers: HeaderMap) -> String {
    for (key, value) in &headers {
        println!("key:{:?} , value:{:?}", key, value);
    }
    format!("{:?}", headers)
}

use tower_http::{
    cors::{self, CorsLayer},
    services::{ServeDir, ServeFile},
};

pub const ASSET_DIR_ENV: &str = "ASSET_DIR";
#[tokio::main]
async fn main() {
    let asset_dir = env::var(ASSET_DIR_ENV).unwrap_or_else(|_| "static".to_string());

    let serve_dir = ServeDir::new(&asset_dir)
        .not_found_service(ServeFile::new(format!("{}/index.html", asset_dir)));
    let serve_dir = get_service(serve_dir);

    // TODO: enable in development only!!!

    // our router
    let app = Router::new()
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(
            CorsLayer::new()
                .allow_headers(cors::Any)
                .allow_origin(cors::Any),
        )
        .route("/plain_text", get(plain_text))
        .route("/plain_text_string", get(plain_text_string))
        .route("/bytes", get(bytes))
        .route("/empty", get(empty))
        .route("/empty_with_status", get(empty_with_status))
        .route("/with_status", get(with_status))
        .route("/with_headers", get(with_headers))
        .route("/with_headers_and_status", get(with_headers_and_status))
        // .route("/with_easy_headers", get(with_easy_headers))
        .route("/html", get(html))
        .route("/json", get(json))
        .route("/result", get(result))
        .route("/response", get(response))
        .route("/blog", get(blog_struct))
        .route("/blog_cn", get(blog_struct_cn))
        .route("/custom_error", get(custom_error))
        .route("/user/:id", get(user_info))
        .route("/person/:id/:age", get(person))
        .route("/path_req/:a/:b/:c/:d", get(path_req))
        .route("/query_req", get(query_req))
        .route("/query", get(query))
        .route("/form", post(form_request))
        .route("/json", post(json_request))
        .route("/header", get(get_all_header));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
