use crate::{
    core::{BooleanConfig, Node, Screen, ScreenBase, ScreenKind},
    protocols::{CreateFormRepository, CreateGraphRepository, FormRepository, GraphRepository},
    usecase::{CreateForm, CreateFormInput, CreateFormOutput},
};
use anyhow::Result;
use std::{collections::HashMap, vec};

pub struct CreateFormUseCase<F, G> {
    pub form: F,
    pub graph: G,
}

impl<F, G> CreateFormUseCase<F, G>
where
    F: FormRepository,
    G: GraphRepository,
{
    pub fn new(form: F, graph: G) -> Self {
        Self { form, graph }
    }
}

impl<F, G> CreateForm for CreateFormUseCase<F, G>
where
    F: FormRepository,
    G: GraphRepository,
{
    async fn create(&self, data: CreateFormInput) -> Result<CreateFormOutput> {
        let graph = self
            .graph
            .create(CreateGraphRepository {
                nodes: HashMap::from([(
                    0,
                    Node {
                        id: 0,
                        screen: Screen {
                            base: ScreenBase {
                                title: "Pergunta".into(),
                                description: None,
                                required: false,
                            },
                            kind: ScreenKind::Boolean(BooleanConfig {
                                true_label: Some("Verdade".into()),
                                false_label: Some("Mentira".into()),
                            }),
                        },
                    },
                )]),
                edges: HashMap::new(),
                start: 0,
                end: vec![0],
            })
            .await?;

        let form = self
            .form
            .create(CreateFormRepository {
                customer_id: data.customer_id,
                graph_id: graph.id.clone(),
                slug: data.slug,
                title: data.title,
                description: data.description,
            })
            .await?;

        Ok(CreateFormOutput { form, graph })
    }
}
