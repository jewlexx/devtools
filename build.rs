use std::{fs::File, io::Read};

fn main() {
    let current_path = std::env::current_dir().unwrap();
    let target_path = current_path.join("src/styles/blueprint.css");
    yewprint_css::download_css(&target_path).unwrap();

    // If running in debug build mode, no need to minify css
    if cfg!(debug_assertions) {
        return;
    }

    let blueprint_css = {
        let mut blueprint_css_file = File::open(&target_path).unwrap();

        let mut contents = vec![];

        blueprint_css_file.read_to_end(&mut contents).unwrap();

        String::from_utf8(contents).unwrap()
    };

    let css_minified = minifier::css::minify(&blueprint_css).unwrap();

    let target_file = File::create(&target_path).unwrap();

    css_minified.write(target_file).unwrap();
}
