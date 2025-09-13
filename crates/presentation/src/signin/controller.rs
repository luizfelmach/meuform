use crate::{
    Controller, HttpError, HttpRequest, HttpResponse, SignInBody, SignInResponse, SignUpBody,
};

pub struct SignInController;
pub struct SignUpController;

#[async_trait::async_trait]
impl Controller for SignInController {
    async fn handle(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        let _body: SignInBody = request.json()?;

        let response = SignInResponse {
            token: "signin".into(),
        };

        return HttpResponse::ok(response);
    }
}

#[async_trait::async_trait]
impl Controller for SignUpController {
    async fn handle(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        let _body: SignUpBody = request.json()?;

        let response = SignInResponse {
            token: "signup".into(),
        };

        return HttpResponse::ok(response);
    }
}
