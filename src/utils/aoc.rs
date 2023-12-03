use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};

use colored::*;

const TREE: &str = r##########"
          *     *  *      +     *      *          *
     *       *      *     #       *           *
        *      *         ###            *      *      *
      *      *   "#:. .:##"##:* .:#"  *      *
          *      * "####"###"####"  *
       *     "#:.    .:#"###"#:.    .:#"  *        *       *
  *             "#########"#########"        *        *
        *    "#:.  "####"###"####"  .:#"   *       *
     *     *  "#######""##"##""#######"                  *
                ."##"#####"#####"##"           *      *
    *   "#:. ...  .:##"###"###"##:.  ... .:#"     *
      *     "#######"##"#####"##"#######"      *     *
    *    *     "#####""#######""#####"    *      *
            *     "      000      "    *     *
       *         *   *   000     *        *       *
__ __ __________________O000O________________________ ______
"##########;

pub fn print_header() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    writeln!(lock, "\t\t{}", "Adventure of Code 2023".bold().bright_red())?;

    for symbol in TREE.chars() {
        write!(
            lock,
            "{}",
            match symbol {
                '"' => symbol.to_string().bright_red(),
                '+' | '*' => symbol.to_string().yellow(),
                '0' | 'O' | '_' => symbol.to_string().bright_black(),
                _ => symbol.to_string().bold().bright_green(),
            }
        )?;
    }

    writeln!(lock)
}

/// A struct for the result of a solution part with its result and elapsed
/// running time of the solving method.
struct AocResult<ResponseType: Display> {
    output: ResponseType,
    elapsed: Duration,
}

/// Read the input file that is located in `input/<DAY>.txt`, where `DAY`
/// is the current day number without padded zeros.
///
/// This function assumes that the input file already exists and does not
/// create any when it doesn't.
fn read_input_file(day: u32) -> std::io::Result<String> {
    let mut input_file =
        std::env::current_dir().expect("Failed to access current working directory.");
    input_file.push("input");
    input_file.push(format!("{}.txt", day));

    fs::read_to_string(input_file)
}

/// Write the output file that will be located in `output/<DAY>.txt`, where
/// `DAY` is the current day number without padded zeros.
///
/// This function will overwrite any preexisting output file.
fn write_output_file<T: Display>(day: u32, solutions: &[AocResult<T>]) -> std::io::Result<()> {
    let mut output_file =
        std::env::current_dir().expect("Failed to access current working directory.");
    output_file.push("output");
    output_file.push(format!("{}.txt", day));

    let output = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)?;

    let mut writer = BufWriter::new(&output);

    for (n, solution) in solutions.iter().enumerate() {
        writeln!(
            &mut writer,
            "Part {} (time: {} s)",
            n + 1,
            solution.elapsed.as_secs_f64()
        )?;
        writeln!(&mut writer, "====================")?;
        writeln!(&mut writer, "{}", solution.output)?;
        writeln!(&mut writer, "====================\n")?;
    }

    Ok(())
}

/// Runs the given method and returns the output and elapsed time in as a
/// [`AocResult`].
fn timed_execute<T: Display>(solve_func: fn(&str) -> T, input: &str) -> AocResult<T> {
    let start = Instant::now();
    let output = solve_func(input);
    let elapsed = start.elapsed();

    AocResult { output, elapsed }
}

/// A trait for the solution of an Advent of Code day.
///
/// Implementors of the `AocSolution` trait should implement the two parts of
/// the day in the methods `solve_first` and `solve_second`. Both parts must
/// have the same type. In the case they should differ, choose a common
/// denominator between those, usually it will be [`String`].
pub trait AocSolution<const DAY: u32> {
    type ResponseType: Display;

    fn solve_first(input: &str) -> Self::ResponseType;
    fn solve_second(input: &str) -> Self::ResponseType;

    /// Setup and execute the implemented solutions.
    ///
    /// This method will retrieve the input file for the solutions, run both of
    /// them while timing their runtime and output the results to console and a
    /// text file.
    fn execute(&self) -> Option<()> {
        println!(
            "{} {:^10} {}",
            "~".repeat(24).white(),
            format!("{} {:>2}", "Day", DAY).bold(),
            "~".repeat(24).white()
        );

        let solutions = match read_input_file(DAY) {
            Ok(input) => vec![
                timed_execute(Self::solve_first, &input),
                timed_execute(Self::solve_second, &input),
            ],
            Err(why) => {
                println!(
                    "{} {} {}",
                    "❌ Failed.".red(),
                    "Could not read input file:".bright_black(),
                    why.to_string().bright_black()
                );

                vec![]
            }
        };

        for (n, solution) in solutions.iter().enumerate() {
            println!("{:>4}{:<15} ", "", format!("{} {}", "Part", n + 1));
            println!(
                "\t{} {} {} {}",
                "output".white(),
                "(".white(),
                solution.output,
                ")".white()
            );
            println!(
                "\t{} {} {} {}",
                "time".white(),
                "(".white(),
                format!("{} {}", solution.elapsed.as_micros(), "us").bold(),
                ")".white()
            );
        }

        if let Err(why) = write_output_file(DAY, &solutions) {
            println!(
                "{} {} {}",
                "❗ Warning.".yellow(),
                "Could not write output file:".bright_black(),
                why
            );
        }

        Some(())
    }
}
