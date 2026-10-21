//! See the crate documentation. Each entry mirrors a consumer's `build.rs`:
//! `Visibility::Public`, derives `Debug, Clone, PartialEq`, and the codec the
//! protocol uses on the wire (APER for NGAP/S1AP/NRPPa, UPER for RRC/LPP).
//!
//! `rustfmt` is switched OFF here, deliberately: consumers format the output,
//! but the formatted bytes depend on the installed rustfmt version, which
//! would make the hashes drift with the toolchain rather than with this
//! compiler. The unformatted generator output is what this repo controls.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use asn1_compiler::generator::{Codec, Derive, Visibility};
use asn1_compiler::Asn1Compiler;
use sha2::{Digest, Sha256};

/// Wire codec, as an enum of our own because `Codec` is neither `Copy` nor `Clone`.
#[derive(Clone, Copy)]
enum Wire {
    Aper,
    Uper,
}

impl Wire {
    fn codec(self) -> Codec {
        match self {
            Wire::Aper => Codec::Aper,
            Wire::Uper => Codec::Uper,
        }
    }
}

/// (output label, schema file, codec, consumers)
const TARGETS: &[(&str, &str, Wire, &str)] = &[
    (
        "ngap",
        "ngap-17.9.asn",
        Wire::Aper,
        "nextgcore-ngap-gen, nextgsim-ngap",
    ),
    (
        "s1ap",
        "s1ap-19.2.0.asn",
        Wire::Aper,
        "nextgcore-s1ap-gen, nextgsim-s1ap",
    ),
    (
        "nrppa-nextgsim",
        "nrppa-19.2.0.nextgsim.asn1",
        Wire::Aper,
        "nextgsim-ngap",
    ),
    (
        "nrppa-nextgcore",
        "nrppa-19.2.0.nextgcore.asn",
        Wire::Aper,
        "nextgcore-nrppa-gen",
    ),
    ("lpp", "lpp-19.3.0.asn", Wire::Uper, "nextgcore-lpp-gen"),
    ("nr-rrc", "rrc-19.3.0.asn1", Wire::Uper, "nextgsim-rrc"),
    (
        "lte-rrc",
        "lte-rrc-19.4.0.asn",
        Wire::Uper,
        "nextgsim-lte-rrc",
    ),
];

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn generate(schema: &Path, wire: Wire, out: &Path) -> Vec<u8> {
    let mut compiler = Asn1Compiler::new(
        out.to_str().expect("utf-8 path"),
        &Visibility::Public,
        vec![wire.codec()],
        vec![Derive::Debug, Derive::Clone, Derive::PartialEq],
    );
    compiler.set_rustfmt_generated_code(false);
    compiler
        .compile_files(&[schema])
        .unwrap_or_else(|e| panic!("compiling {}: {e}", schema.display()));
    std::fs::read(out).expect("generated output")
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn read_golden(path: &Path) -> BTreeMap<String, String> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (hash, name) = l.split_once("  ").expect("`<sha256>  <name>` line");
            (name.trim().to_owned(), hash.to_owned())
        })
        .collect()
}

#[test]
fn generated_code_matches_the_golden_hashes() {
    let dir = crate_dir();
    let golden_path = dir.join("golden").join("SHA256SUMS");
    let scratch = std::env::temp_dir().join(format!("codegen-golden-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).expect("scratch dir");

    let mut actual = BTreeMap::new();
    for (label, schema, codec, _) in TARGETS {
        let out = scratch.join(format!("{label}.rs"));
        let bytes = generate(&dir.join("schemas").join(schema), *codec, &out);
        assert!(!bytes.is_empty(), "{label}: empty output");
        actual.insert(format!("{label}.rs"), sha256_hex(&bytes));
    }
    let _ = std::fs::remove_dir_all(&scratch);

    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        let mut text = String::from(
            "# SHA-256 of the UNFORMATTED generator output for each pinned schema.\n\
             # Regenerate with: UPDATE_GOLDEN=1 cargo test -p codegen-golden\n",
        );
        for (name, hash) in &actual {
            text.push_str(&format!("{hash}  {name}\n"));
        }
        std::fs::create_dir_all(golden_path.parent().unwrap()).unwrap();
        std::fs::write(&golden_path, text).expect("write golden");
        return;
    }

    let golden = read_golden(&golden_path);
    let mut drift = Vec::new();
    for (name, hash) in &actual {
        match golden.get(name) {
            Some(g) if g == hash => {}
            Some(g) => drift.push(format!("{name}: golden {g}, generated {hash}")),
            None => drift.push(format!("{name}: missing from golden/SHA256SUMS")),
        }
    }
    for name in golden.keys() {
        if !actual.contains_key(name) {
            drift.push(format!(
                "{name}: in golden/SHA256SUMS but no longer generated"
            ));
        }
    }
    assert!(
        drift.is_empty(),
        "generated code changed -- review the diff, then UPDATE_GOLDEN=1:\n{}",
        drift.join("\n")
    );
}

#[test]
fn generation_is_deterministic() {
    // A hash pin is only meaningful if two runs give identical bytes; the
    // resolver iterates collections, so prove it on a large schema.
    let dir = crate_dir();
    let scratch = std::env::temp_dir().join(format!("codegen-det-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    let schema = dir.join("schemas").join("ngap-17.9.asn");
    let a = generate(&schema, Wire::Aper, &scratch.join("a.rs"));
    let b = generate(&schema, Wire::Aper, &scratch.join("b.rs"));
    let _ = std::fs::remove_dir_all(&scratch);
    assert_eq!(sha256_hex(&a), sha256_hex(&b), "two runs differ");
}
