#[derive(Debug)]
pub struct Upload<'a> {
    pub description: &'a str,
    pub message: &'a str,
    pub file_path: &'a str,
}
