use super::*;
use std::io::Write as _;

fn manifest_with(entries: &[(&str, &str)]) -> Manifest {
    Manifest {
        versions: entries
            .iter()
            .map(|(p, v)| {
                (
                    (*p).to_string(),
                    semver::Version::parse(v).expect("version"),
                )
            })
            .collect(),
    }
}

fn write_file(dir: &tempfile::TempDir, name: &str, body: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let mut f = std::fs::File::create(&path).expect("create");
    f.write_all(body.as_bytes()).expect("write");
    path
}

#[test]
fn missing_file_yields_empty_manifest() {
    let dir = tempfile::tempdir().expect("tempdir");
    let m = Manifest::load(&dir.path().join("nope.json")).expect("load");
    assert!(m.versions.is_empty());
}

#[test]
fn load_parses_versions() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = write_file(
        &dir,
        "m.json",
        "{\n  \".\": \"1.2.3\",\n  \"crates/a\": \"0.4.0-rc.1\"\n}\n",
    );
    let m = Manifest::load(&path).expect("load");
    assert_eq!(m.versions.len(), 2);
    assert_eq!(m.versions["."], semver::Version::new(1, 2, 3));
    assert_eq!(
        m.versions["crates/a"],
        semver::Version::parse("0.4.0-rc.1").expect("version")
    );
}

#[test]
fn to_json_is_sorted_pretty_with_trailing_newline() {
    let m = manifest_with(&[("pkgs/b", "0.2.3"), ("a", "1.0.0")]);
    assert_eq!(
        m.to_json(),
        "{\n  \"a\": \"1.0.0\",\n  \"pkgs/b\": \"0.2.3\"\n}\n"
    );
}

#[test]
fn empty_manifest_serializes_to_braces() {
    assert_eq!(Manifest::default().to_json(), "{}\n");
}

#[test]
fn roundtrip_through_disk_is_lossless() {
    let dir = tempfile::tempdir().expect("tempdir");
    let original = manifest_with(&[(".", "2.0.0"), ("crates/x", "0.1.0-beta.2")]);
    let path = write_file(&dir, "m.json", &original.to_json());
    assert_eq!(Manifest::load(&path).expect("load"), original);
}

#[test]
fn invalid_version_is_an_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = write_file(&dir, "m.json", "{\".\": \"one\"}");
    assert!(Manifest::load(&path).is_err());
}

#[test]
fn non_string_value_is_an_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = write_file(&dir, "m.json", "{\".\": 1}");
    assert!(Manifest::load(&path).is_err());
}

#[test]
fn invalid_json_is_an_error() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = write_file(&dir, "m.json", "{not json");
    assert!(Manifest::load(&path).is_err());
}
