use proc_macro::{TokenStream, TokenTree};
use std::{iter::once, process::Command};

#[proc_macro]
pub fn git_describe(input: TokenStream) -> TokenStream {
    let mut v = once(String::from("describe"))
        .chain(input.into_iter().filter_map(|t| {
            if let TokenTree::Ident(i) = t {
                Some(format!("--{}", i))
            } else {
                None
            }
        }))
        .collect::<Vec<_>>();
    if v.len() == 1 {
        v.extend(
            ["--tags", "--dirty", "--long"]
                .iter()
                .map(ToString::to_string),
        );
    }
    let o = Command::new("git").args(v).output().unwrap();
    assert!(
        o.status.success(),
        "git describe failed with output: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    format!(r#""{}""#, String::from_utf8(o.stdout).unwrap())
        .parse()
        .unwrap()
}
