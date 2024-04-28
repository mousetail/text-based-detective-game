use crate::svg::{Color, LineStyle, TextAlign, TextStyle, TextVerticalAlign};

pub const HORIZONTAL_SPACING: usize = 32;
pub const VERTICAL_SPACING: usize = 64;
pub const LEFT_BAR_WIDTH: usize = 80;
pub const MIDDLE_BAR_WIDTH: usize = 320;
pub const RIGHT_BAR_WIDTH: usize = 320;
pub const MIN_COLUMN_WIDTH: usize = 4;

pub const EVENT_LINE_COLOR: Color = Color("#222");
pub const LOCATION_SEPERATOR_LINE_COLOR: Color = Color("#888");
pub const LOCATION_TITLE_TEXT_COLOR: Color = Color("Black");
pub const EVENT_TEXT_COLOR: Color = Color("Black");
pub const TIME_TEXT_COLOR: Color = Color("#888");

pub const CHARACTER_COLORS: &'static [Color] = &[
    Color("Red"),
    Color("Green"),
    Color("Blue"),
    Color("pink"),
    Color("Purple"),
    Color("Yellow"),
    Color("Brown"),
];

pub const LOCATION_HEADER_TEXT_STYLE: TextStyle = TextStyle {
    align: TextAlign::Center,
    size: 12,
    vertical_align: TextVerticalAlign::Baseline,
};

pub const EVENT_NAME_TEXT_STYLE: TextStyle = TextStyle {
    align: TextAlign::Left,
    size: 12,
    vertical_align: TextVerticalAlign::Baseline,
};

pub const LOCATION_SEPERATOR_LINE_STYLE: LineStyle = LineStyle {
    dash_array: &[8, 8],
    width: 2,
};

pub const EVENT_LINE_STYLE: LineStyle = LineStyle {
    dash_array: &[],
    width: 2,
};

pub const TIME_TEXT_STYLE: TextStyle = TextStyle {
    align: TextAlign::Right,
    size: 12,
    vertical_align: TextVerticalAlign::Baseline,
};
