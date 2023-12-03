use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};

use colored::*;

/// A struct for the result of a solution part with its result and elapsed
/// running time of the solving method.
pub struct AocResult<ResponseType: Display> {
    output: ResponseType,
    elapsed: Duration,
}

/// A trait for the solution of an Advent of Code day.
///
/// Implementors of the `AocSolution` trait should implement the two parts of
/// the day in the methods `solve_first` and `solve_second`. Both parts must
/// have the same type. In the case they should differ, choose a common
/// denominator between those, usually it will be [`String`].
pub trait AocSolution<const DAY: u32> {
    type ResponseType: Display;

    fn solve_first(&self, input: &str) -> Self::ResponseType;
    fn solve_second(&self, input: &str) -> Self::ResponseType;

    /// Read the input file that is located in `input/<DAY>.txt`, where `DAY`
    /// is the current day number without padded zeros.
    ///
    /// This function assumes that the input file already exists and does not
    /// create any when it doesn't.
    fn read_input_file(&self) -> std::io::Result<String> {
        let mut input_file =
            std::env::current_dir().expect("Failed to access current working directory.");
        input_file.push("input");
        input_file.push(format!("{}.txt", DAY));

        fs::read_to_string(input_file)
    }

    /// Write the output file that will be located in `output/<DAY>.txt`, where
    /// `DAY` is the current day number without padded zeros.
    ///
    /// This function will overwrite any preexisting output file.
    fn write_output_file(
        &self,
        solutions: &[AocResult<Self::ResponseType>],
    ) -> std::io::Result<()> {
        let mut output_file =
            std::env::current_dir().expect("Failed to access current working directory.");
        output_file.push("output");
        output_file.push(format!("{}.txt", DAY));

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
    fn timed_execute(
        &self,
        solve_func: fn(&Self, &str) -> Self::ResponseType,
        input: &str,
    ) -> AocResult<Self::ResponseType> {
        let start = Instant::now();
        let output = solve_func(self, input);
        let elapsed = start.elapsed();

        AocResult { output, elapsed }
    }

    /// Setups and executes the implemented solutions.
    ///
    /// This method will read the input file for the given day, run both
    /// solving methods and write the output to an output file as well as
    /// the console.
    fn execute(&self) -> Option<()> {
        print!("Executing solution for day {}... ", DAY);

        let solutions = match self.read_input_file() {
            Ok(input) => vec![
                self.timed_execute(Self::solve_first, &input),
                self.timed_execute(Self::solve_second, &input),
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

        println!("{}", "✅ Passed.".green());

        for (n, solution) in solutions.iter().enumerate() {
            println!(
                "Part {} ran for {} s.",
                n + 1,
                solution.elapsed.as_secs_f64()
            );
            println!("{}", "====== Output ======".bright_black());
            println!("{}", solution.output);
            println!("{}", "====================\n".bright_black());
        }

        if let Err(why) = self.write_output_file(&solutions) {
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

pub fn print_header() -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    writeln!(
        lock,
        "                 {}\n",
        "Adventure of Code 2023".bright_red()
    )?;

    writeln!(
        lock,
        "{}",
        "          .     .  .      +     .      .          .".green()
    )?;
    writeln!(
        lock,
        "{}",
        "     .       .      .     #       .           .".green()
    )?;
    writeln!(
        lock,
        "{}",
        "        .      .         ###            .      .      .".green()
    )?;
    writeln!(
        lock,
        "{}",
        r###"      .      .   "#:. .:##"##:. .:#"  .      ."###.green()
    )?;
    writeln!(
        lock,
        "{}",
        r#####"          .      . "####"###"####"  ."#####.green()
    )?;
    writeln!(
        lock,
        "{}",
        r####"       .     "#:.    .:#"###"#:.    .:#"  .        .       ."####.green()
    )?;
    writeln!(
        lock,
        "{}",
        r##########"  .             "#########"#########"        .        ."##########.green()
    )?;
    writeln!(
        lock,
        "{}",
        r#####"        .    "#:.  "####"###"####"  .:#"   .       ."#####.green()
    )?;
    writeln!(
        lock,
        "{}",
        r########"     .     .  "#######""##"##""#######"                  ."########.green()
    )?;
    writeln!(
        lock,
        "{}",
        r######"                ."##"#####"#####"##"           .      ."######.green()
    )?;
    writeln!(
        lock,
        "{}",
        r####"    .   "#:. ...  .:##"###"###"##:.  ... .:#"     ."####.green()
    )?;
    writeln!(
        lock,
        "{}",
        r########"      .     "#######"##"#####"##"#######"      .     ."########.green()
    )?;
    writeln!(
        lock,
        "{}",
        r#########"    .    .     "#####""#######""#####"    .      ."#########.green()
    )?;
    writeln!(
        lock,
        "{}",
        r#"            .     "      000      "    .     ."#.green()
    )?;
    writeln!(
        lock,
        "{}",
        "       .         .   .   000     .        .       .".green()
    )?;
    writeln!(
        lock,
        "{}\n",
        ".. .. ..................O000O........................ ...... ...".green()
    )?;

    writeln!(lock, "Running solution set...")?;

    Ok(())
}
