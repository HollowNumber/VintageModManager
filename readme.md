# VintageModManager

VintageModManager is a Rust-based application designed to manage and organize mods for Vintage Story. This project aims to provide a comprehensive solution for mod management, including features such as encoding and decoding mod strings, handling mod information, and integrating with external services.

## Project Status

**Note:** This project is currently under development and is not yet finished. Many features are still being implemented, and the codebase is subject to significant changes. Contributions and feedback are welcome as we continue to improve and expand the functionality of VintageModManager.

## Features

- **Mod String Encoding/Decoding:** Encode and decode mod strings using base64 encoding.
- **Mod Information Handling:** Parse and manage mod information from JSON data.
- **Integration with External Services:** Planned integration with external services for downloading and updating mods.

## Project Structure

```
VintageModManager/
├── .gitignore
├── .idea/
│   ├── .gitignore
│   ├── misc.xml
│   ├── modules.xml
│   ├── vcs.xml
│   ├── VintageModManager.iml
│   └── workspace.xml
├── Cargo.lock
├── Cargo.toml
├── readme
├── src/
│   ├── main.rs
│   └── utils/
│       ├── api.rs
│       ├── encoding.rs
│       ├── logger.rs
│       └── mod.rs
└── target/
    ├── .rustc_info.json
    ├── CACHEDIR.TAG
    └── debug/
        ├── .cargo-lock
        ├── .fingerprint/
        ├── build/
        ├── deps/
        ├── examples/
        ├── incremental/
        ├── VintageModManager.d
        └── VintageModManager.exe
```

## Installation

To build and run the project, you need to have Rust installed. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/).

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/VintageModManager.git
    cd VintageModManager
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the project:
    ```sh
    cargo run
    ```

## Usage

### Encoding Mod Strings

You can encode mod strings using the [`encode_mod_string`](command:_github.copilot.openSymbolFromReferences?%5B%22encode_mod_string%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22c%3A%5C%5CUsers%5C%5CMikkel%5C%5CRustroverProjects%5C%5CVintageModManager%5C%5Csrc%5C%5Cutils%5C%5Cencoding.rs%22%2C%22_sep%22%3A1%2C%22external%22%3A%22file%3A%2F%2F%2Fc%253A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2Fencoding.rs%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2Fencoding.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A3%2C%22character%22%3A7%7D%7D%5D%5D "Go to definition") function in [`src/utils/encoding.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2Fencoding.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "c:\Users\Mikkel\RustroverProjects\VintageModManager\src\utils\encoding.rs"):

```rust
let encoded = encode_mod_string("your_mod_string");
println!("Encoded: {}", encoded);
```

### Decoding Mod Strings

You can decode mod strings using the [`decode_mod_string`](command:_github.copilot.openSymbolFromReferences?%5B%22decode_mod_string%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22c%3A%5C%5CUsers%5C%5CMikkel%5C%5CRustroverProjects%5C%5CVintageModManager%5C%5Csrc%5C%5Cutils%5C%5Cencoding.rs%22%2C%22_sep%22%3A1%2C%22external%22%3A%22file%3A%2F%2F%2Fc%253A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2Fencoding.rs%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2Fencoding.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A7%2C%22character%22%3A7%7D%7D%5D%5D "Go to definition") function in [`src/utils/encoding.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2Fencoding.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "c:\Users\Mikkel\RustroverProjects\VintageModManager\src\utils\encoding.rs"):

```rust
if let Some(decoded) = decode_mod_string("your_encoded_mod_string") {
    println!("Decoded: {}", decoded);
} else {
    println!("Failed to decode the mod string.");
}
```

### Handling Mod Information

You can parse and manage mod information using the [`ModInfo`](command:_github.copilot.openSymbolFromReferences?%5B%22ModInfo%22%2C%5B%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22c%3A%5C%5CUsers%5C%5CMikkel%5C%5CRustroverProjects%5C%5CVintageModManager%5C%5Csrc%5C%5Cutils%5C%5CModInfo.rs%22%2C%22_sep%22%3A1%2C%22external%22%3A%22file%3A%2F%2F%2Fc%253A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2FModInfo.rs%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2FModInfo.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%22pos%22%3A%7B%22line%22%3A42%2C%22character%22%3A11%7D%7D%5D%5D "Go to definition") struct and its associated methods in [`src/utils/ModInfo.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fc%3A%2FUsers%2FMikkel%2FRustroverProjects%2FVintageModManager%2Fsrc%2Futils%2FModInfo.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "c:\Users\Mikkel\RustroverProjects\VintageModManager\src\utils\ModInfo.rs"):

```rust
let json_str = r#"{"mod": {"modid": 1, "name": "Example Mod", "text": "Description", "tags": ["tag1", "tag2"], "author": "Author", "releases": [{"modversion": "1.0", "mainfile": "file.zip"}], "downloads": 100}}"#;
match ModInfo::from_json(json_str) {
    Ok(mod_info) => println!("Mod Info: {:?}", mod_info),
    Err(e) => println!("Error parsing mod info: {}", e),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or find any bugs.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

---

**Disclaimer:** This project is a work in progress, and many features are still being developed. The current implementation may not be fully functional, and the API is subject to change. Use at your own risk.
