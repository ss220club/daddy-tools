// Transliteration

#define rustutils_cyrillic_to_latin(text) CALL_LIB(RUST_UTILS, "cyrillic_to_latin")("[text]")
#define rustutils_latin_to_cyrillic(text) CALL_LIB(RUST_UTILS, "latin_to_cyrillic")("[text]")
