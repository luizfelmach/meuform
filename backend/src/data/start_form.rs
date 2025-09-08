use crate::{
    protocols::{AnswerRepository, CreateAnswerRepository, FormRepository},
    usecase::{StartForm, StartFormInput, StartFormOutput},
};
use anyhow::anyhow;

pub struct StartFormUseCase<F, A> {
    pub form: F,
    pub answer: A,
}

impl<F, A> StartFormUseCase<F, A>
where
    F: FormRepository,
    A: AnswerRepository,
{
    pub fn new(form: F, answer: A) -> Self {
        Self { form, answer }
    }
}

impl<F, A> StartForm for StartFormUseCase<F, A>
where
    F: FormRepository,
    A: AnswerRepository,
{
    async fn start(&self, data: StartFormInput) -> anyhow::Result<StartFormOutput> {
        let form = self
            .form
            .get_by_slug(&data.slug)
            .await?
            .ok_or_else(|| anyhow!("Form not found"))?;

        let answer = self
            .answer
            .create(CreateAnswerRepository {
                form_id: form.id.clone(),
                graph_id: form.graph_id.clone(),
            })
            .await?;

        Ok(StartFormOutput {
            answer_id: answer.id,
            form,
        })
    }
}
