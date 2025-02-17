fn inverter_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let palavra = "Olá sou Andressa";
    let invertida = inverter_string(palavra);
    println!("String original: {}", palavra);
    println!("String invertida: {}", invertida);
}
