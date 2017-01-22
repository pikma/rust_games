extern crate clap;

extern crate game_search;

use std::fs;
use std::io;
use std::process;

use game_search::sudoku;
use game_search::solver;


fn main() {
    let args = clap::App::new("Game Solver")
        .version("0.1")
        .author("Pierre Kreitmann <pierre.kreitmann@gmail.com>")
        .about("Solves basic games. Pretty useless.")
        .subcommand(clap::SubCommand::with_name("sudoku")
                    .about("Solves sudokus")
                    .arg(clap::Arg::with_name("board_file")
                         .long("board_file")
                         .short("f")
                         .takes_value(true)
                         .required(true)
                         .help("The path to the file that describes the sudoku puzzle. \
                                Example contents:\n\
                                    003020600\n\
                                    900305001\n\
                                    001806400\n\
                                    008102900\n\
                                    700000008\n\
                                    006708200\n\
                                    002609500\n\
                                    800203009\n\
                                    005010300")))
        .get_matches();

    match args.subcommand() {
        ("sudoku", Some(sub_m)) => {
            let file_name = sub_m.value_of("board_file").unwrap();
            let mut input_file = io::BufReader::new(fs::File::open(file_name).unwrap());
            let mut state = sudoku::State::read(&mut input_file).unwrap();
            println!("{}", state);
            if !solver::solve(&mut state) {
                println!("No solution");
                process::exit(1);
            }

            println!("{}", state);
        },
        _ => unimplemented!(),
    }
}
