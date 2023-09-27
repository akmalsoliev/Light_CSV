mod util;

use std::fs::File;
use std::path::Path;

use chrono::Local;
use polars::prelude::ParquetWriter;

use crate::util::load_frame::load_frame;

fn main() {
    let csv_path = Path::new("data")
        .join("input")
        .join("tiktok_google_play_reviews.csv");
    let mut df = load_frame(csv_path).collect().expect("Failed to collect");
    let dt = Local::now().to_string();
    let trimmed_dt = dt.replace(" ", "");
    let file_name = format!("output_file_{trimmed_dt}.parquet");
    let path_paquet_file = Path::new("data").join("output").join(file_name);
    let fail_message_ppf = format!("Failed to create {path_paquet_file:?}");
    let mut parquet_file = File::create(path_paquet_file).expect(&fail_message_ppf);
    let _ = ParquetWriter::new(&mut parquet_file).finish(&mut df);
}
