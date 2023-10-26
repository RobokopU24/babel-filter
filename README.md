**Todo:**

- [ ] explore if regex parser is faster than serde_json
  - `r#"curie":\s*"([^\s,]*)"`
- [ ] add functionality for jsonl indentifier keys to be set by command line args
- [ ] add verbosity selector to clap, log/warning?/error/silent
- [ ] figure out best way to check if babel files are jsonl (just extension?)
- [ ] improve error handling/matching
  - reduce nesting
  - don't silently skip nodes
- [ ] error file when a node fails or write to stderr? config via flag?
- [ ] add better progress indication, colors, est time remaining
- [ ] allow read/write buffer to be configured by flag
- [ ] multithreading?
  - per file--probably not worth the headache, especially if this is running on a HDD
  - per directory--potentially doable and could save time if the OS supports concurrent r/w
- [x] support compressing/decompressing gzip
  - detect `.gz` and comp/decomp accordingly so that normal files are allowed as well
- [ ] print info table -> how many nodes were removed
