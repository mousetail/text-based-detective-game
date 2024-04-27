use std::fs::OpenOptions;
use std::io::Write;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Color(pub &'static str);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector{
    pub x: usize,
    pub y: usize
}

pub enum SvgElement {
    Circle{color: Color, position: Vector},
    Curve{color: Color, points: Vec<Vector>},
    Line{color: Color, points: Vec<Vector>},
    Text{color: Color, position: Vector, content: String}
}

pub struct Svg {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<SvgElement>
}

impl Svg {
    pub fn write(&self, filename: String) -> std::io::Result<()> {
        println!("About to create file");
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("out.svg")?;
        println!("File created");
        write!(file, r#"
        <svg version="1.1"
            width="{width}" height="{height}"
            viewBox="0 0 {width} {height}"
            xmlns="http://www.w3.org/2000/svg">
        "#, width=self.width, height=self.height)?;

        for element in &self.elements {
            match element {
                SvgElement::Circle { color, position: Vector{ x, y} } => write!(file, r#"
                    <circle cx="{x}" cy="{y}" fill="{color}" r="5"/>
                "#, color=xml::escape::escape_str_attribute(&color.0))?,
                SvgElement::Curve { color, points } => {
                    write!(file, r#"
                        <path stroke="{color}" stroke-width="2" fill="transparent"  d="
                    "#, color=xml::escape::escape_str_attribute(&color.0))?;

                    if let Some((first, rest)) = points.split_first(){
                        write!(
                            file, "M {} {} ", first.x, first.y
                        )?;

                        let mut previous = first;

                        for point in rest {
                            write!(
                                file, "C {} {} {} {} {} {} ",
                                previous.x, (previous.y * 2 + point.y) / 3,
                                point.x, (previous.y + point.y * 2) / 3,
                                point.x, point.y
                            )?; 

                            previous = point;
                        }
                    }

                    write!(
                        file, r#""/>"#
                    )?;
                },
                SvgElement::Line { color, points } => {
                    write!(file, r#"
                        <path stroke="{color}" stroke-width="2" fill="transparent" d="
                    "#, color=xml::escape::escape_str_attribute(&color.0))?;

                    if let Some((first, rest)) = points.split_first(){
                        write!(
                            file, "M {} {} ", first.x, first.y
                        )?;

                        let mut previous = first;

                        for point in rest {
                            write!(
                                file, "L {} {} ",
                                point.x, point.y
                            )?; 

                            previous = point;
                        }
                    }

                    write!(
                        file, r#""/>"#
                    )?;
                },
                SvgElement::Text { color, position: Vector{ x, y}, content } => write!(file,
                    r#"
                        <text x="{x}" y="{y}" fill="{color}">{content}</text>
                    "#,
                    color=xml::escape::escape_str_attribute(&color.0),
                    content=xml::escape::escape_str_pcdata(&content)
                )?,
            }
        }

        write!(
            file, "</svg>"
        )?;

        return Ok(());
    }
}