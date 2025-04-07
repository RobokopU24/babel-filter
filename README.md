To view usage, run `babel-filter --help`:
```
This script takes a directory of Babel files (JSONL) and creates filtered versions in a new directory containing only the lines where the `curie` value is present in the filter file (based on `id` key in filter file lines)

Usage: babel-filter [OPTIONS] <BABEL_DIRECTORY> <FILTER_FILE> <OUTPUT_DIRECTORY>

Arguments:
  <BABEL_DIRECTORY>   The directory containing Babel JSONL files
  <FILTER_FILE>       The path to the filter JSONL file to be used
  <OUTPUT_DIRECTORY>  The directory to put the filtered JSONL output files

Options:
  -e, --exclude-category <CATEGORY>    Exclude nodes with these biolink categories from the output. Multiple categories can be specified by using the flag again
  -c, --output-format <OUTPUT_FORMAT>  Force format of all output files. If not set, output files will match their input files [possible values: gzipped, plaintext]
      --read-buf-capacity <BYTES>      read buffer capacity, in bytes [default: 32000]
      --write-buf-capacity <BYTES>     write buffer capacity, in bytes [default: 32000]
  -h, --help                           Print help
  -V, --version                        Print version
```