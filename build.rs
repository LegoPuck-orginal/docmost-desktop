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
    }
}
