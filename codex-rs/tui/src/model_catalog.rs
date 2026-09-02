use codex_protocol::openai_models::ModelPreset;
use std::convert::Infallible;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
pub(crate) struct ModelCatalog {
    models: Arc<Mutex<Vec<ModelPreset>>>,
}

impl Clone for ModelCatalog {
    fn clone(&self) -> Self {
        Self {
            models: Arc::clone(&self.models),
        }
    }
}

impl ModelCatalog {
    pub(crate) fn new(models: Vec<ModelPreset>) -> Self {
        Self {
            models: Arc::new(Mutex::new(models)),
        }
    }

    pub(crate) fn try_list_models(&self) -> Result<Vec<ModelPreset>, Infallible> {
        Ok(self.models.lock().unwrap().clone())
    }

    /// Add a model preset to the catalog (for newly configured providers).
    pub(crate) fn add_model(&self, preset: ModelPreset) {
        let mut models = self.models.lock().unwrap();
        if !models.iter().any(|m| m.model == preset.model) {
            models.push(preset);
        }
    }
}
