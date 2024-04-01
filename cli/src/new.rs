use std::{
    fs::{create_dir, File},
    io::Write,
    path::PathBuf,
};

pub fn new(project_name: &str) {
    let mut path: PathBuf = PathBuf::from(project_name);

    create_dir(&path).unwrap();

    let mut deno_conf_file = File::create(path.join("deno.jsonc")).unwrap();

    deno_conf_file
        .write_all(include_bytes!("../resources/scaffolding/deno.jsonc"))
        .unwrap();

    let mut gitignore_file = File::create(path.join(".gitignore")).unwrap();

    gitignore_file
        .write_all(include_bytes!("../resources/scaffolding/gitignore"))
        .unwrap();

    let mut dotenv_file = File::create(path.join(".env")).unwrap();

    dotenv_file
        .write_all(include_bytes!("../resources/scaffolding/env"))
        .unwrap();

    path.push("src");

    create_dir(&path).unwrap();

    let mut main_ts_file = File::create(path.join("main.ts")).unwrap();

    main_ts_file
        .write_all(include_bytes!("../resources/scaffolding/src/main.ts"))
        .unwrap();

    path.push("script");

    create_dir(&path).unwrap();

    let mut start_dia_file = File::create(path.join("start.dia")).unwrap();

    start_dia_file
        .write_all(include_bytes!(
            "../resources/scaffolding/src/script/start.dia"
        ))
        .unwrap();

    println!("Project {} created successfully!", project_name);
    println!(
        "To start the project, run `cd {}` and then `deno task dev`.",
        project_name
    );
}
