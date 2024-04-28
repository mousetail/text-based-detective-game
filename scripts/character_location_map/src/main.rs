use std::collections::HashMap;

use story::get_story;
use styles::*;
use svg::SvgElement;

use crate::{
    scene_metadata_calculations::{
        get_character_positions_by_time, get_location_widths, get_people_per_location,
    },
    svg::Vector,
};

mod scene_metadata_calculations;
mod story;
mod styles;
mod svg;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Character<'a> {
    name: &'a str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Location<'a> {
    name: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Action<'a> {
    characters: Vec<Character<'a>>,
    name: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Movement<'a> {
    characters: Vec<Character<'a>>,
    to: Option<Location<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Time<'a> {
    time: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Event<'a> {
    action: Option<Action<'a>>,
    time: Option<&'a str>,
    movement: Vec<Movement<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scene<'a> {
    locations: Vec<Location<'a>>,
    characters: Vec<Character<'a>>,
    events: Vec<Event<'a>>,
}

type Story<'a> = Vec<Scene<'a>>;

fn generate_svg(story: Story) {
    let mut y = 0;

    let mut elements = Vec::<svg::SvgElement>::new();
    for scene in story {
        elements.extend(generate_svg_for_scene(scene, &mut y).into_iter());
    }

    svg::Svg {
        width: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH + RIGHT_BAR_WIDTH,
        height: y,
        elements,
    }
    .write("out.svg".to_string())
    .unwrap();
}

fn generate_svg_for_scene(scene: Scene, y: &mut usize) -> Vec<svg::SvgElement> {
    // first, find the people in each location

    let people_per_location = get_people_per_location(&scene);
    let location_widths = get_location_widths(&scene, &people_per_location);
    let character_positions_by_time =
        get_character_positions_by_time(&scene, &people_per_location, &location_widths);

    let mut shapes: Vec<SvgElement> = vec![];

    let mut x = 0;

    *y += VERTICAL_SPACING;
    for location in scene.locations {
        shapes.push(SvgElement::Text {
            color: LOCATION_TITLE_TEXT_COLOR,
            position: svg::Vector {
                x: LEFT_BAR_WIDTH
                    + x * HORIZONTAL_SPACING
                    + (location_widths.get(&location).unwrap_or(&1) - 1) * HORIZONTAL_SPACING / 2,
                y: *y,
            },
            content: location.name.to_string(),
            style: LOCATION_HEADER_TEXT_STYLE,
        });
        x += location_widths.get(&location).unwrap_or(&0);

        shapes.push(SvgElement::Line {
            color: LOCATION_SEPERATOR_LINE_COLOR,
            style: LOCATION_SEPERATOR_LINE_STYLE,
            points: vec![
                svg::Vector {
                    x: x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH - HORIZONTAL_SPACING / 2,
                    y: *y + VERTICAL_SPACING / 2,
                },
                svg::Vector {
                    x: x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH - HORIZONTAL_SPACING / 2,
                    y: *y + VERTICAL_SPACING * character_positions_by_time.len(),
                },
            ],
        })
    }
    *y += VERTICAL_SPACING;

    for (person, color) in (&scene.characters).iter().zip(CHARACTER_COLORS) {
        let mut curves = vec![];
        let mut last_curve = vec![];

        for (index, time) in character_positions_by_time
            .iter()
            .chain(std::iter::once(&HashMap::new()))
            .enumerate()
        {
            if let Some(x) = time.get(person) {
                let new_point = svg::Vector {
                    x: x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH,
                    y: index * VERTICAL_SPACING + *y,
                };

                if last_curve.len() == 0 {
                    shapes.push(SvgElement::Circle {
                        color: *color,
                        position: new_point,
                    });
                }
                last_curve.push(new_point);
            } else if last_curve.len() > 0 {
                let last_curve = std::mem::replace(&mut last_curve, vec![]);
                let last_point = last_curve.last().unwrap();
                shapes.push(SvgElement::Circle {
                    color: *color,
                    position: *last_point,
                });
                curves.push(last_curve);
            }
        }

        for curve in curves {
            shapes.push(SvgElement::Curve {
                color: *color,
                points: curve,
            })
        }
    }

    for (index, event) in (&scene.events).iter().enumerate() {
        if let Some(Action { characters, name }) = &event.action {
            shapes.push(SvgElement::Text {
                color: EVENT_TEXT_COLOR,
                position: Vector {
                    x: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH + HORIZONTAL_SPACING / 2,
                    y: *y + index * VERTICAL_SPACING,
                },
                content: name.to_string(),
                style: EVENT_NAME_TEXT_STYLE,
            });

            let mut min_x = None;
            for character in characters {
                let character_x = *character_positions_by_time[index].get(character).unwrap();
                shapes.push(SvgElement::Circle {
                    color: EVENT_LINE_COLOR,
                    position: Vector {
                        x: character_x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH,
                        y: *y + index * VERTICAL_SPACING,
                    },
                });
                min_x = Some(min_x.unwrap_or(character_x).min(character_x));
            }

            if let Some(min_x) = min_x {
                shapes.push(SvgElement::Line {
                    color: EVENT_LINE_COLOR,
                    style: EVENT_LINE_STYLE,
                    points: vec![
                        Vector {
                            x: LEFT_BAR_WIDTH + min_x * HORIZONTAL_SPACING,
                            y: *y + index * VERTICAL_SPACING,
                        },
                        Vector {
                            x: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH,
                            y: *y + index * VERTICAL_SPACING,
                        },
                    ],
                });
            }
        }
    }

    for (index, event) in (&scene.events).iter().enumerate() {
        if let Some(time) = event.time {
            shapes.push(SvgElement::Text {
                color: TIME_TEXT_COLOR,
                position: Vector {
                    x: LEFT_BAR_WIDTH - HORIZONTAL_SPACING / 2,
                    y: *y + index * VERTICAL_SPACING,
                },
                content: time.to_string(),
                style: TIME_TEXT_STYLE,
            })
        }
    }

    *y += scene.events.len() * VERTICAL_SPACING;

    return shapes;
}

fn main() {
    generate_svg(get_story());
}
