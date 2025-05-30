use crate::context::Context;
use crate::resolver::File;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn get_output_path(ctx: &Context, src: &Path) -> String {
    let name = src.file_stem().unwrap();

    let output_path: String;
    if name == "index" {
        output_path = format!(
            "{}/{}/{}",
            ctx.config.output.clone(),
            src.parent().unwrap().to_str().unwrap(),
            "index.html"
        );
    } else {
        output_path = format!(
            "{}/{}/{}",
            ctx.config.output.clone(),
            name.to_str().unwrap(),
            "index.html",
        );
    };

    std::path::absolute(&output_path)
        .unwrap_or_else(|_| panic!("Failed to get absolute path: {}", output_path))
        .to_str()
        .unwrap()
        .to_string()
}

fn save(ctx: &Context, path: &str, file: &mut File) {
    let src = Path::new(path).strip_prefix(ctx.config.src.clone());
    let output_path = get_output_path(ctx, &src.unwrap());

    fs::create_dir_all(Path::new(&output_path).parent().unwrap()).unwrap_or_else(|err| {
        panic!("Failed to create directory: {}: {}", path, err);
    });

    let mut content = String::new();
    let ast = file.ast.clone();
    let render: Vec<String> = ast.iter().map(|node| node.render(file)).collect();
    content.push_str(&render.join(""));

    std::fs::write(&output_path, content.clone()).unwrap_or_else(|err| {
        panic!("Failed to write file: {}: {}", path, err);
    });

    println!("Saved {} to {}", path, output_path);
}

pub fn build(ctx: &Context) {
    WalkDir::new(&ctx.config.src)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension() == Some("ds".as_ref())
        })
        .for_each(|entry| {
            let path = entry.path();
            let mut file = File::load_absolute(ctx, path.to_str().unwrap());
            save(ctx, path.to_str().unwrap(), &mut file);
        });
}
