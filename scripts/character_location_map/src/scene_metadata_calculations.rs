use std::collections::HashMap;

use crate::{Character, Location, Movement, Scene, MIN_COLUMN_WIDTH};



pub fn get_people_per_location<'a>(
    scene: &Scene<'a>,
) -> Vec<HashMap<Location<'a>, Vec<Character<'a>>>> {
    let mut people_per_location: Vec<HashMap<Location, Vec<Character>>> =
        (0..scene.events.len()).map(|_| HashMap::new()).collect();

    let mut previous_people_per_location: HashMap<Location, Vec<Character>> =
        HashMap::new();
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

pub fn get_location_widths<'a>(scene: &Scene<'a>, people_per_location: &Vec<HashMap<Location, Vec<Character>>>) -> HashMap<Location<'a>, usize> {
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

pub fn get_character_positions_by_time<'a>(scene: &Scene<'a>, people_per_location: &Vec<HashMap<Location, Vec<Character<'a>>>>,
location_widths: &HashMap<Location<'a>, usize>) -> Vec<HashMap<Character<'a>, usize>> {
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

    return character_positions_by_time;
}