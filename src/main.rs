mod api;
mod utils;

use crate::utils::{ModManager, ModManagerError};

#[tokio::main]
async fn main() -> Result<(), ModManagerError> {
    ModManager::run().await
}
