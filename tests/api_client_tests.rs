use api::VintageAPIHandler;
use tokio;

#[tokio::test]
async fn test_get_mod_from_id() {
    let api_client = VintageAPIHandler::new();
    let result = api_client.get_mod_from_id(3209).await;
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.contains("modid"));
}

#[tokio::test]
async fn test_get_mod_from_name() {
    let api_client = VintageAPIHandler::new();
    let result = api_client.get_mod_from_name("CarryCapacity").await;
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.contains("modid"));
}

#[tokio::test]
async fn test_search_mods() {
    let api_client = VintageAPIHandler::new();
    let query = "orderBy=last_released";
    let result = api_client.search_mods(query.to_string()).await;
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.contains("mods"));
}

#[tokio::test]
async fn test_get_filestream() {
    let api_client = VintageAPIHandler::new();
    let file_path = "files/asset/3218/BLaMMFix.zip".to_string();
    let result = api_client.get_filestream(file_path).await;
    assert!(result.is_ok());
    let bytes = result.unwrap();
    assert!(!bytes.is_empty());
}
