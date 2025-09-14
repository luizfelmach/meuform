use anyhow::Result;
use domain::{Customer, CustomerId, CustomerWithoutPassword};
use usecases::{
    AuthCustomer, AuthenticatedCustomer, CreateCustomer, DeleteCustomer, ForgotPasswordCustomer,
    GetCustomer, UpdateCustomer, UpdateCustomerInput, UpdatePasswordCustomer,
};

pub struct AuthCustomerImpl;
pub struct CreateCustomerImpl;
pub struct GetCustomerImpl;
pub struct UpdateCustomerImpl;
pub struct DeleteCustomerImpl;
pub struct ForgotPasswordCustomerImpl;
pub struct UpdatePasswordCustomerImpl;
pub struct AuthenticatedCustomerImpl;

#[async_trait::async_trait]
impl AuthCustomer for AuthCustomerImpl {
    async fn execute(&self, _email: &String, _password: &String) -> Result<String> {
        Ok("fake_token".into())
    }
}

#[async_trait::async_trait]
impl CreateCustomer for CreateCustomerImpl {
    async fn execute(
        &self,
        name: &String,
        email: &String,
        password: &String,
    ) -> Result<CustomerWithoutPassword> {
        let customer = Customer::new(
            "fake_id".into(),
            name.clone(),
            email.clone(),
            password.clone(),
        );
        Ok(customer.into())
    }
}

#[async_trait::async_trait]
impl GetCustomer for GetCustomerImpl {
    async fn execute(&self, id: &CustomerId) -> Result<CustomerWithoutPassword> {
        let customer = Customer::new(
            id.clone(),
            "fake_name".into(),
            "fake_email".into(),
            "fake_password".into(),
        );

        Ok(customer.into())
    }
}

#[async_trait::async_trait]
impl UpdateCustomer for UpdateCustomerImpl {
    async fn execute(&self, data: UpdateCustomerInput) -> Result<CustomerWithoutPassword> {
        let customer = Customer::new(
            data.id.clone(),
            "fake_name".into(),
            "fake_email".into(),
            "fake_password".into(),
        );
        Ok(customer.into())
    }
}

#[async_trait::async_trait]
impl DeleteCustomer for DeleteCustomerImpl {
    async fn execute(&self, _id: &CustomerId) -> Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl ForgotPasswordCustomer for ForgotPasswordCustomerImpl {
    async fn execute(&self, _id: &CustomerId) -> Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl UpdatePasswordCustomer for UpdatePasswordCustomerImpl {
    async fn execute(
        &self,
        _token: &String,
        _password: &String,
    ) -> Result<CustomerWithoutPassword> {
        let customer = Customer::new(
            "fake_id".into(),
            "fake_name".into(),
            "fake_email".into(),
            "fake_password".into(),
        );
        Ok(customer.into())
    }
}

#[async_trait::async_trait]
impl AuthenticatedCustomer for AuthenticatedCustomerImpl {
    async fn execute(&self, _token: &String) -> Result<CustomerId> {
        Ok("fake_id".into())
    }
}
