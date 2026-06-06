mod apply;
mod backfill;
mod live;
mod replay;

use config::AppConfig;
use storage::Storage;

#[derive(Clone)]
pub struct IngestService {
    config: AppConfig,
    storage: Storage,
}

impl IngestService {
    pub fn new(config: AppConfig, storage: Storage) -> Self {
        Self { config, storage }
    }
}
