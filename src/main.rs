fn main() {
    let tab: [&str; 6] = ["42", "3.5", "-7", "true", "0", "quarante"];

    for str in tab {
        let entier = match str.parse::<i32>() {
            Ok(_) => "ok",
            Err(_) => "echec",
        };

        let float = match str.parse::<f64>() {
            Ok(_) => "ok",
            Err(_) => "echec",
        };

        let bool = match str.parse::<bool>() {
            Ok(_) => "ok",
            Err(_) => "echec",
        };

        println!("{:?} -> i32: {}, f64: {}, bool: {}",str, entier, float, bool);
    }
}
