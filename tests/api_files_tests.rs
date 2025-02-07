use api::files::FileManager;
use bytes::Bytes;
use tokio;

#[tokio::test]
async fn test_save_file() {
    let file_manager = FileManager::new();
    let file_name = "test_save_file.txt";
    let content = Bytes::from("Hello, world!");

    let result = file_manager.save_file(file_name, content.clone()).await;
    assert!(result.is_ok());

    let read_result = file_manager.read_file(file_name).await;
    assert!(read_result.is_ok());
    assert_eq!(read_result.unwrap(), content);

    let delete_result = file_manager.delete_file(file_name).await;
    assert!(delete_result.is_ok());
}

#[tokio::test]
async fn test_read_file() {
    let file_manager = FileManager::new();
    let file_name = "test_read_file.txt";
    let content = Bytes::from("Hello, world!");

    let save_result = file_manager.save_file(file_name, content.clone()).await;
    assert!(save_result.is_ok());

    let result = file_manager.read_file(file_name).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), content);

    let delete_result = file_manager.delete_file(file_name).await;
    assert!(delete_result.is_ok());
}

#[tokio::test]
async fn test_delete_file() {
    let file_manager = FileManager::new();
    let file_name = "test_delete_file.txt";
    let content = Bytes::from("Hello, world!");

    let save_result = file_manager.save_file(file_name, content).await;
    assert!(save_result.is_ok());

    let delete_result = file_manager.delete_file(file_name).await;
    assert!(delete_result.is_ok());

    let read_result = file_manager.read_file(file_name).await;
    assert!(read_result.is_err());
}

#[tokio::test]
async fn test_file_exists() {
    let file_manager = FileManager::new();
    let file_name = "test_file_exists.txt";
    let content = Bytes::from("Hello, world!");

    let save_result = file_manager.save_file(file_name, content).await;
    assert!(save_result.is_ok());

    let exists_result = file_manager.file_exists(file_name).await;
    assert!(exists_result.is_ok());
    assert!(exists_result.unwrap());

    let delete_result = file_manager.delete_file(file_name).await;
    assert!(delete_result.is_ok());

    let exists_result = file_manager.file_exists(file_name).await;
    assert!(exists_result.is_ok());
    assert!(!exists_result.unwrap());
}
