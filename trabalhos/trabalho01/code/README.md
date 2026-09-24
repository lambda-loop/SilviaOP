# SilviaOP interactive program

This version provides the simple interface required by the course assignment.
It discovers the test instances, lets the user choose an instance and a
constructive method, prints and saves the experiment report, and displays the
best route found.

## Expected directory layout

```text
SilviaOP/
├── Cargo.toml
├── data/
│   ├── instance_01.txt
│   ├── instance_02.txt
│   └── ...
└── src/
    └── ...
```

Subdirectories inside `data/` are supported. Every `.txt` file is treated as a
separate test instance.

## Running

From the project root:

```text
cargo run --release
```

If the instances are stored elsewhere, pass the data directory as the first
argument:

```text
cargo run --release -- /path/to/data
```

The terminal interface asks for:

1. the test instance;
2. one method, or all displayed methods; and
3. the number of constructions per method.

The default is one construction for a quick functional test. Enter a larger
number for randomized methods or when searching for a better route.

## Output

The complete report is printed in the terminal and saved under `reports/`. It
contains the selected instance, method, number of constructions, number of
distinct routes, best score, cost, feasibility status, and complete route.

After the report is saved, the Macroquad window displays the best route. When
multiple methods were selected, use the arrow keys to inspect their best
routes. Press `Esc` or `Q` to close the viewer.

## Standalone executable

Build the release binary with:

```text
cargo build --release
```

The resulting executable is `target/release/SilviaOP`. For course submission,
place the tested executable beside the `data/` directory and include the source
code. Test the copied package on the same operating system that will be used
for evaluation.

## Preserved experiment driver

The previous 10,000-construction route-diversity driver was preserved as
`tools/main_10000_diverse_routes.rs`. It is not compiled as part of the
interactive program; it remains available for reproducing the route-grid CSV
when needed.
