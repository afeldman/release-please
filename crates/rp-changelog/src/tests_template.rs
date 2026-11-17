use crate::render_entry;
use crate::tests::{commits, input, sections};
use semver::Version;

#[test]
fn template_gets_documented_context() {
    let v = Version::parse("1.2.0").unwrap();
    let (c, s) = (commits(), sections());
    let mut inp = input(&v, &c, &s);
    let tpl = "## {{ tag }} {{ version }} {{ previous_tag }}\n{% for b in breaking %}BREAK {{ b.description }}\n{% endfor %}{% for sec in sections %}{{ sec.title }}:{{ sec.commits | length }}\n{% endfor %}";
    inp.template = Some(tpl);
    let out = render_entry(&inp).unwrap();
    assert_eq!(
        out,
        "## v1.2.0 1.2.0 v1.1.0\nBREAK drop old API\nFeatures:1\nBug Fixes:1\n"
    );
}

#[test]
fn invalid_template_is_an_error() {
    let v = Version::parse("1.2.0").unwrap();
    let (c, s) = (commits(), sections());
    let mut inp = input(&v, &c, &s);
    inp.template = Some("{% for x in %}");
    assert!(render_entry(&inp).is_err());
}
