use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
    middleware::Next,
    extract::Request,
};

use crate::core::{consts, state};
use crate::utility::{jwt, resp};

pub async fn auth(mut request: Request, next: Next) -> Response {
    let mut user_addr = "".to_string();
    let mut is_login = false;
    let mut is_auth_path = true;

    for route in consts::NO_AUTH_ROUTERS {
        if request.uri().path().starts_with(route) {
            is_auth_path = false;
            break;
        }
    }

    if is_auth_path {
        match request.headers().get("Authorization") {
            None => is_login = false,
            Some(auth_header) => match auth_header.to_str() {
                Err(_) => is_login = false,
                Ok(auth_str) => {
                    if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                        let token = auth_str[6..auth_str.len()].trim();
                        if request.uri().path().starts_with("/api/svc/") {
                            if token == consts::SVC_AUTH_TOKEN {
                                is_login = true;
                            }
                        } else {
                            if let Ok(result) = jwt::decode_token(token.into()) {
                                is_login = true;
                                user_addr = result.id;
                            }
                        }
                    }
                }
            },
        }
    }


    if is_auth_path && !is_login {
        let rsp = resp::ErrorResponse {
            code: 403,
            msg: "Forbidden".to_string(),
            data: None,
        };
        return (StatusCode::FORBIDDEN, axum::Json(rsp)).into_response();
    }
    request.extensions_mut().insert(state::ReqContext { user_addr });
    next.run(request).await
}
