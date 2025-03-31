use std::collections::HashMap;
use std::env;
use std::process;

struct Lang {
    name: String,
    code: String,
    text: String,
}

fn main() {
    let mut args = env::args();
    let lang = args.nth(1).unwrap_or("en".to_string());
    let mut langs = HashMap::new();

    for lang in [
        ("ar", "Arabic", "مرحبا بالعالم!"),
        ("de", "German", "Hallo, Welt!"),
        ("en", "English", "Hello, world!"),
        ("es", "Spanish", "¡Hola, mundo!"),
        ("fr", "French", "Bonjour, le monde!"),
        ("hi", "Hindi", "नमस्ते, दुनिया!"),
        ("it", "Italian", "Ciao, mondo!"),
        ("ja", "Japanese", "こんにちは、世界！"),
        ("ko", "Korean", "안녕하세요, 세계!"),
        ("ml", "Malayalam", "ഹലോ, വേൾഡ്!"),
        ("pt", "Portuguese", "Olá, mundo!"),
        ("ru", "Russian", "Привет, мир!"),
        ("tr", "Turkish", "Merhaba, dünya!"),
        ("zh", "Chinese", "你好，世界！"),
    ] {
        langs.insert(
            lang.0.to_string(),
            Lang {
                name: lang.1.to_string(),
                code: lang.0.to_string(),
                text: lang.2.to_string(),
            },
        );
    }

    // Show list of available languages if "--langs" is passed.
    if lang.eq("--langs") {
        println!("Available languages:");
        for (_, lang) in langs {
            println!("{}: {}", lang.code, lang.name);
        }
        return;
    }

    match langs.get(&lang) {
        Some(lang) => println!("{}", lang.text),
        None => {
            println!("Language not found. Use '--langs' to see available languages.");
            process::exit(1);
        }
    }
}
