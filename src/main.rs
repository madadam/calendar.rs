extern crate chrono;
extern crate docopt;
extern crate rustc_serialize;

mod advanced_iterator;
mod date;
mod format;

use advanced_iterator::AdvancedIterator;
use date::dates;
use format::layout_month;
use docopt::Docopt;

const USAGE: &'static str = "
Calendar.

Usage:
  calendar <year> [--months-per-line=<num>]
  calendar (-h | --help)

Options:
  -h --help                 Show this screen
  --months-per-line=<num>   Number of months per line [default: 3]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_year: i32,
    flag_months_per_line: usize
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());

    let calendar = dates(args.arg_year)
                  .by_month()
                  .map(layout_month)
                  .chunk(args.flag_months_per_line)
                  .map(|c| c.transpose())
                  .chain_all()
                  .map(|c| c.collect::<String>())
                  .join("\n");

    println!("{}", calendar);
}
