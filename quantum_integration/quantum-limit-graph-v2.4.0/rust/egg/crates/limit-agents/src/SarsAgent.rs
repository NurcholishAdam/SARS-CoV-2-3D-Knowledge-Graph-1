use async_trait::async_trait;
use serde_json::json;
use limit_core::{Session, BackendRunner, RunnerOutput};
use limit_sarscov2::{MultiIntentQuestion, QueryPlan, SarsCov2Graph, VirusNode, VirologyNode};

pub struct SarsAgent;

#[async_trait]
impl BackendRunner for SarsAgent {
    fn kind(&self) -> limit_core::runners::RunnerKind { limit_core::runners::RunnerKind::Rust }

    async fn run(&self, task: serde_json::Value) -> anyhow::Result<RunnerOutput> {
        // Parse multi-intent question
        let q: MultiIntentQuestion = serde_json::from_value(task)?;
        let plan = QueryPlan {
            id: uuid::Uuid::new_v4(),
            description: "Decompose into virology + genomics intents".into(),
            steps: vec!["retrieve spike-ACE2 evidence".into(), "list variant mutations".into()],
        };

        // Build a minimal graph
        let mut graph = SarsCov2Graph::new(VirusNode { id: uuid::Uuid::new_v4(), name: "SARS-CoV-2".into(), genome_kb: 30.0 });
        graph.add_virology(VirologyNode {
            id: uuid::Uuid::new_v4(),
            topic: "Spike-ACE2 binding".into(),
            details: "Key residue interactions implicated in entry".into(),
        });

        Ok(RunnerOutput {
            ok: true,
            stdout: "SARS-CoV-2 graph initialized".into(),
            stderr: "".into(),
            metrics: json!({ "plan_steps": plan.steps.len(), "virology_nodes": graph.virology.len() }),
        })
    }
}
