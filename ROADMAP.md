# Roadmap

This document outlines planned and potential features for `asciigraph-rs`, ordered from easiest to most complex to implement. Contributions are welcome — feel free to open a pull request or issue for any of these.

no_std is not difficult but I will handle it closer to the end.

--
## REFACTORING
Yes, absolutely — and it is actually a great idea to do that now before adding more features, because `plot_many` is already quite long and will only get longer as you add things like grid lines, threshold lines, and zero line highlighting.

Looking at the current structure of `plot_many`, there are already natural boundaries where you could extract functions:

```
plot_many
├── data preparation
│   ├── deep copy
│   ├── padding with NaN
│   └── interpolation
├── bounds calculation
│   ├── min/max finding
│   └── lower/upper bound application
├── grid initialization
│   └── create 2D cell grid
├── precision calculation
├── y-axis rendering
│   ├── label formatting
│   └── axis characters
├── series rendering          ← the biggest chunk
│   ├── NaN gap handling
│   └── arc/line character selection
├── row joining
│   └── color and text assembly
├── x-axis rendering          ← already separate (add_x_axis)
├── caption rendering
└── legend rendering          ← already separate (add_legends)
```

You have already extracted `add_x_axis` and `add_legends` — so the pattern is established. The natural next extractions would be:

```rust
//fn prepare_data(data: &[&[f64]], config: &Config) -> Vec<Vec<f64>>//
//fn calculate_bounds(data: &[Vec<f64>], config: &Config) -> (f64, f64)
//fn calculate_precision(maximum: f64, minimum: f64, config: &Config) -> usize
//fn render_y_axis(plot: &mut Vec<Vec<Cell>>, config: &Config, ...)
//fn render_series(plot: &mut Vec<Vec<Cell>>, data: &[Vec<f64>], config: &Config, ...)
//fn join_rows(plot: &Vec<Vec<Cell>>, config: &Config) -> String
```

---

**The benefits of doing this before adding features:**

Each new feature then becomes a small, focused addition to one specific function rather than more code piled into an already long `plot_many`. For example:

- Zero line highlighting → one line in `render_y_axis`
- Threshold lines → one line in `render_y_axis`
- Grid lines → one new pass in `render_series`
- Log scale → a parameter change in `calculate_bounds`
- Statistical annotations → a new function `render_annotations`

Rather than hunting through hundreds of lines to find where to insert each feature, you would have clear, named functions that tell you exactly where each concern lives.

---

**The risk:**

Refactoring a working function always carries some risk of introducing bugs. The mitigation is straightforward though — your 67 tests are comprehensive enough that if you break anything during the refactor, they will catch it immediately. The tests make this refactoring safe to do.

Would you like to tackle the refactoring as the first step before implementing any new features?

---

---

## Trivially Easy

### 1. Zero Line Highlighting [DONE]
**What it accomplishes:** When a graph's data range spans both positive and negative values, the Y=0 baseline is drawn with a visually distinct character (e.g. `┄` or `─`) across the full width of the graph. This makes it immediately obvious where positive and negative values are relative to each other.

**Expected impact:** High usefulness for financial data, temperature graphs, profit/loss charts, or any metric where zero has special meaning. Small, self-contained change with no API additions required — could be opt-in via a `Config` option like `.zero_line(true)`.

---

### 2. Threshold Lines [DONE]
**What it accomplishes:** User specifies one or more Y values via the config and a horizontal reference line is drawn at each position. For example, drawing a line at `y = 100` to represent a target or limit.

**Expected impact:** Immediately useful for monitoring and observability use cases — CPU threshold, memory limit, SLA target. Requires a small API addition like `.threshold(value: f64)` or `.thresholds(values: &[f64])` and straightforward rendering logic.

---

### 3. Moving Average Overlay
**What it accomplishes:** Computes a rolling average over the input data and renders it as an additional series on top of the existing graph. Window size is configurable.

**Expected impact:** Useful for smoothing noisy data in financial or sensor graphs. Can be implemented entirely as a data transformation before rendering, meaning no changes to the core rendering engine are required. API addition like `.moving_average(window: usize)`.

---

### 4. Auto Tick Count
**What it accomplishes:** Instead of requiring the user to specify `x_axis_tick_count`, the library automatically calculates an appropriate number of ticks based on the available graph width and the length of the tick labels.

**Expected impact:** Improves the out-of-the-box X-axis experience. Currently users must experiment with tick counts to avoid label overlap — auto detection handles this transparently. Small calculation change with no API additions required.

---

### 5. Data Point Markers
**What it accomplishes:** Renders a configurable symbol (e.g. `●`, `○`, `×`) at each actual data point position on the graph, making individual values more visible especially when the graph is wide and data points are sparse.

