mod markov_chain;
use markov_chain::Markov;

fn main() {
    let names = vec![
        "alice",
        "alina",
        "alex",
        "anna",
        "amelia",
        "aria",
    ];

    let markov = Markov::train(&names);

    let mut rng = rand::rng();

    for _ in 0..10 {
        let name = markov.generate(&mut rng);
        println!("Name generated: {} | Length: {}", name, name.len());
    }
}
