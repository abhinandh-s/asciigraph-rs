// CharSet — the set of box-drawing characters used to render a data series.

// ---------------------------------------------------------------------------
// CharSet
// ---------------------------------------------------------------------------

/// Defines the set of characters used to draw a data series on the graph.
///
/// Each field controls a specific part of the line rendering — horizontal
/// runs, vertical segments, corner arcs, NaN gap caps, axis corners, and
/// tick marks. Swap out individual characters to change the visual style of
/// a series without affecting the rendering logic.
///
/// Use [`create_char_set`] to create a uniform set where every character is
/// the same (e.g. `*` or `•`). Use struct update syntax (`..Default::default()`)
/// to override only the fields you care about while keeping the rest as the
/// defaults from [`DEFAULT_CHAR_SET`].
///
/// # Example
///
/// ```rust
/// use asciigraph::options::CharSet;
///
/// // Override only the horizontal and vertical characters.
/// let partial = CharSet {
///     horizontal: '=',
///     vertical_line: '|',
///     ..Default::default()
/// };
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct CharSet {
    /// Horizontal line character used for flat segments. Default: `─`
    pub horizontal: char,

    /// Vertical line character used for steep segments. Default: `│`
    pub vertical_line: char,

    /// Corner arc going down and to the right (rising series). Default: `╭`
    pub arc_down_right: char,

    /// Corner arc going down and to the left (falling series). Default: `╮`
    pub arc_down_left: char,

    /// Corner arc going up and to the right (falling series). Default: `╰`
    pub arc_up_right: char,

    /// Corner arc going up and to the left (rising series). Default: `╯`
    pub arc_up_left: char,

    /// End cap drawn at the last finite point before a NaN gap. Default: `╴`
    pub end_cap: char,

    /// Start cap drawn at the first finite point after a NaN gap. Default: `╶`
    pub start_cap: char,

    /// Bottom-left corner character for the X-axis. Default: `└`
    pub up_right: char,

    /// Tick mark character used on the X-axis. Default: `┬`
    pub down_horizontal: char,

    /// Dashed horizontal character used for threshold lines. Default: `╌`
    pub dash_horizontal: char,

    /// Dashed horizontal character used for mean annotation. Default: `┄`
    pub double_dash_horizontal: char,

    /// Heavy dashed horizontal character used for median annotation. Default: `╍`
    pub heavy_dash_horizontal: char,

    /// Dotted horizontal character used for standard deviation annotation. Default: `·`
    pub dot_horizontal: char,
}

impl Default for CharSet {
    fn default() -> Self {
        // Delegate to the constant so there is exactly one place where
        // the actual character values are defined.
        CharSet {
            horizontal:             DEFAULT_CHAR_SET.horizontal,
            vertical_line:          DEFAULT_CHAR_SET.vertical_line,
            arc_down_right:         DEFAULT_CHAR_SET.arc_down_right,
            arc_down_left:          DEFAULT_CHAR_SET.arc_down_left,
            arc_up_right:           DEFAULT_CHAR_SET.arc_up_right,
            arc_up_left:            DEFAULT_CHAR_SET.arc_up_left,
            end_cap:                DEFAULT_CHAR_SET.end_cap,
            start_cap:              DEFAULT_CHAR_SET.start_cap,
            up_right:               DEFAULT_CHAR_SET.up_right,
            down_horizontal:        DEFAULT_CHAR_SET.down_horizontal,
            dash_horizontal:        DEFAULT_CHAR_SET.dash_horizontal,
            double_dash_horizontal: DEFAULT_CHAR_SET.double_dash_horizontal,
            heavy_dash_horizontal:  DEFAULT_CHAR_SET.heavy_dash_horizontal,
            dot_horizontal:         DEFAULT_CHAR_SET.dot_horizontal,
        }
    }
}

/// The default box-drawing character set used when no custom [`CharSet`] is provided.
pub const DEFAULT_CHAR_SET: CharSet = CharSet {
    horizontal:             '─',
    vertical_line:          '│',
    arc_down_right:         '╭',
    arc_down_left:          '╮',
    arc_up_right:           '╰',
    arc_up_left:            '╯',
    end_cap:                '╴',
    start_cap:              '╶',
    up_right:               '└',
    down_horizontal:        '┬',
    dash_horizontal:        '╌',
    double_dash_horizontal: '┄',
    heavy_dash_horizontal:  '╍',
    dot_horizontal:         '·',
};

// ---------------------------------------------------------------------------
// create_char_set
// ---------------------------------------------------------------------------

/// Creates a [`CharSet`] where every character is set to the same value.
///
/// Useful for simple, uniform plot styles such as `*`, `•`, or `#`, where
/// the distinction between horizontal runs, vertical segments, and arcs is
/// not important — every position in the series uses the same character.
///
/// # Example
///
/// ```rust
/// use asciigraph::options::create_char_set;
///
/// let asterisk = create_char_set('*');
/// let dot = create_char_set('•');
/// ```
pub fn create_char_set(character: char) -> CharSet {
    CharSet {
        horizontal:             character,
        vertical_line:          character,
        arc_down_right:         character,
        arc_down_left:          character,
        arc_up_right:           character,
        arc_up_left:            character,
        end_cap:                character,
        start_cap:              character,
        up_right:               character,
        down_horizontal:        character,
        dash_horizontal:        character,
        double_dash_horizontal: character,
        heavy_dash_horizontal:  character,
        dot_horizontal:         character,
    }
}