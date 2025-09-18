use axum::{
    Form, Json, RequestExt,
    extract::{
        FromRequest, OptionalFromRequest, Request,
        rejection::{FormRejection, JsonRejection},
    },
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct JsonOrForm<T>(pub T);

pub enum JsonOrFormRejection {
    NoMediaType,
    UnsupportedMediaType,
    Json(JsonRejection),
    Form(FormRejection),
}

impl IntoResponse for JsonOrFormRejection {
    fn into_response(self) -> Response {
        match self {
            JsonOrFormRejection::NoMediaType => StatusCode::BAD_REQUEST.into_response(),
            JsonOrFormRejection::UnsupportedMediaType => {
                StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response()
            }
            JsonOrFormRejection::Json(rej) => rej.into_response(),
            JsonOrFormRejection::Form(rej) => rej.into_response(),
        }
    }
}

impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<(), Rejection = JsonRejection>,
    Form<T>: FromRequest<(), Rejection = FormRejection>,
    T: 'static,
{
    type Rejection = JsonOrFormRejection;

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req
                    .extract::<Json<T>, _>()
                    .await
                    .map_err(|err| JsonOrFormRejection::Json(err))?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req
                    .extract()
                    .await
                    .map_err(|e| JsonOrFormRejection::Form(e))?;
                return Ok(Self(payload));
            }

            return Err(JsonOrFormRejection::UnsupportedMediaType);
        }

        Err(JsonOrFormRejection::NoMediaType)
    }
}

impl<S, T> OptionalFromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    T: 'static,
    Json<T>: FromRequest<(), Rejection = JsonRejection>,
    Form<T>: FromRequest<(), Rejection = FormRejection>,
{
    type Rejection = JsonOrFormRejection;

    async fn from_request(req: Request, state: &S) -> Result<Option<Self>, Self::Rejection> {
        match <Self as FromRequest<S>>::from_request(req, state).await {
            Ok(value) => Ok(Some(value)),
            Err(err) => match err {
                JsonOrFormRejection::NoMediaType => Ok(None),
                _ => Err(err),
            },
        }
    }
}
