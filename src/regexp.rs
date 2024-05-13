use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regexp_test() {
        let pattern = r"(?i)\bго+л\b";

        let re = Regex::new(pattern).unwrap();

        let input = "Сука Гооооооооооооооол в гооооландские ворота.";

        let output = re.replace_all(input, "поподание мячем");

        assert_eq!(output, "Сука поподание мячем в гооооландские ворота.");
    }
}
