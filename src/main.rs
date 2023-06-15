use float_eq::float_eq;
use std::io::Write;
use cgp::global_params::CgpParameters;
use cgp::utils::runner::Runner;
use cgp::datasets::*;
use clap::Parser;
use std::fs;
use std::fs::File;
use std::path::Path;


#[derive(Parser)]
#[clap(author, version, about, name = "testname")]
struct Args {
    #[arg(long, default_value_t = 0)]
    run_id: usize,

    #[arg(long, default_value_t = 0)]
    dataset: usize,

    #[arg(long, default_value_t = 100)]
    nbr_nodes: usize,

    #[arg(long, default_value_t = 2)]
    cgp_type: usize,

    // 0: single
    // 1: prob
    #[arg(long, default_value_t = 0)]
    mutation_type: usize,

    #[arg(long, default_value_t = -1.)]
    mutation_prob: f32,

}

fn main() {
    let args = Args::parse();

    if args.mutation_type == 1 {
        if float_eq!(args.mutation_prob, -1., abs <= 0.01) {
            panic!("Mutation prob not listed");
        }
    }

    let cgp_type = match args.cgp_type {
        0 => "Vanilla",
        1 => "DAG",
        2 => "Reorder",
        3 => "E-Reorder",
        _ => panic!(),
    };

    let (data, label) = match args.dataset {
        0 => parity::get_dataset(),
        1 => encode::get_dataset(),
        2 => decode::get_dataset(),
        3 => multiply::get_dataset(),
        _ => panic!("Wrong dataset"),
    };

    let mut params = CgpParameters::default();

    params.graph_width = args.nbr_nodes;

    let nbr_inputs = data.shape()[1];
    let nbr_outputs = label.shape()[1];

    params.nbr_inputs = nbr_inputs;
    params.nbr_outputs = nbr_outputs;

    // let stdout = std::io::stdout();
    // let mut lock = stdout.lock();

    let mut runner = Runner::new(params.clone(), data, label, args.mutation_type, args.mutation_prob);
    let mut i = 0;
    // let mut prev_it: Vec<f32> = Vec::from(runner.get_best_fitness());
    loop {
        i += 1;
        // if i % params.eval_after_iterations == 0 {
        //     writeln!(lock, "Iteration: {i}, Fitness: {:?}", runner.get_best_fitness());
        // }
        runner.learn_step();
        // if float_eq!(runner.get_best_fitness()[0], 0., abs <= 0.000_1) {  // for multiple parents
        if float_eq!(runner.get_best_fitness(), 0., abs <= 0.000_1) {  // for single parent
            break;
        }

        if i > 1_000_000 {
            break;
        }

        // for (pr, aft) in prev_it.iter().zip(runner.get_best_fitness().iter()) {
        //     if aft > pr {
        //         println!("Panic at it {i}");
        //         println!("{:?}, {:?}", prev_it, runner.get_best_fitness());
        //         panic!();
        //     }
        // }
    }

    println!("{i}");

    let temp_str;
    if args.mutation_type == 0 {
        temp_str = format!("single");
    } else {
        temp_str = format!("prob_{}", args.mutation_prob);
    }

    let dataset_string = match args.dataset {
        0 => "parity",
        1 => "encode",
        2 => "decode",
        3 => "multiply",
        _ => panic!("Wrong dataset"),
    };

    let save_path = Path::new(".")
        .join("Experiments_Output")
        .join(cgp_type)
        .join(dataset_string)
        .join(format!("number_nodes_{}_{}", args.nbr_nodes, temp_str));
    fs::create_dir_all(save_path.clone()).expect("cannot create dir");

    let save_file_iteration = format!("run_{}_iteration.txt", args.run_id);
    let mut output = File::create(save_path.join(save_file_iteration))
        .expect("cannot create file");
    write!(output, "End at iteration: {}", i).expect("cannot write");

    let save_file_active_node = format!("run_{}_active_node.txt", args.run_id);
    let mut output = File::create(save_path.join(save_file_active_node))
        .expect("cannot create file");
    let mut parent = runner.get_parent();
    parent.get_active_nodes_id();

    write!(output, "{:?}", parent.active_nodes.unwrap()).expect("cannot write");
}

// use cgp::reorder::linspace::linspace;
//
// fn has_unique_elements<T>(iter: T) -> bool
//     where
//         T: IntoIterator,
//         T::Item: Eq + Hash,
// {
//     let mut uniq = HashSet::new();
//     iter.into_iter().all(move |x| uniq.insert(x))
// }
//
// fn main() {
//     for i in 1..1000 {
//         let a = linspace(0, 1000, i);
//         println!("{}: {:?}\n", i, a);
//         assert!(has_unique_elements(a));
//         // let b =
//     }
//
//     // println!("{:?}", a);
// }
