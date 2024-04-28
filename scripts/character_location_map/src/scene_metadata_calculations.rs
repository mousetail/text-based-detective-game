use std::collections::{HashMap};

use crate::{Character, Location, Movement, Scene, MIN_COLUMN_WIDTH};

pub fn get_people_per_location<'a>(
    scene: &Scene<'a>,
) -> Vec<HashMap<Location<'a>, Vec<Character<'a>>>> {
    let mut people_per_location: Vec<HashMap<Location, Vec<Character>>> =
        (0..scene.events.len()).map(|_| HashMap::new()).collect();

    let mut previous_people_per_location: HashMap<Location, Vec<Character>> = HashMap::new();
    for (people, event) in people_per_location.iter_mut().zip(scene.events.iter()) {
        for Movement { characters, to } in &event.movement {
            for character in characters {
                previous_people_per_location
                    .values_mut()
                    .for_each(|value| value.retain(|c2| c2 != character));
            }

            if let Some(destination) = to {
                previous_people_per_location
                    .entry(*destination)
                    .or_insert(vec![])
                    .extend(characters.iter().cloned());
            }
        }

        *people = previous_people_per_location.clone();
    }

    return people_per_location;
}

pub fn get_location_widths<'a>(
    scene: &Scene<'a>,
    people_per_location: &Vec<HashMap<Location, Vec<Character>>>,
) -> HashMap<Location<'a>, usize> {
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

    return location_widths;
}

pub fn get_character_positions_by_time<'a>(
    scene: &Scene<'a>,
    people_per_location: &Vec<HashMap<Location, Vec<Character<'a>>>>,
    location_widths: &HashMap<Location<'a>, usize>,
) -> Vec<HashMap<Character<'a>, usize>> {
    let mut character_positions_by_time: Vec<HashMap<Character, usize>> = vec![];

    let mut previous_character_positions_by_time: HashMap<Character, usize> = HashMap::new();
    for people_per_location in people_per_location {
        let mut out = std::collections::HashMap::new();
        let mut location_start_x = 0;
        for location in &scene.locations {
            let location_width = *location_widths.get(location).unwrap_or(&0);

            if let Some(people) = people_per_location.get(location) {
                // Try to keep characters in the same place as much as possible
                let people_with_positions: Vec<_> = people
                    .iter()
                    .map(|person| {
                        (
                            person,
                            previous_character_positions_by_time
                                .get(person)
                                .and_then(|&k| {
                                    (k >= location_start_x && k < location_start_x + location_width)
                                        .then_some(k)
                                }),
                        )
                    })
                    .collect();

                for (person, position) in &people_with_positions {
                    if let Some(postion) = position {
                        out.insert((*person).clone(), *postion);
                    }
                }

                for (person, position) in &people_with_positions {
                    if let None = position {
                        out.insert(
                            (*person).clone(),
                            (location_start_x..location_start_x + location_width)
                                .find(|&w| out.values().all(|&v| v != w))
                                .unwrap(),
                        );
                    }
                }
            }
            location_start_x += location_width;
        }
        previous_character_positions_by_time = out.clone();
        character_positions_by_time.push(out);
    }

    return character_positions_by_time;
}

pub fn get_character_introduction_times<'a>(people_per_location: &Vec<HashMap<Location, Vec<Character<'a>>>>) -> (
    Vec<usize>,
    HashMap<Character<'a>, usize>
) {
    let mut event_times: Vec<usize> = vec![];
    let mut character_introduciton_times: HashMap<Character, usize> = HashMap::new();

    let mut y = 0;
    for people_at_time in people_per_location {
        for person in people_at_time.values().cloned().flatten() {
            if !character_introduciton_times.contains_key(&person) {
                character_introduciton_times.insert(person, y);
                y+=1;
            }
        }
        event_times.push(y);
        y+=1;
    }

    return (event_times, character_introduciton_times);
}
