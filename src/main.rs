use clap::{Parser};

mod markov_chain;
use markov_chain::order2;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Defines how much smoothing to use for the Markov Chain
    /// 
    /// Higher smoothing = more creativity
    ///
    /// Lower smoothing = more accuracy
    /// 
    /// Remember that more accuracy with lower smoothing
    /// is entirely dependent on the quality of the provided dataset.
    #[arg(short, long, default_value_t=0.0)]
    smoothing: f64,

    /// Weather to generate a name or not. Default is false.
    #[arg(short, long, default_value_t=false)]
    generate: bool,

    /// How many names to generate.
    #[arg(short, long, default_value_t=1)]
    count: usize
}

fn main() {
    let args = Args::parse();

    if args.generate {
        let names = vec!["alice", "alina", "alex", "anna", "amelia", "aria"];
        let markov = order2::Markov::train(&names, args.smoothing);
        let distributions = markov.precompute_distributions();
        let mut rng = rand::rng();

        let mut generated_names = Vec::with_capacity(args.count);

        let mut i = 0;
        let mut reruns = 0;
        while i < args.count {
            if reruns >= 10 { break };

            let name = markov.generate(&mut rng, &distributions);

            if generated_names.contains(&name) {
                reruns += 1;
                continue;
            }

            reruns = 0;

            generated_names.push(name.clone());
            println!("{}", name);

            i += 1;
        }
    }
}
