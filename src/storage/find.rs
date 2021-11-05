pub fn find(value: &str) -> Result<&[u8], std::io::Error> {
    Ok(value.as_bytes())
}