**Expected impact:** Useful for discrete measurement data where the exact sample positions matter. Requires a small API addition like `.marker(char)` and minor changes to the cell rendering logic.

---

## Easy

### 6. X and Y Axis Labels
**What it accomplishes:** Adds descriptive text labels alongside the axes — a label running vertically beside the Y-axis and a label running horizontally below the X-axis. For example labeling the Y-axis as "Memory (MB)" or the X-axis as "Time (seconds)".

**Expected impact:** Makes graphs self-documenting without needing a caption to explain what the axes represent. Inspired by `baehyunsol/asciigraph` which supports this via `set_x_axis_label` and `set_y_axis_label`. Requires calculating additional padding around the plot area to accommodate the label text.

---

### 7. Serde Support
**What it accomplishes:** Adds `#[derive(Serialize, Deserialize)]` to `Config` and `AnsiColor` behind an optional `serde` feature flag, allowing graph configurations to be saved and loaded from JSON, TOML, or any other serde-compatible format.

**Expected impact:** Useful for applications that want to store and restore graph configurations, or expose graph settings through a configuration file. Requires adding `serde` as an optional dependency and annotating the relevant types. No changes to the rendering logic.

---

### 8. Statistical Annotations
**What it accomplishes:** Draws horizontal reference lines or labels for statistical values — minimum, maximum, and mean — directly on the graph. Each annotation is optionally colored and labeled.

**Expected impact:** Saves users from having to compute and manually overlay these values. Particularly useful for exploratory data analysis in the terminal. Requires small API additions and reuses calculations already performed internally during rendering.

---

### 9. Step Graphs
**What it accomplishes:** Instead of diagonal arc characters connecting data points, step graphs draw flat horizontal segments followed by vertical jumps. This accurately represents discrete or sampled data where values do not change continuously between measurements.

**Expected impact:** More accurate representation for sensor readings, state changes, or any data that holds a constant value between samples. Requires a new `SeriesStyle` option and changes to the character selection logic in the plotting loop.

---

### 10. Better Error Handling
**What it accomplishes:** Changes `plot` and `plot_many` to return `Result<String, AsciigraphError>` instead of panicking on invalid input such as empty data, mismatched series lengths, or invalid configuration values. Defines a proper `AsciigraphError` enum with meaningful variants.

**Expected impact:** Makes the library safe to use in production code where panics are unacceptable. Allows callers to handle errors gracefully rather than crashing. This is a breaking API change that would require a version bump to `0.2.0` under semantic versioning.

---

### 11. Right-to-Left Rendering
**What it accomplishes:** Adds an option to render the graph with the most recent data point on the left rather than the right, which is the natural reading direction for some use cases like real-time monitoring where the newest data should be prominent.

**Expected impact:** Useful for right-to-left language users and for certain monitoring dashboards. Requires reversing the order of data processing and adjusting axis label positioning.

---

### 12. Export to File
**What it accomplishes:** Adds a convenience function that writes the graph output directly to a file path rather than returning a `String`. Supports both plain text and ANSI color output.

**Expected impact:** Small quality-of-life improvement for CLI tools that want to save graph snapshots. Trivial to implement as a thin wrapper over the existing `plot` and `plot_many` functions.

---

## Moderate

### 13. Skip Ranges
**What it accomplishes:** Allows the user to specify Y-axis value ranges that should be visually compressed or skipped in the rendering — useful when data has extreme outliers that would otherwise compress the rest of the graph into a tiny area. A visual break marker (e.g. `~~~~~`) is drawn where the skip occurs.

**Expected impact:** Significantly improves readability for data with outliers. Inspired by `baehyunsol/asciigraph` which implements this via `SkipValue`. Without this feature, a single extreme value forces the entire Y-axis scale to expand, making all other values appear flat. Requires changes to the Y-axis scale calculation and a new rendering pass for the break marker.

---

### 14. Side-by-Side Graph Layout
**What it accomplishes:** Adds a `merge_horizontal` function that takes two or more rendered graph strings and merges them into a single side-by-side layout, aligning their heights automatically. Useful for comparing two related metrics at a glance without stacking them vertically.

**Expected impact:** Genuinely unique feature for terminal graph libraries — `baehyunsol/asciigraph` is the only other Rust library with this via their `merge_horiz` function. Particularly useful for dashboards, benchmarks, and before/after comparisons. Requires careful string manipulation to align multi-line strings horizontally while respecting ANSI color codes.

---

### 15. Y-Axis on the Right Side
**What it accomplishes:** Adds a config option to render the Y-axis labels and tick marks on the right side of the graph instead of the left. Useful when composing multiple graphs side by side where the left margin would be redundant.

**Expected impact:** Moderate usefulness for dashboard-style layouts. Requires restructuring how the label padding and axis characters are positioned in the grid, but does not fundamentally change the rendering model.

