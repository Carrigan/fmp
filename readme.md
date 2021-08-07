# Frontmatter Processor

A utility for extracting and processing YAML frontmatter from markdown files.

```
Frontmatter Processor 0.1

USAGE:
    fmp [FLAGS] [OPTIONS] [path]

ARGS:
    <path>    [default: .]

FLAGS:
    -h, --help       Prints help information
    -v               Print debug information during run
    -V, --version    Prints version information

OPTIONS:
    -f <filter>...   Filters only frontmatters with keys containing <filter>. Can be supplied
                     multiple times and will be ORed together.
```

## Todo

- [X] Sort rows by date
- [X] Sort columns alphabetically with "date" coming first
- [X] Add command line interface
- [X] Add ability to filter
- [ ] Clean up code
