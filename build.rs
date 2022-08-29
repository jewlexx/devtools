fn main() {
    let current_path = std::env::current_dir().unwrap();
    yewprint_css::download_css(current_path.join("public/blueprint.css")).unwrap();
}