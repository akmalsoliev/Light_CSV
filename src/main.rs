mod util;

use crate::util::{args_parse::args_parse, run::run_conversion};

fn main() {
    let (stdin_path, stdout_path) = args_parse();
    run_conversion(stdin_path, stdout_path)
}
