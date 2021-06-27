use git_describe::git_describe;

const _: &str = git_describe!(tags, long);
const _: &str = git_describe!();

fn main() {}
