use std::env;

fn main() {
    let args = env::args();
    let lang = args.skip(1).next().unwrap_or_else(|| "en".to_string());

    match lang.as_str() {
        "ar" => println!("مرحبا بالعالم!"),
        "de" => println!("Hallo, Welt!"),
        "en" => println!("Hello, world!"),
        "es" => println!("¡Hola, mundo!"),
        "fr" => println!("Bonjour, le monde!"),
        "hi" => println!("नमस्ते, दुनिया!"),
        "it" => println!("Ciao, mondo!"),
        "ja" => println!("こんにちは、世界！"),
        "ko" => println!("안녕하세요, 세계!"),
        "ml" => println!("ഹലോ, വേൾഡ്!"),
        "pt" => println!("Olá, mundo!"),
        "ru" => println!("Привет, мир!"),
        "tr" => println!("Merhaba, dünya!"),
        "zh" => println!("你好，世界！"),
        _ => println!("Language not supported."),
    }
}
