use std::fs::read_to_string;
use evg_bonsai::landscape::landscape::BonsaiLandscape;
use chrono::prelude::*;

fn main() {
    let source_file = std::env::args().nth(1).expect("Missing argument");

    let contents = read_to_string(source_file).unwrap();
    let bonsai_project: BonsaiLandscape = serde_yaml::from_str(&contents).unwrap();
    let evergreen_project = bonsai_project.create_evg_project();

    let project_config = serde_yaml::to_string(&evergreen_project).unwrap();
    println!("# Generated from bonsai");
    let now = Utc::now();
    println!("# Generated at: {}", now);
    println!("{}", project_config);
}
