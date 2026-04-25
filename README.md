# Rust Port of asciigraph

[![Crates.io](https://img.shields.io/crates/v/asciigraph.svg)](https://crates.io/crates/asciigraph)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)

Rust library to make lightweight ASCII line graphs ╭┈╯ in command line apps. This is a direct port of the Go package [guptarohit/asciigraph](https://github.com/guptarohit/asciigraph).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
asciigraph = "0.1.0"
```

## Usage

### Basic graph

```rust
use asciigraph::{plot, Config};

fn main() {
    let data = vec![3.0, 4.0, 9.0, 6.0, 2.0, 4.0, 5.0, 8.0, 5.0, 10.0, 2.0, 7.0, 2.0, 5.0, 6.0];
    let graph = plot(&data, Config::default());
    println!("{}", graph);
}
```

Running this example would render the following graph:
```bash
  10.00 ┤        ╭╮
   9.00 ┤ ╭╮     ││
   8.00 ┤ ││   ╭╮││
   7.00 ┤ ││   ││││╭╮
   6.00 ┤ │╰╮  ││││││ ╭
   5.00 ┤ │ │ ╭╯╰╯│││╭╯
   4.00 ┤╭╯ │╭╯   ││││
   3.00 ┼╯  ││    ││││
   2.00 ┤   ╰╯    ╰╯╰╯
```

### Multiple Series

```rust
use asciigraph::{plot_many, Config};

fn main() {
    let s1 = vec![0.0, 1.0, 2.0, 3.0, 3.0, 3.0, 2.0, 0.0];
    let s2 = vec![5.0, 4.0, 2.0, 1.0, 4.0, 6.0, 6.0];
    let data: Vec<&[f64]> = vec![&s1, &s2];

    let graph = plot_many(&data, Config::default());
    println!("{}", graph);
}
```

Running this example would render the following graph:
```bash
 6.00 ┤    ╭─
 5.00 ┼╮   │
 4.00 ┤╰╮ ╭╯
 3.00 ┤ │╭│─╮
 2.00 ┤ ╰╮│ ╰╮
 1.00 ┤╭╯╰╯  │
 0.00 ┼╯     ╰
```

### Custom Y-axis value formatting

Use `YAxisValueFormatter(...)` to control how values printed on the Y-axis are rendered.
This is useful for human-readable units like bytes, durations, or domain-specific labels.

```rust
use asciigraph::{plot, Config};

fn main() {
    let data = vec![
        30.0 * 1024.0 * 1024.0 * 1024.0,
        70.0 * 1024.0 * 1024.0 * 1024.0,
        2.0 * 1024.0 * 1024.0 * 1024.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .height(5)
            .width(45)
            .y_axis_value_formatter(Box::new(|v: f64| {
                format!("{:.2} GiB", v / 1024.0 / 1024.0 / 1024.0)
            })),
    );

    println!("{}", graph);
}
```

Running this example would render the following graph:
```bash
 70.00 GiB ┤                 ╭──────╮
 56.40 GiB ┤         ╭───────╯      ╰────╮
 42.80 GiB ┤  ╭──────╯                   ╰───╮
 29.20 GiB ┼──╯                              ╰────╮
 15.60 GiB ┤                                      ╰───╮
  2.00 GiB ┤                                          ╰─
```
### X-axis support

Use `.x_axis_range(min, max)` to add a labeled X-axis below the graph. `.x_axis_tick_count(n)` controls how many tick marks appear (default 5, minimum 2).

```rust
use asciigraph::{plot, Config};

fn main() {
    let data = vec![3.0, 4.0, 9.0, 6.0, 2.0, 4.0, 5.0, 8.0, 5.0, 10.0, 2.0, 7.0, 2.0, 5.0, 6.0];

    let graph = plot(
        &data,
        Config::default()
            .x_axis_range(0.0, 14.0)
            .x_axis_tick_count(3),
    );

    println!("{}", graph);
}
```

Running this example would render the following graph:

```
 10.00 ┤        ╭╮
  9.00 ┤ ╭╮     ││
  8.00 ┤ ││   ╭╮││
  7.00 ┤ ││   ││││╭╮
  6.00 ┤ │╰╮  ││││││ ╭
  5.00 ┤ │ │ ╭╯╰╯│││╭╯
  4.00 ┤╭╯ │╭╯   ││││
  3.00 ┼╯  ││    ││││
  2.00 ┤   ╰╯    ╰╯╰╯
       └┬──────┬──────┬
        0      7     14
```

### Colored graphs

Use `.series_colors(...)` to assign ANSI colors to each series.

```rust
use asciigraph::{plot_many, Config, AnsiColor};

fn main() {
    let data: Vec<Vec<f64>> = (0..4)
        .map(|i| {
            (-20..=20)
                .map(|x| {
                    let r = 20 - i;
                    if x >= -r && x <= r {
                        let r = r as f64;
                        let x = x as f64;
                        (r * r - x * x).sqrt() / 2.0
                    } else {
                        f64::NAN
                    }
                })
                .collect()
        })
        .collect();

    let refs: Vec<&[f64]> = data.iter().map(|s| s.as_slice()).collect();

    let graph = plot_many(
        &refs,
        Config::default()
            .precision(0)
            .series_colors(&[
                AnsiColor::RED,
                AnsiColor::YELLOW,
                AnsiColor::GREEN,
                AnsiColor::BLUE,
            ]),
    );

    println!("{}", graph);
}
```

### Legends for colored graphs

The graph can include legends for each series, making it easier to interpret.

```rust
use asciigraph::{plot_many, Config, AnsiColor};

fn main() {
    let data: Vec<Vec<f64>> = (0..3)
        .map(|i| {
            (-12..=12)
                .map(|x| {
                    let r = 12 - i;
                    if x >= -r && x <= r {
                        let r = r as f64;
                        let x = x as f64;
                        (r * r - x * x).sqrt() / 2.0
                    } else {
                        f64::NAN
                    }
                })
                .collect()
        })
        .collect();

    let refs: Vec<&[f64]> = data.iter().map(|s| s.as_slice()).collect();

    let graph = plot_many(
        &refs,
        Config::default()
            .precision(0)
            .series_colors(&[AnsiColor::RED, AnsiColor::GREEN, AnsiColor::BLUE])
            .series_legends(&["Red", "Green", "Blue"])
            .caption("Series with legends"),
    );

    println!("{}", graph);
}
```

## CLI Installation

Install the CLI binary with:

```bash
cargo install asciigraph
```

## CLI Usage

```
asciigraph --help

Usage: asciigraph [OPTIONS]

Options:
  -H, --height <HEIGHT>              height in text rows, 0 for auto-scaling [default: 0]
  -w, --width <WIDTH>                width in columns, 0 for auto-scaling [default: 0]
  -o, --offset <OFFSET>              offset in columns, for the label [default: 3]
  -p, --precision <PRECISION>        precision of data point labels along the y-axis [default: 2]
  -c, --caption <CAPTION>            caption for the graph [default: ]
  -r, --realtime                     enables realtime graph for data stream
  -b, --buffer <BUFFER>              data points buffer when realtime graph enabled [default: 0]
  -f, --fps <FPS>                    fps to control render frequency in realtime mode [default: 24]
      --sc <SERIES_COLORS>           comma-separated series colors [default: ]
      --sl <SERIES_LEGENDS>          comma-separated series legends [default: ]
      --cc <CAPTION_COLOR>           caption color of the plot [default: ]
      --ac <AXIS_COLOR>              y-axis color of the plot [default: ]
      --lc <LABEL_COLOR>             y-axis label color of the plot [default: ]
      --lb <LOWER_BOUND>             lower bound for the vertical axis [default: inf]
      --ub <UPPER_BOUND>             upper bound for the vertical axis [default: -inf]
  -d, --delimiter <DELIMITER>        data delimiter for splitting data points [default: ,]
      --sn <SERIES_NUM>              number of series (columns) in the input data [default: 1]
  -x, --custom-char <CUSTOM_CHAR>    character to use for plotting [default: ]
      --xmin <X_AXIS_MIN>            x-axis minimum value [default: NaN]
      --xmax <X_AXIS_MAX>            x-axis maximum value [default: NaN]
      --xt <X_AXIS_TICKS>            x-axis tick count [default: 5]
  -h, --help                         Print help
```

Feed data points via stdin:

```bash
seq 1 72 | asciigraph -h 10 -c "plot data from stdin" --xmin 0 --xmax 40 --xt 5
```

Output:

```
 72.00 ┤                                                                  ╭────
 64.90 ┤                                                           ╭──────╯
 57.80 ┤                                                    ╭──────╯
 50.70 ┤                                             ╭──────╯
 43.60 ┤                                      ╭──────╯
 36.50 ┤                              ╭───────╯
 29.40 ┤                       ╭──────╯
 22.30 ┤                ╭──────╯
 15.20 ┤         ╭──────╯
  8.10 ┤  ╭──────╯
  1.00 ┼──╯
       └┬─────────────────┬─────────────────┬────────────────┬─────────────────┬
        0                10                20               30                40
                                  plot data from stdin
```

For a real-time graph:

```bash
ping -i.2 google.com | grep -oP '(?<=time=).*(?=ms)' --line-buffered | asciigraph -r -H 10 -w 40 -c "realtime ping (ms)"
```

## Acknowledgement

This project is a Rust port of [guptarohit/asciigraph](https://github.com/guptarohit/asciigraph), which itself started as a Go port of [kroitor/asciichart](https://github.com/kroitor/asciichart).

## Contributing

Feel free to open issues or pull requests!

## License

BSD-3-Clause — see [LICENSE](LICENSE) for details.
