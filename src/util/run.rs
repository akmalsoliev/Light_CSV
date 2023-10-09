use std::fs;
use std::fs::File;
use std::path::PathBuf;

use glob::glob;
use polars::prelude::ParquetWriter;

use crate::util::load_frame::load_frame;
use chrono::Local;

pub fn run_conversion(stdin_path: PathBuf, stdout_path: PathBuf) {
    let string_folder_path = stdin_path
        .join("*.csv")
        .into_os_string()
        .into_string()
        .unwrap();
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

                let err_msg_crt_dir = format!("Failed to create {:?}", stdout_path);
                fs::create_dir_all(&stdout_path).expect(&err_msg_crt_dir);
                let path_output = &stdout_path.join(name_output);
                let string_path_output = &path_output.to_str().unwrap();

                let fail_message_ppf = format!("Failed to create {}", string_path_output);
                let mut parquet_file = File::create(path_output).expect(&fail_message_ppf);
                let _ = ParquetWriter::new(&mut parquet_file).finish(&mut df);
            }
            Err(_) => println!("Failed {file:?}"),
        }
    }
}
