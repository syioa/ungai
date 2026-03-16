mod markov_chain;
use markov_chain::order2;

fn main() {
    let names = vec!["alice", "alina", "alex", "anna", "amelia", "aria"];
    let markov = order2::Markov::train(&names);
    let mut rng = rand::rng();

    for _ in 0..10 {
        let name = markov.generate(&mut rng);

        // if name.len() < 2 {
        //     continue;
        // }

        println!("Name generated: {}\nLength: {}\n", name, name.len());
    }
}