---

### 16. Grid Lines
**What it accomplishes:** Draws faint horizontal reference lines across the full width of the graph at each Y-axis tick position, using a configurable character like `·` or `╌`. Makes it easier to read values at a glance without tracing across the graph.

**Expected impact:** Significantly improves readability for tall graphs or graphs with many series. Requires rendering an additional character layer beneath the plot characters, with logic to avoid overwriting actual plot characters.

---

### 17. Labeled X-Axis Intervals
**What it accomplishes:** Allows the user to annotate specific ranges on the X-axis with descriptive labels rendered below the tick marks. For example labeling a time range as "first pi" or "maintenance window". Multiple overlapping intervals are supported with automatic vertical stacking.

**Expected impact:** Useful for annotating events, phases, or significant periods directly on the graph without needing external documentation. Inspired by `baehyunsol/asciigraph` which supports this via `add_labeled_interval`. Requires additional rendering rows below the X-axis tick labels.

---

### 18. Benchmark Suite
**What it accomplishes:** Adds a comprehensive benchmark suite using the `criterion` crate measuring the performance of `plot`, `plot_many`, interpolation, and legend rendering across a range of data sizes.

**Expected impact:** Establishes a performance baseline and makes it easy to detect regressions when making changes. Required for any serious performance optimization work. Requires learning the `criterion` API and creating a `benches/` folder.

---

### 19. Log Scale Y-Axis
**What it accomplishes:** Adds a config option to render the Y-axis on a logarithmic scale rather than a linear scale. Data points are mapped to row positions using `log10` instead of linear interpolation, compressing large ranges into a readable space.

**Expected impact:** Essential for data that spans multiple orders of magnitude — network traffic, memory usage, or any exponentially growing metric. Requires changing the core Y-position calculation and updating Y-axis label formatting.

---

### 20. Streaming Iterator Input
**What it accomplishes:** Changes the function signatures to accept `impl Iterator<Item = f64>` in addition to `&[f64]`, allowing data to be passed lazily without first collecting it into a `Vec`. Large datasets can be rendered without loading everything into memory.

**Expected impact:** Significant improvement for memory efficiency with large datasets. Requires rethinking parts of the rendering pipeline that currently assume random access into the data, particularly the interpolation and min/max calculation steps.

---

### 21. Area Graphs
**What it accomplishes:** Fills the region between the plot line and the X-axis (or zero line) with a configurable character pattern, creating a filled area graph effect in the terminal.

**Expected impact:** More visually striking than a line graph for cumulative or volume data. Requires an additional rendering pass to fill cells below the plot line and above the baseline, with color support.

---

### 22. Horizontal Bar Charts
**What it accomplishes:** Adds a new `plot_bar` function that renders data as horizontal bars extending from the Y-axis rather than as a connected line. Each data point becomes a row with a bar of proportional width.

**Expected impact:** Useful for comparing discrete categories — benchmark results, file sizes, survey responses. Requires a fundamentally different rendering approach since bars grow horizontally rather than the graph growing vertically.

---

## Hard

### 23. Dual Y-Axis
**What it accomplishes:** Allows two series with very different value ranges to be plotted on the same graph, each with its own independent Y-axis scale — one on the left and one on the right.

**Expected impact:** High value for financial or scientific graphs where two related but differently-scaled metrics need to be compared visually. Requires significant rethinking of the grid layout and label positioning to accommodate two independent scales simultaneously.

---

### 24. Date and Time X-Axis Formatting
**What it accomplishes:** Adds support for treating X-axis values as Unix timestamps and automatically formatting tick labels as human-readable dates and times. Supports configurable format strings like `%Y-%m-%d` or `%H:%M:%S`.

**Expected impact:** Makes the library immediately useful for time-series data which is one of the most common use cases for terminal graphs. Requires either adding `chrono` as an optional dependency or implementing basic date formatting from scratch.

---

### 25. `no_std` Support with `alloc`
**What it accomplishes:** Allows the library to be used in embedded systems and other environments where the full Rust standard library is not available. Replaces `std` types with their `alloc` equivalents (`String`, `Vec`, `Box`) and removes any `std`-only dependencies. The CLI binary (`main.rs`) is unaffected since binaries always have `std` available.

**Expected impact:** Significantly expands the potential user base to include embedded systems developers and WebAssembly users. The library itself has no external dependencies so the main work is changing import paths from `std::` to `alloc::` or `core::` and verifying compilation against a `no_std` target like `thumbv7em-none-eabihf`.

--
You are right — with that understanding the difficulty comes down significantly. Let me revise the assessment.

The actual work is:

**1. `lib.rs` — add two lines**
```rust
#![no_std]
extern crate alloc;
```

