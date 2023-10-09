extern crate argparse;

use argparse::{ArgumentParser, Store};
use std::path::{Path, PathBuf};

pub fn args_parse() -> (PathBuf, PathBuf) {
    let mut stdin_p_str = Path::new("data")
        .join("input")
        .into_os_string()
        .into_string()
        .unwrap();
    let mut stdout_p_str = Path::new("data")
        .join("output")
        .into_os_string()
        .into_string()
        .unwrap();

    {
        let mut ap = ArgumentParser::new();

        ap.set_description("Configuration on stdin and stdout paths.");

        // stdin_p_str modification
        ap.refer(&mut stdin_p_str).add_option(
            &["-p", "--path"],
            Store,
            "path to input with all files to process",
        );

        // stdout_p_str modification
        ap.refer(&mut stdout_p_str).add_option(
            &["-o", "--output"],
            Store,
            "Path to output all processed files",
        );

        ap.parse_args_or_exit();
    }

    let stdin_path = PathBuf::from(stdin_p_str);
    let stdout_path = PathBuf::from(stdout_p_str);

    (stdin_path, stdout_path)
}
