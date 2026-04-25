use asciigraph::{plot_many, Config, AnsiColor};

fn main() {
    let data: Vec<Vec<f64>> = (0..6)
        .map(|i| {
            (-40..=40)
                .map(|x| {
                    let r = 40 - i;

                    if x >= -r && x <= r {
                        let r = r as f64;
                        let x = x as f64;
                        // Semi-circle formula: y = sqrt(r² - x²) / 2
                        (r * r - x * x).sqrt() / 2.0
                    } else {
                        f64::NAN
                    }
                }).collect()
        }).collect();

    // Convert each owned Vec<f64> into a &[f64] slice reference for plot_many
    let refs: Vec<&[f64]> = data.iter().map(|s| s.as_slice()).collect();

    let graph = plot_many(
        &refs,
        Config::default()
            .precision(0)
            .series_colors(&[
                AnsiColor::RED,
                AnsiColor::ORANGE,
                AnsiColor::YELLOW,
                AnsiColor::GREEN,
                AnsiColor::BLUE,
                AnsiColor::PURPLE
            ])
            .series_legends(&[
                "Red",
                "Orange",
                "Yellow",
                "Green",
                "Blue",
                "Purple"
            ])
            .caption("Rainbow with color legends")
    );

    println!("{graph}");

    // Output:
    //   20 ┤
    //   20 ┤                               ╭───────╭╮───────╮
    //   19 ┤                        ╭──╭───╭───────╭╮───────╮───╮──╮
    //   18 ┤                    ╭─╭──╭─╭───╭───────╭╮───────╮───╮─╮──╮─╮
    //   17 ┤                 ╭─╭─╭─╭─╭──╭──────────╯╰──────────╮──╮─╮─╮─╮─╮
    //   16 ┤              ╭─╭─╭╭─╭─╭────╯                      ╰────╮─╮─╮╮─╮─╮
    //   15 ┤            ╭╭─╭─╭╭─╭──╯                                ╰──╮─╮╮─╮─╮╮
    //   14 ┤          ╭╭─╭╭─╭╭──╯                                      ╰──╮╮─╮╮─╮╮
    //   13 ┤        ╭─╭╭╭─╭╭─╯                                            ╰─╮╮─╮╮╮─╮
    //   12 ┤       ╭╭╭─╭╭╭─╯                                                ╰─╮╮╮─╮╮╮
    //   11 ┤     ╭─╭╭╭╭╭─╯                                                    ╰─╮╮╮╮╮─╮
    //   10 ┤    ╭╭─╭╭╭╭╯                                                        ╰╮╮╮╮─╮╮
    //    9 ┤   ╭╭╯╭╭╭╭╯                                                          ╰╮╮╮╮╰╮╮
    //    8 ┤  ╭╭╯╭╭╭╭╯                                                            ╰╮╮╮╮╰╮╮
    //    7 ┤  ││╭╭╭╭╯                                                              ╰╮╮╮╮││
    //    6 ┤ ╭╭╭╭╭╭╯                                                                ╰╮╮╮╮╮╮
    //    5 ┤ ││││││                                                                  ││││││
    //    4 ┤╭╭╭╭╭╭╯                                                                  ╰╮╮╮╮╮╮
    //    3 ┤││││││                                                                    ││││││
    //    2 ┤││││││                                                                    ││││││
    //    1 ┤││││││                                                                    ││││││
    //    0 ┼╶╶╶╶╶╯                                                                    ╰╴╴╴╴╴
    //                                  Rainbow with color legends
    //
    //                   ■ Red   ■ Orange   ■ Yellow   ■ Green   ■ Blue   ■ Purple
}