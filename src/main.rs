mod days;
mod utils;

fn main() {
    utils::aoc::print_header().expect("The header couldn't be displayed.");

    days::execute_all();
}
