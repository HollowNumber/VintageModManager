use api::{APIData, APIDataHandler, Mod, Release, Screenshot};

#[tokio::test]
async fn parse_mod_data_valid_json() {
    let handler = APIDataHandler::new();
    let json_data = r#"
                    {
                        "statuscode": "200",
                        "mod": {
                            "modid": 1,
                            "assetid": 1,
                            "name": "Test Mod",
                            "text": "This is a test mod.",
                            "author": "Test Author",
                            "urlalias": null,
                            "logofilename": null,
                            "logofile": null,
                            "homepageurl": null,
                            "sourcecodeurl": null,
                            "trailervideourl": null,
                            "issuetrackerurl": null,
                            "wikiurl": null,
                            "downloads": 100,
                            "follows": 10,
                            "trendingpoints": 5,
                            "comments": 2,
                            "side": "Client",
                            "type": "Mod",
                            "created": "2023-01-01",
                            "lastmodified": "2023-01-02",
                            "tags": ["tag1", "tag2"],
                            "releases": [
                                {
                                    "releaseid": 1,
                                    "mainfile": "mainfile.zip",
                                    "filename": "mainfile.zip",
                                    "fileid": 1,
                                    "downloads": 50,
                                    "tags": ["tag1"],
                                    "modidstr": "mod1",
                                    "modversion": "1.0.0",
                                    "created": "2023-01-01"
                                }
                            ],
                            "screenshots": [
                                {
                                    "fileid": 1,
                                    "mainfile": "screenshot.png",
                                    "filename": "screenshot.png",
                                    "thumbnailfilename": null,
                                    "created": "2023-01-01"
                                }
                            ]
                        }
                    }"#;
    let result = handler.parse_mod_data(json_data);
    assert!(result.is_ok());
}

#[tokio::test]
async fn parse_mod_data_invalid_json() {
    let handler = APIDataHandler::new();
    let json_data = r#"
                    {
                        "statuscode": "200",
                        "mod": {
                            "modid": 1,
                            "assetid": 1,
                            "name": "Test Mod",
                            "text": "This is a test mod.",
                            "author": "Test Author",
                            "urlalias": null,
                            "logofilename": null,
                            "logofile": null,
                            "homepageurl": null,
                            "sourcecodeurl": null,
                            "trailervideourl": null,
                            "issuetrackerurl": null,
                            "wikiurl": null,
                            "downloads": 100,
                            "follows": 10,
                            "trendingpoints": 5,
                            "comments": 2,
                            "side": "Client",
                            "type": "Mod",
                            "created": "2023-01-01",
                            "lastmodified": "2023-01-02",
                            "tags": ["tag1", "tag2"],
                            "releases": [
                                {
                                    "releaseid": 1,
                                    "mainfile": "mainfile.zip",
                                    "filename": "mainfile.zip",
                                    "fileid": 1,
                                    "downloads": 50,
                                    "tags": ["tag1"],
                                    "modidstr": "mod1",
                                    "modversion": "1.0.0",
                                    "created": "2023-01-01"
                                }
                            ],
                            "screenshots": [
                                {
                                    "fileid": 1,
                                    "mainfile": "screenshot.png",
                                    "filename": "screenshot.png",
                                    "thumbnailfilename": null,
                                    "created": "2023-01-01"
                                }
                            ]
                        }
                    "#; // Missing closing brace
    let result = handler.parse_mod_data(json_data);
    assert!(result.is_err());
}

#[tokio::test]
async fn get_mainfile_path_valid() {
    let handler = APIDataHandler::new();
    let mod_data = APIData {
        statuscode: "200".to_string(),
        mod_data: Mod {
            modid: 1,
            assetid: 1,
            name: "Test Mod".to_string(),
            text: "This is a test mod.".to_string(),
            author: "Test Author".to_string(),
            urlalias: None,
            logofilename: None,
            logofile: None,
            homepageurl: None,
            sourcecodeurl: None,
            trailervideourl: None,
            issuetrackerurl: None,
            wikiurl: None,
            downloads: 100,
            follows: 10,
            trendingpoints: 5,
            comments: 2,
            side: "Client".to_string(),
            type_: "Mod".to_string(),
            created: "2023-01-01".to_string(),
            lastmodified: "2023-01-02".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            releases: vec![Release {
                releaseid: 1,
                mainfile: "mainfile.zip".to_string(),
                filename: "mainfile.zip".to_string(),
                fileid: 1,
                downloads: 50,
                tags: vec!["tag1".to_string()],
                modidstr: "mod1".to_string(),
                modversion: "1.0.0".to_string(),
                created: "2023-01-01".to_string(),
            }],
            screenshots: vec![Screenshot {
                fileid: 1,
                mainfile: "screenshot.png".to_string(),
                filename: "screenshot.png".to_string(),
                thumbnailfilename: None,
                created: "2023-01-01".to_string(),
            }],
        },
    };
    let result = handler.get_mainfile_path(&mod_data);
    assert_eq!(result, Some("mainfile.zip"));
}

#[tokio::test]
async fn get_mainfile_path_no_releases() {
    let handler = APIDataHandler::new();
    let mod_data = APIData {
        statuscode: "200".to_string(),
        mod_data: Mod {
            modid: 1,
            assetid: 1,
            name: "Test Mod".to_string(),
            text: "This is a test mod.".to_string(),
            author: "Test Author".to_string(),
            urlalias: None,
            logofilename: None,
            logofile: None,
            homepageurl: None,
            sourcecodeurl: None,
            trailervideourl: None,
            issuetrackerurl: None,
            wikiurl: None,
            downloads: 100,
            follows: 10,
            trendingpoints: 5,
            comments: 2,
            side: "Client".to_string(),
            type_: "Mod".to_string(),
            created: "2023-01-01".to_string(),
            lastmodified: "2023-01-02".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            releases: vec![],
            screenshots: vec![Screenshot {
                fileid: 1,
                mainfile: "screenshot.png".to_string(),
                filename: "screenshot.png".to_string(),
                thumbnailfilename: None,
                created: "2023-01-01".to_string(),
            }],
        },
    };
    let result = handler.get_mainfile_path(&mod_data);
    assert_eq!(result, None);
}
