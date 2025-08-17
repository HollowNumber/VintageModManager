mod api;
mod config;
mod utils;

use crate::utils::{ModManager, ModManagerError};

#[tokio::main]
async fn main() -> Result<(), ModManagerError> {
    ModManager::run().await
}
