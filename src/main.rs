use evg_bonsai::BonsaiProject;
use std::fs::read_to_string;

fn main() {
    let source_file = std::env::args().nth(1).expect("Missing argument");

    let contents = read_to_string(source_file).unwrap();
    let bonsai_project: BonsaiProject = serde_yaml::from_str(&contents).unwrap();
    let evergreen_project = bonsai_project.create_evg_project();

    let project_config = serde_yaml::to_string(&evergreen_project).unwrap();
    println!("{}", project_config);
}