**2. All library files — change import paths**
```rust
// find and replace across all src/ files except main.rs
use std::fmt;        //→ use core::fmt
use std::string;     //→ use alloc::string
use std::vec;        //→ use alloc::vec
use std::format;     //→ use alloc::format
use std::string::String; //→ use alloc::string::String
```

**3. `main.rs` — completely untouched**
The CLI binary keeps `std` exactly as it is. No changes needed.

**4. `Cargo.toml` — mark std as optional**
```toml
[features]
default = ["std"]
std = []
```

---

The real remaining complexity is just two things:

- **Verifying it compiles** against a `no_std` target which requires installing an additional Rust target with `rustup target add thumbv7em-none-eabihf` and running `cargo build --target thumbv7em-none-eabihf --no-default-features`
- **Documenting** that users need to provide their own global allocator

That is it. Realistically this is a **moderate** difficulty feature, not hard. The hard rating was inflated by assuming you would also need to support embedded targets with no heap at all — but since you are keeping `alloc`, you sidestep the truly hard parts like replacing `String` and `Vec` with fixed-size buffers.

It could reasonably be your next meaningful feature after the easier ones.

---

### 26. Scatter Plots
**What it accomplishes:** Adds a `plot_scatter` function that renders individual data points as symbols at their `(x, y)` coordinates without connecting lines between them. Accepts `Vec<(f64, f64)>` pairs rather than a plain `Vec<f64>`.

**Expected impact:** Useful for correlation analysis, distribution visualization, and any data where the relationship between X and Y values matters independently. Requires a different data model and a fundamentally different rendering approach since there is no concept of consecutive points to connect.

---

### 27. HTML Output
**What it accomplishes:** Adds a `plot_html` function that renders the graph as an HTML `<pre>` block with inline CSS color styling instead of ANSI escape codes. The output can be embedded directly in a web page or served from a web server.

**Expected impact:** Opens up the library to web use cases — embedding terminal-style graphs in documentation, dashboards, or web reports without requiring ANSI support. Requires a new rendering target that maps `AnsiColor` values to CSS `color` properties.

---

### 28. Candlestick Charts
**What it accomplishes:** Adds a `plot_candlestick` function that renders OHLC (Open, High, Low, Close) financial data as ASCII candlestick charts. Each candle shows the open/close body and high/low wicks using box-drawing characters.

**Expected impact:** Makes the library useful for financial data visualization in the terminal. Requires a completely new data model accepting `Vec<(f64, f64, f64, f64)>` tuples and a new rendering approach for the candle body and wick characters.

---

## Very Hard

### 29. SVG Output
**What it accomplishes:** Adds a `plot_svg` function that renders the graph as a valid SVG file with precise vector coordinates, proper fonts, and CSS styling. The output is a scalable, printable graph suitable for documentation or reports.

**Expected impact:** Dramatically expands the library's output capabilities beyond the terminal. Requires generating valid XML, mapping the ASCII grid coordinates to SVG viewport coordinates, and handling text rendering, color, and stroke properties through SVG attributes.

---

### 30. WASM Support
**What it accomplishes:** Compiles the library to WebAssembly and exposes JavaScript bindings via `wasm-bindgen`, allowing the library to be used directly in web browsers and Node.js applications without a server. Requires `no_std` support as a prerequisite.

**Expected impact:** Opens up the entire JavaScript ecosystem as potential users. Requires `wasm-bindgen` integration, JavaScript type mappings, and an npm package for distribution.

---

### 31. Gradient Colors
**What it accomplishes:** Colors each segment of a plot line based on its value rather than a flat per-series color. High values could render in red and low values in blue, with smooth interpolation between colors giving a heatmap-like appearance.

**Expected impact:** Visually striking and immediately informative for data where the color itself encodes meaning. Requires computing color interpolation across the ANSI 256-color palette for each cell, with significant changes to how colors are assigned during the rendering pass.

---

### 32. Procedural Derive Macro
**What it accomplishes:** Adds a `#[derive(AsciigraphConfig)]` procedural macro that automatically generates a `Config` builder from a user-defined struct, mapping struct fields to graph configuration options through attributes.

**Expected impact:** Significant developer experience improvement for applications that have their own configuration structs and want to drive graph rendering from them without manual conversion. Requires creating a separate `asciigraph-rs-derive` crate and learning the `proc-macro`, `syn`, and `quote` crates which are an advanced Rust topic.

---

## Contributing

If you would like to work on any of these features, please open a GitHub issue first to discuss the approach before submitting a pull request. This helps avoid duplicate work and ensures the implementation aligns with the project's design goals.

For bug reports, feature requests not listed here, or general feedback, feel free to open an issue at [github.com/neneodonkor/asciigraph-rs](https://github.com/neneodonkor/asciigraph-rs).