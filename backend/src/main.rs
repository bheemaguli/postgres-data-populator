use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
mod utils;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `GET /` goes to `root`
        .route("/generate-excel", get(generate_excel));
    // .route("/excel", get(get_excel_view));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Welcome to the Excel Generator! Visit /generate-excel to download an Excel file."
}

async fn generate_excel() -> impl IntoResponse {
    match utils::excel::generate_excel_file() {
        Ok(buffer) => {
            // Build the response with headers and body
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                )
                .header(
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"faker-generated-files.xlsx\"",
                );

            // Return the file as a response
            response.body(buffer.into()).unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Failed to create response".into())
                    .unwrap()
            })
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create Excel file: {}", e),
        )
            .into_response(),
    }
}
