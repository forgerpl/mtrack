use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::PathBuf;

use include_dir::{include_dir, Dir, File};
use mime_guess::{mime, Mime};

static ROOT: &str = "";
static UI_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/ui");

async fn serve_asset(path: Option<Path<String>>) -> impl IntoResponse {
    let serve_file = |file: &File, mime_type: Option<Mime>, code: Option<StatusCode>| {
        Response::builder()
            .status(code.unwrap_or(StatusCode::OK))
            .header(
                header::CONTENT_TYPE,
                mime_type.unwrap_or(mime::TEXT_HTML).to_string(),
            )
            .body(Body::from(file.contents().to_owned()))
            .unwrap()
    };

    let serve_not_found = || {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File Not Found"))
            .unwrap()
    };

    let serve_default = |path: &str| {
        let default_file_path = PathBuf::from(path).join("index.html");

        if UI_DIR.get_file(default_file_path.clone()).is_some() {
            return serve_file(UI_DIR.get_file(default_file_path).unwrap(), None, None);
        }

        serve_not_found()
    };

    match path {
        Some(Path(path)) => {
            if path == ROOT {
                return serve_default(&path);
            }

            UI_DIR.get_file(&path).map_or_else(
                || match UI_DIR.get_dir(&path) {
                    Some(_) => serve_default(&path),
                    None => serve_not_found(),
                },
                |file| {
                    let mime_type =
                        mime_guess::from_path(PathBuf::from(path.clone())).first_or_octet_stream();
                    serve_file(file, Some(mime_type), None)
                },
            )
        }
        None => serve_not_found(),
    }
}

pub(crate) fn router() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { serve_asset(Some(Path(String::from(ROOT)))).await }),
        )
        .route(
            "/*path",
            get(|path| async { serve_asset(Some(path)).await }),
        )
}
