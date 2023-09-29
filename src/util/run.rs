use std::fs;
use std::{fs::File, path::Path};

use glob::glob;
use polars::prelude::ParquetWriter;

use crate::util::load_frame::load_frame;
use chrono::Local;

pub fn run_conversion() {
    let folder_path = Path::new("data").join("input").join("*.csv");
    let string_folder_path = folder_path
        .into_os_string()
        .into_string()
        .expect("Failed to convert path to string");

    let folder = glob(&string_folder_path).expect("Failed to parse the {string_folder_path}");

    for file in folder.into_iter() {
        match file {
            Ok(file_path) => {
                let mut df = load_frame(file_path.clone())
                    .collect()
                    .expect("Failed to read {file_path} into LazyFrame");

                // This will be used as file name
                let name_file = file_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(".csv", "");

                let dt = Local::now().format("%d_%M_%YY").to_string();
                let name_output = format!("{}_{}.parquet", name_file, dt);

                let dir_path_output = Path::new("data").join("output");
                let err_msg_crt_dir = format!("Failed to create {:?}", dir_path_output);
                fs::create_dir_all(&dir_path_output).expect(&err_msg_crt_dir);
                let path_output = &dir_path_output.join(name_output);
                let string_path_output = &path_output.to_str().unwrap();

                let fail_message_ppf = format!("Failed to create {}", string_path_output);
                let mut parquet_file = File::create(path_output).expect(&fail_message_ppf);
                let _ = ParquetWriter::new(&mut parquet_file).finish(&mut df);
            }
            Err(_) => println!("Failed {file:?}"),
        }
    }
}
