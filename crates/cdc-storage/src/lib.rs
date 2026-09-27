pub fn greeting() -> &'static str {
    "ready"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_ready() {
        assert_eq!(greeting(), "ready");
    }
}
