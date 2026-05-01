# Rust Port of asciigraph

[![Crates.io](https://img.shields.io/crates/v/asciigraph-rs.svg)](https://crates.io/crates/asciigraph-rs)
[![Docs.rs](https://docs.rs/asciigraph-rs/badge.svg)](https://docs.rs/asciigraph-rs)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)

Rust library to make lightweight ASCII line graphs ╭┈╯ in command line apps. This port is a complete and 
faithful implementation of the Go original (version 0.9.0) [guptarohit/asciigraph]
(https://github.com/guptarohit/asciigraph),
supporting:

- Single and multi-series plots
- ANSI color support for series, axes, labels, and captions
- Series legends
- X-axis with configurable tick labels
- Custom character sets for plot lines
- Y-axis and X-axis value formatters
- NaN gap handling with proper start and end caps
- Lower and upper bound constraints
- Configurable precision for Y-axis labels
- Line ending configuration (CRLF support)
- A full CLI binary with realtime streaming support and configurable FPS

Full API documentation is available on [docs.rs](https://docs.rs/asciigraph-rs).
[Changelog](CHANGELOG.md)

## Installation

Run the following command in your project directory:

```bash
cargo add asciigraph-rs
```

Or manually add this to your `Cargo.toml`:

```toml
[dependencies]
asciigraph-rs = "0.1.0"
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
```
 6.00 ┤    ╭─
 5.00 ┼╮   │
 4.00 ┤╰╮ ╭╯
 3.00 ┤ │╭│─╮
 2.00 ┤ ╰╮│ ╰╮
 1.00 ┤╭╯╰╯  │
 0.00 ┼╯     ╰
```

### Custom Y-axis value formatting

Use `.y_axis_value_formatter(...)` to control how values printed on the Y-axis are rendered.
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
```
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

### Zero-line highlighting

Opt in to a horizontal reference line at Y = 0.0 by passing a `ZeroLine` value
to `Config::zero_line()`. The line is only rendered when the data range straddles
zero — if all values are positive or all negative, the option has no effect.

```rust
use asciigraph::{plot, Config, ZeroLine};

fn main() {
    let data = vec![
        3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -3.0, -2.0, -1.0, 0.0,
        1.0, 2.0, 3.0, 2.0, 1.0, 0.0, -1.0, -2.0, -1.0, 0.0,
        1.0, 3.0, 5.0, 3.0, 1.0, -1.0, -3.0, -5.0, -3.0, -1.0,
    ];
    let graph = plot(&data, Config::default().zero_line(ZeroLine::new()));
    println!("{}", graph);
}
```

Running this example renders the following graph:

```
  5.00 ┤                     ╭╮
  4.00 ┤                     ││
  3.00 ┼╮          ╭╮       ╭╯╰╮
  2.00 ┤╰╮        ╭╯╰╮      │  │
  1.00 ┤ ╰╮      ╭╯  ╰╮    ╭╯  ╰╮
  0.00 ┤──╰╮────╭╯────╰╮──╭╯────│─────
 -1.00 ┤   ╰╮  ╭╯      ╰╮╭╯     ╰╮  ╭
 -2.00 ┤    ╰╮╭╯        ╰╯       │  │
 -3.00 ┤     ╰╯                  ╰╮╭╯
 -4.00 ┤                          ││
 -5.00 ┤                          ╰╯
```

The `─` characters along the `0.00` row are the zero line filling the empty
cells. Series arc characters take priority and render on top of the zero line
where they overlap.

To render the zero line in a specific color:

```rust
use asciigraph::{plot, Config, ZeroLine, AnsiColor};

fn main() {
    let graph = plot(
        &data,
        Config::default().zero_line(ZeroLine::with_color(AnsiColor::RED)),
    );
    println!("{}", graph);
}
````

### Threshold lines

Add one or more horizontal reference lines at user-specified Y values using
`Config::threshold()`. Each threshold is rendered as a dashed line (`╌`) and
carries its own color independently. Call `.threshold()` multiple times to add
more than one line.

```rust
use asciigraph::{plot, Config, Threshold, AnsiColor};

fn main() {
    let data = vec![
        60.0, 65.0, 72.0, 78.0, 85.0, 91.0, 88.0, 76.0, 70.0, 64.0,
        68.0, 75.0, 82.0, 89.0, 94.0, 87.0, 79.0, 71.0, 66.0, 60.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .threshold(Threshold::with_color(80.0, AnsiColor::YELLOW))
            .threshold(Threshold::with_color(90.0, AnsiColor::RED)),
    );

    println!("{}", graph);
}
```

Running this example renders the following graph:

```
 94.00 ┤             ╭╮
 93.00 ┤             ││
 92.00 ┤             ││
 91.00 ┤    ╭╮       ││
 90.00 ┤╌╌╌╌││╌╌╌╌╌╌╌││╌╌╌╌╌
 89.00 ┤    ││      ╭╯│
 88.00 ┤    │╰╮     │ │
 87.00 ┤    │ │     │ ╰╮
 86.00 ┤    │ │     │  │
 85.00 ┤   ╭╯ │     │  │
 84.00 ┤   │  │     │  │
 83.00 ┤   │  │     │  │
 82.00 ┤   │  │    ╭╯  │
 81.00 ┤   │  │    │   │
 80.00 ┤╌╌╌│╌╌│╌╌╌╌│╌╌╌│╌╌╌╌
 79.00 ┤   │  │    │   ╰╮
 78.00 ┤  ╭╯  │    │    │
 77.00 ┤  │   │    │    │
 76.00 ┤  │   ╰╮   │    │
 75.00 ┤  │    │  ╭╯    │
 74.00 ┤  │    │  │     │
 73.00 ┤  │    │  │     │
 72.00 ┤ ╭╯    │  │     │
 71.00 ┤ │     │  │     ╰╮
 70.00 ┤ │     ╰╮ │      │
 69.00 ┤ │      │ │      │
 68.00 ┤ │      │╭╯      │
 67.00 ┤ │      ││       │
 66.00 ┤ │      ││       ╰╮
 65.00 ┤╭╯      ││        │
 64.00 ┤│       ╰╯        │
 63.00 ┤│                 │
 62.00 ┤│                 │
 61.00 ┤│                 │
 60.00 ┼╯                 ╰
```

Thresholds outside the visible Y range are silently ignored. Series arc
characters always render on top of threshold lines where they overlap.

### Moving average overlay

Add a smoothed trend line on top of your data using `Config::moving_average()`.
The moving average is computed over a sliding window and rendered as an
additional series. Pair it with `series_colors` to visually distinguish the
smoothed series from the raw data.

```rust
use asciigraph::{plot, Config, AnsiColor};

fn main() {
    let data = vec![
        3.0, 1.0, 5.0, 2.0, 8.0, 4.0, 7.0, 2.0, 6.0, 3.0,
        9.0, 4.0, 6.0, 2.0, 7.0, 3.0, 8.0, 1.0, 5.0, 3.0,
    ];

    let graph = plot(
        &data,
        Config::default()
            .moving_average(5)
            .series_colors(&[AnsiColor::DEFAULT, AnsiColor::YELLOW]),
    );

    println!("{}", graph);
}
```

Running this example renders the following graph:

```
 9.00 ┤         ╭╮
 8.00 ┤   ╭╮    ││    ╭╮
 7.00 ┤   ││╭╮  ││  ╭╮││
 6.00 ┤   ││││╭╮╭╮╭╮││││
 5.00 ┤ ╭╮╭──╮╭─╯╰╯│╭╮╭╮╭╮
 4.00 ┤ ╭─╯╰╯╰╯││╰╯╰╯╰╯╰─╮
 3.00 ┼─╯││  ││╰╯  ││╰╯││╰
 2.00 ┤││╰╯  ╰╯    ╰╯  ││
 1.00 ┤╰╯              ╰╯
```

The raw data is rendered in the default color. The yellow series is the
5-point moving average, showing the underlying trend with short-term noise
smoothed out.

## CLI Installation

Install the CLI binary with:

```bash
cargo install asciigraph-rs
```

## CLI Usage

```
asciigraph --help

Usage: asciigraph [OPTIONS]

Options:
  -h, --height <HEIGHT>              height in text rows, 0 for auto-scaling [default: 0]
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
      --help                         Print help
```

Feed data points via stdin:

**Linux/macOS:**
```bash
seq 1 72 | asciigraph -h 10 -c "plot data from stdin" --xmin 0 --xmax 40 --xt 5
```

**Windows:**
```powershell
1..72 | ForEach-Object { $_ } | asciigraph -h 10 -c "plot data from stdin" --xmin 0 --xmax 40 --xt 5
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

### Realtime graphs

The CLI supports streaming data in realtime using the `-r` flag.
Data is read line by line from stdin, and the graph re-renders
at the specified FPS.

**Linux/macOS:**
```bash
ping google.com | grep -oP '(?<=time=).*(?=ms)' --line-buffered | asciigraph -r -h 10 -w 40 -c "ping (ms)"
```

**Windows:**

A built-in data generator (demo) is included for testing and demonstrating
realtime mode. Build and run it with:

```powershell
cargo build
.\target\debug\datagen.exe
```

This generates random data and pipes it directly into `asciigraph`
with realtime rendering enabled. The generator bypasses Windows pipe
buffering by writing directly to the child process stdin, which is
required for smooth realtime updates on Windows.

For your own data sources on Windows, pipe buffering can prevent
realtime updates from rendering correctly. The recommended approach
is to write a small program in any language that flushes stdout
explicitly after each line, similar to how `datagen.rs` works.

## Acknowledgement

This project is a Rust port of [guptarohit/asciigraph](https://github.com/guptarohit/asciigraph), which itself started as a Go port of [kroitor/asciichart](https://github.com/kroitor/asciichart).

## Contributing

Feel free to open issues or pull requests!

## License

BSD-3-Clause — see [LICENSE](LICENSE) for details.
