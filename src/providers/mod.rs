pub mod arch;
pub mod authors;
pub mod autoconf;
pub mod composer_json;
pub mod debian;
pub mod doap;
pub mod git;
pub mod gobo;
pub mod go;
pub mod haskell;
pub mod launchpad;
pub mod maven;
pub mod meson;
pub mod metainfo;
#[cfg(feature = "opam")]
pub mod ocaml;
pub mod package_json;
pub mod package_xml;
pub mod package_yaml;
pub mod perl;
pub mod pubspec;
pub mod python;
pub mod r;
pub mod ruby;
#[cfg(feature = "cargo")]
pub mod rust;
pub mod security_md;
pub mod waf;
