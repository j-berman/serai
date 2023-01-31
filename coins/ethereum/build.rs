use ethers_solc::{Project, ProjectPathsConfig};
use ethers::contract::Abigen;

fn main() {
  println!("cargo:rerun-if-changed=contracts");
  println!("cargo:rerun-if-changed=artifacts");

  // configure the project with all its paths, solc, cache etc.
  let project = Project::builder()
    .paths(ProjectPathsConfig::hardhat(env!("CARGO_MANIFEST_DIR")).unwrap())
    .build()
    .unwrap();
  project.compile().unwrap();

  // Tell Cargo that if a source file changes, to rerun this build script.
  project.rerun_if_sources_changed();

  Abigen::new("Router", format!("./artifacts/Router.sol/Router.json"))
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file(format!("./src/router.rs"))
    .unwrap();
}
