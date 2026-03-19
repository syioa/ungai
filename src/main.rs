use clap::{Parser};

mod markov_chain;
use markov_chain::order2;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Defines how much smoothing to use for the Markov Chain
    /// 
    /// Higher Smoothing = More Creativity
    /// Lower Smoothing = More Accuracy
    /// 
    /// Remember that more accuracy with lower smoothing
    /// is entirely dependent on the quality of the provided dataset.
    #[arg(short, long, default_value_t=0.0, verbatim_doc_comment)]
    smoothing: f64,

    /// Weather to generate a name or not
    ///
    /// [default: false]
    #[arg(short, long, default_value_t=false, verbatim_doc_comment)]
    generate: bool,

    /// How many names to generate
    #[arg(short, long, default_value_t=1, verbatim_doc_comment)]
    count: usize,

    /// Whether to write transitions to a file for better performance
    /// in the next run
    ///
    /// This flag requires you to specify the name of the file to
    /// write to. Note that the file extension should be `.zst`
    /// but you are free to choose whatever you want.
    #[arg(short, long, verbatim_doc_comment)]
    write_transitions: Option<String>,

    /// Whether to read transitions from a file for better performance
    ///
    /// This flag requires you to specify the name of the file
    /// which contains the data.
    #[arg(short, long, verbatim_doc_comment)]
    read_transitions: Option<String>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let markov: order2::Markov;
    match args.read_transitions {
        Some(file_name) => {
            match order2::Markov::read_transitions_from(&file_name) {
                Ok(data) => markov = data,
                Err(e) => {
                    eprintln!("can't read from file due to the following error:");
                    return Err(e.to_string());
                }
            }
        },
        None => {
            let names = vec!["alice", "alina", "alex", "anna", "amelia", "aria"];
            markov = order2::Markov::train(&names, args.smoothing);
        },
    }

    // write transitions to a file
    if let Some(file_name) = args.write_transitions {
        if let Err(e) = markov.write_transitions_to_file(&file_name) {
            eprintln!("can't write to file because of the following error:");
            return Err(e.to_string());
        }
    }

    // generate a name/names
    if args.generate {
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

    Ok(())
}
