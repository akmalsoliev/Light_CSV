use polars::prelude::*;
use std::path::PathBuf;

pub fn load_frame(path: PathBuf) -> LazyFrame {
    let df = LazyCsvReader::new(path)
        .with_try_parse_dates(true)
        .with_infer_schema_length(Some(50))
        .low_memory(false)
        .has_header(true)
        .finish()
        .expect("Failed to construct the DataFrame");

    df
}
