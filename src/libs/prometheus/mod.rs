use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::registry::Registry;

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    pub worker_id: String,
    pub application: String,
    pub circuit: String,
    pub env: String,
    pub project: String,
}

#[derive(Clone)]
pub struct Prometheus {
    registry: Arc<Mutex<Registry>>,

    pub labels: Labels,

    pub received: Family<Labels, Counter>,
    pub processed: Family<Labels, Counter>,
    pub errors: Family<Labels, Counter>,
    pub dropped: Family<Labels, Counter>,
}

impl Labels {
    pub fn new(application: Option<String>, circuit: String, env: String, project: String) -> Self {
        Self {
            worker_id: "unknown".to_string(),
            application: application.unwrap_or_else(|| "sleipnir".to_string()),
            circuit,
            env,
            project,
        }
    }

    pub fn worker_id(&self, worker_id: String) -> Self {
        Self {
            worker_id,
            application: self.application.clone(),
            circuit: self.circuit.clone(),
            env: self.env.clone(),
            project: self.project.clone(),
        }
    }
}

impl Prometheus {
    pub fn new(application: String, circuit: String, env: String, project: String) -> Self {
        let mut registry = Registry::default();

        let labels = Labels::new(Some(application), circuit, env, project);

        let received = Family::<Labels, Counter>::default();
        let processed = Family::<Labels, Counter>::default();
        let errors = Family::<Labels, Counter>::default();
        let dropped = Family::<Labels, Counter>::default();

        registry.register("received", "Number of messages received", received.clone());

        registry.register(
            "processed",
            "Number of metrics processed",
            processed.clone(),
        );

        registry.register("errors", "Number of errors occurred", errors.clone());

        registry.register("dropped", "Number of messages dropped", dropped.clone());

        Self {
            registry: Arc::new(Mutex::new(registry)),
            labels,
            received,
            processed,
            errors,
            dropped,
        }
    }

    pub fn worker_id(&self, worker_id: usize) -> Labels {
        self.labels.worker_id(worker_id.to_string())
    }

    pub fn export(&self) -> String {
        let registry = self.registry.lock().unwrap();
        let mut buffer = String::new();
        encode(&mut buffer, &registry).unwrap();
        buffer
    }
}
