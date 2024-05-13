use translit::{CharsMapping, Transliterator};

byond_fn!(fn cyrillic_to_latin (string) {
    Some(cyr_to_latin(string))
});

byond_fn!(fn latin_to_cyrillic (string) {
    Some(latin_to_cyr(string))
});

fn cyr_to_latin(string: &str) -> String {
    Transliterator::new(table_to_latin()).convert(string, false)
}

fn latin_to_cyr(string: &str) -> String {
    Transliterator::new(table_from_latin()).convert(string, true)
}

fn table_to_latin() -> CharsMapping {
    [
        ("А", "A"),
        ("Б", "B"),
        ("В", "V"),
        ("Г", "G"),
        ("Д", "D"),
        ("Е", "E"),
        ("Ё", "Yo"),
        ("Ж", "Zh"),
        ("З", "Z"),
        ("И", "I"),
        ("Й", "I"),
        ("К", "K"),
        ("Л", "L"),
        ("М", "M"),
        ("Н", "N"),
        ("О", "O"),
        ("П", "P"),
        ("Р", "R"),
        ("С", "S"),
        ("Т", "T"),
        ("У", "U"),
        ("Ф", "F"),
        ("Х", "Kh"),
        ("Х", "H"),
        ("Ц", "Ts"),
        ("Ч", "Ch"),
        ("Ш", "Sh"),
        ("Щ", "Shch"),
        ("Ъ", "Ie"),
        ("Ы", "Y"),
        ("Ь", "'"),
        ("Э", "E"),
        ("Ю", "Iu"),
        ("Я", "Ia"),
        ("а", "a"),
        ("б", "b"),
        ("в", "v"),
        ("г", "g"),
        ("д", "d"),
        ("е", "e"),
        ("ё", "yo"),
        ("ж", "zh"),
        ("з", "z"),
        ("и", "i"),
        ("й", "i"),
        ("к", "k"),
        ("л", "l"),
        ("м", "m"),
        ("н", "n"),
        ("о", "o"),
        ("п", "p"),
        ("р", "r"),
        ("с", "s"),
        ("т", "t"),
        ("у", "u"),
        ("ф", "f"),
        ("х", "kh"),
        ("ц", "ts"),
        ("ч", "ch"),
        ("ш", "sh"),
        ("щ", "shch"),
        ("ъ", "ie"),
        ("ы", "y"),
        ("ь", "'"),
        ("э", "e"),
        ("ю", "iu"),
        ("я", "ia"),
        ("№", "#"),
    ]
    .to_vec()
}

fn table_from_latin() -> CharsMapping {
    [
        ("А", "A"),
        ("Б", "B"),
        ("В", "V"),
        ("В", "W"),
        ("Г", "G"),
        ("Д", "D"),
        ("Дж", "J"),
        ("Э", "E"),
        ("Ё", "Yo"),
        ("Ж", "Zh"),
        ("З", "Z"),
        ("З", "Th"),
        ("Зэ", "The"),
        ("И", "I"),
        ("Й", "I"),
        ("К", "C"),
        ("К", "K"),
        ("К", "Q"),
        ("К", "Ck"),
        ("Кс", "X"),
        ("Л", "L"),
        ("М", "M"),
        ("Н", "N"),
        ("О", "O"),
        ("Оу", "Ow"),
        ("П", "P"),
        ("Р", "R"),
        ("С", "S"),
        ("Т", "T"),
        ("У", "U"),
        ("Ф", "F"),
        ("Х", "Kh"),
        ("Х", "H"),
        ("Ц", "Ts"),
        ("Ч", "Ch"),
        ("Ш", "Sh"),
        ("Щ", "Shch"),
        ("Ъ", "Ie"),
        ("Ы", "Y"),
        ("Ь", "'"),
        ("Е", "E"),
        ("Ю", "Iu"),
        ("Я", "Ia"),
        ("а", "a"),
        ("б", "b"),
        ("в", "v"),
        ("в", "w"),
        ("г", "g"),
        ("д", "d"),
        ("дж", "j"),
        ("е", "e"),
        ("ё", "yo"),
        ("ж", "zh"),
        ("з", "z"),
        ("з", "th"),
        ("зэ", "the"),
        ("и", "i"),
        ("й", "i"),
        ("к", "c"),
        ("к", "k"),
        ("к", "q"),
        ("к", "ck"),
        ("кс", "x"),
        ("л", "l"),
        ("м", "m"),
        ("н", "n"),
        ("о", "o"),
        ("оу", "ow"),
        ("п", "p"),
        ("р", "r"),
        ("с", "s"),
        ("т", "t"),
        ("у", "u"),
        ("ф", "f"),
        ("х", "kh"),
        ("х", "h"),
        ("ц", "ts"),
        ("ч", "ch"),
        ("ш", "sh"),
        ("щ", "shch"),
        ("ъ", "ie"),
        ("ы", "y"),
        ("ь", "'"),
        ("э", "e"),
        ("ю", "iu"),
        ("я", "ia"),
        ("е", "ѣ"),
        ("Е", "Ѣ"),
        ("И", "І"),
        ("и", "і"),
        ("а", "ä"),
        ("А", "Ä"),
        ("Йо", "ö"),
        ("йо", "Ö"),
        ("Оэ", "Ø"),
        ("оэ", "ø"),
        ("А", "Æ"),
        ("а", "æ"),
        ("О", "Å"),
        ("о", "å"),
        ("Аэ", "Ä"),
        ("аэ", "ä"),
        ("Оо", "Ꝏ"),
        ("оо", "ꝏ"),
        ("Ау", "Ꜽ"),
        ("ау", "ꜽ"),
        ("Ое", "Œ"),
        ("ое", "œ"),
        ("№", "#"),
    ]
    .to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_test() {
        let result = cyr_to_latin("Съешь же ещё этих мягких французских булок, да выпей чаю!");
        assert_eq!(
            result,
            "Sieesh' zhe eshchyo etikh miagkikh frantsuzskikh bulok, da vypei chaiu!"
        );

        let result2 = cyr_to_latin("Привет мир! Hello world!");
        assert_eq!(result2, "Privet mir! Hello world!");

        let result3 = latin_to_cyr("Privet mir! Hello world! Zhypyr perotin kuroden.");
        assert_eq!(result3, "Привет мир! Хелло ворлд! Жыпыр перотин куроден.");

        let result4 = latin_to_cyr("Привет мир! Hello world!");
        assert_eq!(result4, "Привет мир! Хелло ворлд!");

        let result5 = latin_to_cyr("The quick brown fox jumps over the lazy dog!");
        assert_eq!(result5, "Зэ куик броун фокс джумпс овер зэ лазы дог!");
    }
}
