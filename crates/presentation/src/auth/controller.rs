use crate::{Controller, HttpError, HttpRequest, HttpResponse};
use crate::{SignInBody, SignInResponse, SignUpBody};
use usecases::{DynAuthenticateCustomer, DynCustomerRetrieveProfile};

pub struct SignInController {
    pub auth: DynAuthenticateCustomer,
}
pub struct SignUpController {
    pub auth: DynAuthenticateCustomer,
    pub create: DynCustomerRetrieveProfile,
}

#[async_trait::async_trait]
impl Controller for SignInController {
    async fn handle(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        let SignInBody { email, password }: SignInBody = request.json()?;

        match self.auth.execute(&email, &password).await {
            Ok(token) => HttpResponse::ok(SignInResponse { token }),
            Err(_) => Err(HttpError::Unauthorized),
        }
    }
}

#[async_trait::async_trait]
impl Controller for SignUpController {
    async fn handle(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        let SignUpBody {
            name,
            email,
            password,
        } = request.json()?;

        if let Err(err) = self.create.execute(&name, &email, &password).await {
            return Err(HttpError::BadRequest(err.to_string()));
        }

        match self.auth.execute(&email, &password).await {
            Ok(token) => HttpResponse::ok(SignInResponse { token }),
            Err(_) => Err(HttpError::Unauthorized),
        }
    }
}
