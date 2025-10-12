use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = Path::new("src/argocd");

    fs::create_dir_all(src_dir).expect("Failed to create argocd directory");

    let crds = vec![
        (
            "https://raw.githubusercontent.com/argoproj/argo-cd/refs/tags/v3.1.7/manifests/crds/application-crd.yaml",
            "application.rs",
        ),
        (
            "https://raw.githubusercontent.com/argoproj/argo-cd/refs/tags/v3.1.7/manifests/crds/applicationset-crd.yaml",
            "application_set.rs",
        ),
        (
            "https://raw.githubusercontent.com/argoproj/argo-cd/refs/tags/v3.1.7/manifests/crds/appproject-crd.yaml",
            "application_project.rs",
        ),
    ];

    for (url, filename) in crds {
        println!("cargo:rerun-if-changed=build.rs");

        let response = reqwest::blocking::get(url)
            .unwrap_or_else(|_| panic!("Failed to download CRD from {}", url));

        let yaml_content = response.text().expect("Failed to read response text");

        let temp_yaml_path =
            Path::new(&out_dir).join(format!("{}.yaml", filename.strip_suffix(".rs").unwrap()));
        fs::write(&temp_yaml_path, yaml_content).expect("Failed to write temporary YAML file");

        let output = Command::new("kopium")
            .arg("-Af")
            .arg(&temp_yaml_path)
            .output()
            .expect("Failed to execute kopium command. Make sure kopium is installed.");

        if !output.status.success() {
            panic!("kopium failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let rust_code =
            String::from_utf8(output.stdout).expect("Failed to parse kopium output as UTF-8");

        let target_path = src_dir.join(filename);
        fs::write(&target_path, rust_code).expect(&format!("Failed to write {}", filename));

        fs::remove_file(&temp_yaml_path).ok();
    }
}
