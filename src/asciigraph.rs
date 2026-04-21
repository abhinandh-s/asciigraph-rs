// The main file that plots the graph.

use crate::{options, utils, AnsiColor};
use crate::options::{CharSet, Config, DEFAULT_CHAR_SET};
use crate::utils::{calculate_height, interpolate_array, min_max_float64_slice, round};

/// Plot returns ascii graph for a series.
pub fn plot(series: &[f64], options: &[Config]) -> String {
    plot_many(&mut [series], options)
}

/// get_char_set returns the CharSet for a given series index, falling back to DEFAULT_CHAR_SET.
pub(crate) fn get_char_set(config: &Config, series_index: usize) -> CharSet {
    if series_index < config.series_chars.len() {
        let mut char_set = config.series_chars[series_index].clone();

        if char_set.horizontal.is_empty() {
            char_set.horizontal = DEFAULT_CHAR_SET.horizontal;
        }

        if char_set.vertical_line.is_empty() {
            char_set.vertical_line = DEFAULT_CHAR_SET.vertical_line;
        }

        if char_set.arc_down_right.is_empty() {
            char_set.arc_down_right = DEFAULT_CHAR_SET.arc_down_right
        }

        if char_set.arc_down_left.is_empty() {
            char_set.arc_down_left = DEFAULT_CHAR_SET.arc_down_left;
        }

        if char_set.arc_up_right.is_empty() {
            char_set.arc_up_right = DEFAULT_CHAR_SET.arc_up_right;
        }

        if char_set.arc_up_left.is_empty() {
            char_set.arc_up_left = DEFAULT_CHAR_SET.arc_up_left;
        }

        if char_set.end_cap.is_empty() {
            char_set.end_cap = DEFAULT_CHAR_SET.end_cap;
        }

        if char_set.start_cap.is_empty() {
            char_set.start_cap = DEFAULT_CHAR_SET.start_cap;
        }

        if char_set.up_right.is_empty() {
            char_set.up_right = DEFAULT_CHAR_SET.up_right;
        }

        if char_set.down_horizontal.is_empty() {
            char_set.down_horizontal = DEFAULT_CHAR_SET.down_horizontal;
        }

        return char_set;
    }

    DEFAULT_CHAR_SET
}

pub fn plot_many(data: &[&[f64]], options: &[Config]) -> String {
    let log_maximum: f64;
    let mut config = Config {
        offset: 3,
        precision: None,
        line_ending: "\n".to_string(),
        ..Default::default()
    };

    // deep copy on input data — from here on we work with owned data
    let mut data: Vec<Vec<f64>> = data.iter().map(|s| s.to_vec()).collect();

    // now len_max can use the owned data
    let mut len_max: usize = 0;
    for series in data.iter() {
        len_max = len_max.max(series.len());

        // This is the same as this:
        /*let l = data[i].len();
        if l > len_max {
            len_max = l;
        }*/
    }

    // padding and interpolation
    if config.width > 0 {
        for i in 0..data.len() {
            while data[i].len() < len_max {
                data[i].push(f64::NAN);
            }

            data[i] = interpolate_array(&data[i], config.width);
        }

        len_max = config.width as usize;
    }

    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;

    for i in 0..data.len() {
        let values = min_max_float64_slice(&data[i]);

        match values {
            Some((min_value, max_value)) => {
                if min_value < minimum {
                    minimum = min_value;
                }

                if max_value > maximum {
                    maximum = max_value;
                }
            },
            None => println!("Values were not provided"),
        }
    }

    // Not sure if using unwrap is safe here.
    if config.lower_bound != None && config.lower_bound.unwrap() < minimum {
        minimum = config.lower_bound.unwrap();
    }

    if config.upper_bound != None && config.upper_bound.unwrap() > maximum {
        maximum = config.upper_bound.unwrap();
    }

    let interval = (maximum - minimum).abs();

    if config.height <= 0 {
        config.height = calculate_height(interval)
    }

    if config.offset <= 0 {
        config.offset = 3;
    }

    let mut ratio: f64;
    if interval != 0.0 {
        ratio = f64::from(config.height) / interval;
    } else {
        ratio = 1.0;
    }

    let min2 = utils::round(minimum * ratio);
    let max2 = utils::round(maximum * ratio);
    let intmin2 = min2.round() as usize;
    let intmax2 = max2.round() as usize;

    let rows = ((intmax2 - intmin2) as f64).abs() as usize;
    let width = len_max + config.offset as usize;

    #[derive(Clone, Default)]
    struct Cell {
        text: String,
        color: AnsiColor
    }

    let mut plot: Vec<Vec<Cell>> = vec![vec![Cell::default()]; rows + 1];

    // I HAVE A FEELING THAT I DON'T NEED TO USE A LOOP TO INITIALIZE.
    // LET ME KNOW IF I AM RIGHT, CLAUDE.

    // initialize empty 2D grid.
    let mut idx = 0;
    while idx < rows + 1 {
        // If you do not want to use the Derive macro.
        // let line = vec![Cell { text: String::new(), color: AnsiColor::Default }; width];
        let mut line = vec![Cell::default(); width];

        let idx2 = 0;
        while idx2 < width {
            line[idx2].text = " ".to_string();
            line[idx2].color = AnsiColor::DEFAULT;

            idx += 1;
        }

        plot[idx] = line;
        idx += 1;
    }

    // Default precision to maintain backwards compatibility.
    let mut precision: usize = 2;

    if config.precision.is_some() {
        precision = config.precision.unwrap() as usize;
    }

    // To find number of zeros after decimal
    log_maximum = (maximum.abs().max(minimum.abs())).log10();


    "hello".to_string() // Just to make the compiler quiet
}