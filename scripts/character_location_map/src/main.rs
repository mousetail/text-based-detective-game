use std::{collections::HashMap, fs::OpenOptions};

use svg::{Color, LineStyle, SvgElement, TextStyle};
use styles::*;

use crate::svg::Vector;

mod svg;
mod styles;

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
    to: Location<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Time<'a> {
    time: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Event<'a> {
    Action(Action<'a>),
    Movement(Movement<'a>),
    time(Time<'a>),
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
    let mut people_per_location: Vec<std::collections::HashMap<Location, Vec<Character>>> =
        (0..scene.events.len()).map(|_| HashMap::new()).collect();

    let mut previous_people_per_location: std::collections::HashMap<Location, Vec<Character>> =
        HashMap::new();
    for (people, event) in people_per_location.iter_mut().zip(scene.events.iter()) {
        match event {
            Event::Movement(Movement { characters, to }) => {
                for character in characters {
                    previous_people_per_location
                        .values_mut()
                        .for_each(|value| value.retain(|c2| c2 != character));
                }

                println!("Inserting {characters:?} into {to:?}");
                previous_people_per_location
                    .entry(to.clone())
                    .or_insert(vec![])
                    .extend(characters.iter().cloned());
            }
            _ => (),
        };

        *people = previous_people_per_location.clone();
    }

    println!("{people_per_location:#?}");

    let mut location_widths = std::collections::HashMap::<Location, usize>::new();
    for location in &scene.locations {
        location_widths.insert(
            location.clone(),
            people_per_location
                .iter()
                .map(|k| k.get(&location).map_or(0, |t| t.len()))
                .max()
                .unwrap_or(0)
                .max(MIN_COLUMN_WIDTH),
        );
    }

    println!("{location_widths:#?}");

    let mut character_positions_by_time: Vec<HashMap<Character, usize>> = vec![];
    for people_per_location in people_per_location {
        let mut out = std::collections::HashMap::new();
        let mut x = 0;
        let mut location_start_x = 0;
        for location in &scene.locations {
            if let Some(people) = people_per_location.get(location) {
                for person in people {
                    out.insert(person.clone(), x);
                    x += 1;
                }
            }
            x = location_widths.get(location).unwrap_or(&0) + location_start_x;
            location_start_x = x;
        }
        character_positions_by_time.push(out);
    }

    println!("{character_positions_by_time:#?}");

    let mut shapes: Vec<SvgElement> = vec![];

    let mut x = 0;

    *y += VERTICAL_SPACING;
    for location in scene.locations {
        shapes.push(SvgElement::Text {
            color: LOCATION_TITLE_TEXT_COLOR,
            position: svg::Vector {
                x: LEFT_BAR_WIDTH + x * HORIZONTAL_SPACING + (location_widths.get(&location).unwrap_or(&1) - 1) * HORIZONTAL_SPACING / 2,
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

                if (last_curve.len() == 0) {
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
        match event {
            Event::Movement(_) => (),
            Event::Action(Action { characters, name }) => {
                shapes.push(SvgElement::Text {
                    color: EVENT_TEXT_COLOR,
                    position: Vector {
                        x: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH + HORIZONTAL_SPACING / 2,
                        y: *y + index * VERTICAL_SPACING,
                    },
                    content: name.to_string(),
                    style: EVENT_NAME_TEXT_STYLE,
                });

                let mut min_x = usize::MAX;
                for character in characters {
                    let character_x = *character_positions_by_time[index].get(character).unwrap();
                    shapes.push(SvgElement::Circle {
                        color: EVENT_LINE_COLOR,
                        position: Vector {
                            x: character_x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH,
                            y: *y + index * VERTICAL_SPACING,
                        },
                    });
                    min_x = min_x.min(character_x);
                }

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
            Event::time(_) => todo!(),
        }
    }

    *y += scene.events.len() * VERTICAL_SPACING;

    return shapes;
}

fn main() {
    let rufus_red = Character { name: "Rufus Red" };
    let dianna_robinson = Character {
        name: "Dianna Robinson",
    };
    let judy_woolridge = Character {
        name: "Judy Woolridge",
    };
    let duncan_moss = Character {
        name: "Duncan Moss",
    };
    let rebecca_red = Character {
        name: "Rebecca Red",
    };

    let dining_room = Location {
        name: "Dining Room",
    };
    let garage = Location { name: "Garage" };

    generate_svg(vec![Scene {
        locations: vec![dining_room, garage],
        characters: vec![rufus_red, dianna_robinson],
        events: vec![
            Event::Movement(Movement {
                characters: vec![rufus_red],
                to: dining_room.clone(),
            }),
            Event::Action(Action {
                characters: vec![rufus_red],
                name: "Rufus finds the note",
            }),
            Event::Movement(Movement {
                characters: vec![rufus_red, dianna_robinson],
                to: garage.clone(),
            }),
            Event::Action(Action {
                characters: vec![rufus_red, dianna_robinson],
                name: "Dianna and Rufus Argue",
            }),
            Event::Movement(Movement {
                characters: vec![rufus_red, dianna_robinson],
                to: dining_room,
            }),
        ],
    }]);
}
