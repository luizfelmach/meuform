use crate::{
    protocols::{AnswerRepository, FormRepository},
    usecase::{ListAnswers, ListAnswersInput, ListAnswersOutput},
};
use anyhow::{Ok, Result, anyhow, bail};

pub struct ListAnswersUseCase<F, A> {
    pub form: F,
    pub answer: A,
}

impl<F, A> ListAnswersUseCase<F, A>
where
    F: FormRepository,
    A: AnswerRepository,
{
    pub fn new(form: F, answer: A) -> Self {
        Self { form, answer }
    }
}

impl<F, A> ListAnswers for ListAnswersUseCase<F, A>
where
    F: FormRepository,
    A: AnswerRepository,
{
    async fn list(&self, data: ListAnswersInput) -> Result<ListAnswersOutput> {
        let form = self
            .form
            .get_by_id(&data.form_id)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        if form.customer_id != data.customer_id {
            bail!("Unauthorized")
        }

        let answers = self.answer.list_by_form(&data.form_id).await?;

        Ok(ListAnswersOutput { answers })
    }
}
