#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct File {
    pub path: Option<String>,
    pub encoding: Option<String>,
    pub text: Option<String>,
}
