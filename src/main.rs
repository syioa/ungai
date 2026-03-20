mod markov_chain;
mod utils;

use clap::Parser;
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
    #[arg(short, long, default_value_t = 0.0, verbatim_doc_comment)]
    smoothing: f64,

    /// Weather to generate a name or not
    ///
    /// [default: false]
    #[arg(short, long, default_value_t = false, verbatim_doc_comment)]
    generate: bool,

    /// How many names to generate
    #[arg(short, long, default_value_t = 1, verbatim_doc_comment)]
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

    /// Provide a file from which to train the model
    ///
    /// The provided file must be a `.txt` or `.csv`
    /// which should just contain the names on which to
    /// train the model and should only contain newlines,
    /// and commas as separators.
    /// One thing to note here is that more priority
    /// is given to commas, i.e. only commas are necessary
    /// for separating different names.
    #[arg(short = 'f', long, verbatim_doc_comment)]
    train_from_file: Option<String>,

    /// Provide temperature scaling for further creativity of the model
    ///
    /// This is similar to `smoothing`
    /// Temperature Scaling works in the following way-
    /// temperature > 1.0: More Creative/Random Names
    /// temperature < 1.0: More Predictable/Repetitive Names
    /// temperature = 1.0: No change
    #[arg(short = 't', long, default_value_t = 1.0, verbatim_doc_comment)]
    temperature: f64,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    if let Err(e) = utils::download_precomputed_dataset() {
        eprintln!("error downloading precomputed dataset: {e}");
        return Err(e.to_string());
    };

    let markov: order2::Markov;
    match args.read_transitions {
        Some(file_name) => match order2::Markov::read_transitions_from(&file_name) {
            Ok(data) => markov = data,
            Err(e) => {
                eprintln!("can't read from file due to the following error:");
                return Err(e.to_string());
            }
        },
        None => {
            let names = match args.train_from_file {
                Some(file_name) => utils::parse_file(&file_name).unwrap_or_else(|err| {
                    eprintln!("can't read from the file due to the following error: {err}");
                    eprintln!("reverting back to the default (very small) names list");
                    vec![
                        "alice".to_string(),
                        "alina".to_string(),
                        "alex".to_string(),
                        "anna".to_string(),
                        "amelia".to_string(),
                        "aria".to_string(),
                    ]
                }),
                None => {
                    vec!["nothing".to_string()]
                }
            };

            if names.len() == 1 && names[0] == "nothing" {
                markov = match order2::Markov::read_transitions_from(
                    utils::get_default_dataset_path()
                        .to_str()
                        .expect("invalid path"),
                ) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("can't read from file due to the following error:");
                        return Err(e.to_string());
                    }
                }
            } else {
                markov = order2::Markov::train(&names);
            }
        }
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
        let distributions = markov.precompute_distributions(args.smoothing, args.temperature);
        let mut rng = rand::rng();

        let mut generated_names = Vec::with_capacity(args.count);

        let mut i = 0;
        let mut reruns = 0;
        while i < args.count {
            if reruns >= 10 {
                break;
            };

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
