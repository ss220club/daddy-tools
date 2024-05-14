use regex_cache::RegexCache;
use std::sync::Mutex;

byond_fn!(fn regex_replace (text, re, re_params, replacement) {
    Some(regexp_replace(text, re, re_params, replacement))
});

const CACHE_SIZE: usize = 128;

fn init_regex_cache() -> RegexCache {
    RegexCache::new(CACHE_SIZE)
}

static RE_CACHE: Mutex<Option<RegexCache>> = Mutex::new(None);

fn compile_regex(pattern: &str) -> regex::Regex {
    let mut cache = RE_CACHE.lock().unwrap();
    if let Some(ref mut cache) = *cache {
        cache.compile(pattern).unwrap().clone()
    } else {
        *cache = Some(init_regex_cache());
        init_regex_cache().compile(pattern).unwrap().clone()
    }
}

fn regexp_replace(text: &str, re: &str, re_params: &str, replacement: &str) -> String {
    let pattern = format!(r"(?{}){}", re_params, re);
    let re = compile_regex(pattern.as_str());

    re.replace_all(text, replacement).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regexp_test() {
        let pattern = r"\bго+л\b";

        let pattern_flags = "i";

        let input = "Сука Гооооооооооооооол в гооооландские ворота.";

        let expected_output = "Сука попадание мячем в гооооландские ворота.";

        let replacement = "попадание мячем";

        for _ in 1..2 {
            assert_eq!(
                expected_output,
                regexp_replace(input, pattern, pattern_flags, replacement)
            );
        }
    }
}
