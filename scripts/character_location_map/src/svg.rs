use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct Color(pub &'static str);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector {
    pub x: usize,
    pub y: usize,
}

pub enum SvgElement {
    Circle {
        color: Color,
        position: Vector,
    },
    Curve {
        color: Color,
        points: Vec<Vector>,
    },
    Line {
        color: Color,
        points: Vec<Vector>,
        style: LineStyle,
    },
    Text {
        color: Color,
        position: Vector,
        content: String,
        style: TextStyle,
    },
}

pub struct Svg {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<SvgElement>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LineStyle {
    pub dash_array: &'static [usize],
    pub width: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl Display for TextAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAlign::Left => write!(f, "start"),
            TextAlign::Center => write!(f, "middle"),
            TextAlign::Right => write!(f, "end"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TextVerticalAlign {
    Top,
    Baseline,
}

impl Display for TextVerticalAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextVerticalAlign::Top => write!(f, "hanging"),
            TextVerticalAlign::Baseline => write!(f, "alphabetic"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TextStyle {
    pub size: usize,
    pub align: TextAlign,
    pub vertical_align: TextVerticalAlign,
}

impl Svg {
    pub fn write(&self, filename: String) -> std::io::Result<()> {
        println!("About to create file");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("out.svg")?;
        println!("File created");
        write!(
            file,
            r#"
        <svg version="1.1"
            width="{width}" height="{height}"
            viewBox="0 0 {width} {height}"
            xmlns="http://www.w3.org/2000/svg">
        "#,
            width = self.width,
            height = self.height
        )?;

        for element in &self.elements {
            match element {
                SvgElement::Circle {
                    color,
                    position: Vector { x, y },
                } => write!(
                    file,
                    r#"
                    <circle cx="{x}" cy="{y}" fill="{color}" r="5"/>
                "#,
                    color = xml::escape::escape_str_attribute(&color.0)
                )?,
                SvgElement::Curve { color, points } => {
                    write!(
                        file,
                        r#"
                        <path stroke="{color}" stroke-width="2" fill="transparent"  d="
                    "#,
                        color = xml::escape::escape_str_attribute(&color.0)
                    )?;

                    if let Some((first, rest)) = points.split_first() {
                        write!(file, "M {} {} ", first.x, first.y)?;

                        let mut previous = first;

                        for point in rest {
                            write!(
                                file,
                                "C {} {} {} {} {} {} ",
                                previous.x,
                                (previous.y * 2 + point.y) / 3,
                                point.x,
                                (previous.y + point.y * 2) / 3,
                                point.x,
                                point.y
                            )?;

                            previous = point;
                        }
                    }

                    write!(file, r#""/>"#)?;
                }
                SvgElement::Line {
                    color,
                    points,
                    style,
                } => {
                    write!(
                        file,
                        r#"
                        <path stroke="{color}" stroke-width="2" fill="transparent" stroke-dasharray="{dash_array}" d="
                    "#,
                        color = xml::escape::escape_str_attribute(&color.0),
                        dash_array = style
                            .dash_array
                            .iter()
                            .map(|k| format!("{k} "))
                            .collect::<String>()
                    )?;

                    if let Some((first, rest)) = points.split_first() {
                        write!(file, "M {} {} ", first.x, first.y)?;

                        let mut previous = first;

                        for point in rest {
                            write!(file, "L {} {} ", point.x, point.y)?;

                            previous = point;
                        }
                    }

                    write!(file, r#""/>"#)?;
                }
                SvgElement::Text {
                    color,
                    position: Vector { x, y },
                    content,
                    style,
                } => write!(
                    file,
                    r#"
                        <text x="{x}"
                              y="{y}"
                              fill="{color}"
                              dominant-baseline="{dominant_baseline}"
                              text-anchor="{text_anchor}"
                        >{content}</text>
                    "#,
                    color = xml::escape::escape_str_attribute(&color.0),
                    content = xml::escape::escape_str_pcdata(&content),
                    dominant_baseline = style.vertical_align,
                    text_anchor = style.align
                )?,
            }
        }

        write!(file, "</svg>")?;

        return Ok(());
    }
}
