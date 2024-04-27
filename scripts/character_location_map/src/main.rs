use std::{collections::HashMap, fs::OpenOptions};

use svg::SvgElement;

use crate::svg::Vector;

mod svg;

const HORIZONTAL_SPACING: usize = 32;
const VERTICAL_SPACING: usize = 64;
const LEFT_BAR_WIDTH: usize = 80;
const MIDDLE_BAR_WIDTH: usize = 320;
const RIGHT_BAR_WIDTH: usize = 320;
const MIN_COLUMN_WIDTH: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Character{name: String}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Location{name: String}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Action{
    characters: Vec<Character>,
    name: String
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Movement{
    characters: Vec<Character>,
    to: Location
}


#[derive(Clone, Debug, Eq, PartialEq)]
struct Time {
    time: String
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Event {
    Action(Action),
    Movement(Movement),
    time(Time)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Scene {
    locations: Vec<Location>,
    characters: Vec<Character>,
    events: Vec<Event>
}

type Story = Vec<Scene>;

fn generate_svg(story: Story) {
    let mut y = 0;

    let mut elements = Vec::<svg::SvgElement>::new();
    for scene in story {
        elements.extend(generate_svg_for_scene(scene, &mut y).into_iter());
    }

    svg::Svg {
        width: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH + RIGHT_BAR_WIDTH,
        height: y,
        elements
    }.write("out.svg".to_string()).unwrap();
}

fn generate_svg_for_scene(scene: Scene, y: &mut usize) -> Vec<svg::SvgElement> {
    // first, find the people in each location
    let mut people_per_location : Vec::<std::collections::HashMap::<Location, Vec<Character>>> = (0..scene.events.len()).map(|_|HashMap::new()).collect();

    let mut previous_people_per_location: std::collections::HashMap::<Location, Vec<Character>> = HashMap::new();
    for (people, event) in people_per_location.iter_mut().zip(scene.events.iter()) {
        match event {
            Event::Movement(Movement{characters, to}) => {
                for character in characters {
                    previous_people_per_location.values_mut().for_each(|value| value.retain(|c2|c2!=character));
                }

                println!("Inserting {characters:?} into {to:?}");
                previous_people_per_location.entry(to.clone()).or_insert(vec![]).extend(characters.iter().cloned());
            },
            _ => ()
        };

        *people = previous_people_per_location.clone();
    }

    println!("{people_per_location:#?}");


    let mut location_widths = std::collections::HashMap::<Location, usize>::new();
    for location in &scene.locations {
        location_widths.insert(
            location.clone(),
             people_per_location.iter().map(
                |k|k.get(&location).map_or(0, |t|t.len())
            ).max().unwrap_or(0).max(MIN_COLUMN_WIDTH)
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
                    x+=1;
                }
            }
            x = location_widths.get(location).unwrap_or(&0) + location_start_x;
            location_start_x = x;
        };
        character_positions_by_time.push(out);
    }

    println!("{character_positions_by_time:#?}");


    let mut shapes: Vec<SvgElement> = vec![];

    let mut x=0;

    *y+=VERTICAL_SPACING;
    for location in scene.locations {
        shapes.push(
            SvgElement::Text { color: svg::Color(
                "black"
            ), position: svg::Vector{x: LEFT_BAR_WIDTH + x * HORIZONTAL_SPACING, y: *y}, content: location.name.clone() }
        );
        x+=location_widths.get(&location).unwrap_or(&0);
    }
    *y+=VERTICAL_SPACING;

    for (person, color) in (&scene.characters).iter().zip(
        ["Red", "Green"]
    ) {
        let mut curves = vec![];
        let mut last_curve = vec![];

        for (index, time) in character_positions_by_time.iter().chain(
            std::iter::once(&HashMap::new())
        ).enumerate() {
            if let Some(x) = time.get(person) {
                last_curve.push(svg::Vector{x: x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH, y: index * VERTICAL_SPACING + *y});
            } else if last_curve.len() > 0 {
                curves.push(std::mem::replace(&mut last_curve, vec![]));
            }
        }

        for curve in curves {
            shapes.push(
                SvgElement::Curve { color: svg::Color(color), points: curve }
            )
        }
    }

    for (index, event) in (&scene.events).iter().enumerate() {
        match event {
            Event::Movement(_) => (),
            Event::Action(Action {
                characters,
                name
            }) => {
                shapes.push(SvgElement::Text { color: svg::Color("Black"), position: Vector {
                    x: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH,
                    y: *y + index * VERTICAL_SPACING,
                }, content: name.to_string() });

                let mut min_x = usize::MAX;
                for character in characters {
                    let character_x = *character_positions_by_time[index].get(character).unwrap();
                    shapes.push(
                        SvgElement::Circle { color: svg::Color("Black"), position: Vector {
                            x: character_x * HORIZONTAL_SPACING + LEFT_BAR_WIDTH,
                            y: *y + index * VERTICAL_SPACING
                        } }
                    );
                    min_x = min_x.min(character_x);
                }

                shapes.push(
                    SvgElement::Line {
                        color: svg::Color("Black"),
                        points: vec![
                            Vector {
                                x: LEFT_BAR_WIDTH + min_x * HORIZONTAL_SPACING,
                                y: *y + index * VERTICAL_SPACING
                            }, 
                            Vector {
                                x: LEFT_BAR_WIDTH + MIDDLE_BAR_WIDTH,
                                y: *y + index * VERTICAL_SPACING
                            }, 
                        ]
                    }
                );
            },
            Event::time(_) => todo!(),
        }
    }

    *y+=scene.events.len() * VERTICAL_SPACING;

    return shapes;
}

fn main() {
    let rufus_red = Character{name:"Rufus Red".to_string()};   
    let dianna_robinson = Character{name:"Dianna Robinson".to_string()};
    let judy_woolridge = Character{name:"Judy Woolridge".to_string()};
    let duncan_moss = Character{name: "Duncan Moss".to_string()};
    let rebecca_red = Character{name: "Rebecca Red".to_string()};

    let dining_room = Location{name: "Dining Room".to_string()};
    let garage = Location{name: "Garage".to_string()};

    generate_svg(vec![
        Scene{
            locations: vec![
                dining_room.clone(),
                garage.clone()
            ],
            characters: vec![
                rufus_red.clone(),
                dianna_robinson.clone()
            ],
            events: vec![

            Event::Movement(Movement{
                characters: vec![rufus_red.clone()],
                to: dining_room.clone()
            }),
            Event::Action(Action{
                characters: vec![rufus_red.clone()],
                name: "Rufus finds the note".to_string()
            }),
            Event::Movement(Movement{
                characters: vec![rufus_red.clone(), dianna_robinson.clone()],
                to: garage.clone()
            }),
            Event::Action(Action{
                characters: vec![rufus_red.clone(), dianna_robinson.clone()],
                name: "Dianna and Rufus Argue".to_string()
            }),
            Event::Movement(Movement{
                characters: vec![rufus_red.clone(), dianna_robinson.clone()],
                to: dining_room.clone()
            })
            ]
        },
    ]);
}