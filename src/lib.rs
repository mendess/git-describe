use proc_macro::TokenStream;
use std::process::Command;

#[proc_macro]
pub fn git_describe(input: TokenStream) -> TokenStream {
    assert!(input.is_empty());
    let o = Command::new("git")
        .args(&["describe", "--tags", "--dirty", "--long"])
        .output()
        .unwrap();
    assert!(o.status.success());
    format!(r#""{}""#, String::from_utf8(o.stdout).unwrap())
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
