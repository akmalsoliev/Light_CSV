# CSV Converter

## Overview 
This repository contains a simple and efficient tool for converting
CSV files to different formats.

## Features
- Parses and converts CSV files into parquet with Rust.

## Installation
1. Clone the repository:

`git clone https://github.com/akmalsoliev/Light_CSV`

2. Run `make setup` that will create all required directories for you. 
3. Paste your CSV file(s) into `./data/input/` (check Optional argunemnts if want to specify PATH)
4. Run `make run` (check Optional argunemnts if want to specify OUTPUT)
5. Enjoy your CSV at lightning speed!

## Optional arguments:

  -h,--help             Show this help message and exit

  -p,--path PATH        path to input with all files to process

  -o,--output OUTPUT    Path to output all processed files
