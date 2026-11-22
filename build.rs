fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Docmost");
        res.set("FileDescription", "Docmost Desktop Client");
        res.set("LegalCopyright", "Docmost Desktop");
        if let Err(e) = res.compile() {
            eprintln!("Warning: could not compile windows resource: {}", e);
        }

        // Automatisch WebView2Loader.dll in das Zielverzeichnis (target/debug bzw. target/release) kopieren
        if let Ok(out_dir) = std::env::var("OUT_DIR") {
            let out_path = std::path::PathBuf::from(out_dir);
            if let Some(target_dir) = out_path.ancestors().nth(3) {
                let src = std::path::PathBuf::from("assets").join("WebView2Loader.dll");
                let dest = target_dir.join("WebView2Loader.dll");
                if src.exists() {
                    let _ = std::fs::copy(&src, &dest);
                }
            }
        }
    }
}
