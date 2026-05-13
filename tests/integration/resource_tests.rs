use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_unused_resources_integration() {
    let temp_dir = TempDir::new().unwrap();
    let project_root = temp_dir.path();

    let my_project = project_root.join("myproject");
    fs::create_dir_all(&my_project).unwrap();

    let res_dir = my_project.join("res").join("values");
    fs::create_dir_all(&res_dir).unwrap();

    // Create unused resource
    let strings_xml = res_dir.join("strings.xml");
    fs::write(
        &strings_xml,
        r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="unused_at_integration">Unused</string>
</resources>"#,
    )
    .unwrap();

    fs::write(my_project.join("Dummy.kt"), "package com.example").unwrap();

    // Run analyzer on the sub-directory
    let mut cmd = Command::cargo_bin("searchdeadcode").unwrap();
    cmd.arg(&my_project)
        .arg("--unused-resources")
        .arg("--min-confidence")
        .arg("low")
        .arg("--parallel")
        .arg("false");

    // Use assert_cmd's failure() to check code 1 correctly
    let output = cmd.assert().code(1);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(stdout.contains("unused_at_integration"));
}

#[test]
fn test_resource_baseline_integration() {
    let temp_dir = TempDir::new().unwrap();
    let project_root = temp_dir.path();

    let my_project = project_root.join("myproject");
    fs::create_dir_all(&my_project).unwrap();

    let baseline_path = my_project.join("baseline.json");

    let res_dir = my_project.join("res").join("values");
    fs::create_dir_all(&res_dir).unwrap();

    // Create unused resource
    let strings_xml = res_dir.join("strings.xml");
    fs::write(
        &strings_xml,
        r#"<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="baseline_resource">Baseline</string>
</resources>"#,
    )
    .unwrap();

    fs::write(my_project.join("Dummy.kt"), "package com.example").unwrap();

    // 1. Generate baseline
    let mut cmd = Command::cargo_bin("searchdeadcode").unwrap();
    cmd.arg(&my_project)
        .arg("--unused-resources")
        .arg("--min-confidence")
        .arg("low")
        .arg("--generate-baseline")
        .arg(&baseline_path);

    cmd.assert().code(1); // Fails because issues found

    assert!(baseline_path.exists());
    let baseline_content = fs::read_to_string(&baseline_path).unwrap();
    assert!(baseline_content.contains("baseline_resource"));

    // 2. Run with baseline (should pass)
    let mut cmd = Command::cargo_bin("searchdeadcode").unwrap();
    cmd.arg(&my_project)
        .arg("--unused-resources")
        .arg("--min-confidence")
        .arg("low")
        .arg("--baseline")
        .arg(&baseline_path);

    cmd.assert().success();
}
