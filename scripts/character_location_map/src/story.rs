use crate::{Action, Character, Event, Location, Movement, Scene};

pub fn get_story() -> Vec<Scene<'static>> {
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

    let greenfield = Location { name: "Greenfield" };
    let boxon = Location { name: "Boxon" };

    let rebeccas_appartment = Location {
        name: "Rebecca's Appartment",
    };
    let restaurant = Location { name: "Restaurant" };
    let hotel = Location { name: "Hotel" };
    let other = Location {
        name: "Other",
    };

    return vec![
        Scene {
            locations: vec![boxon, greenfield],
            characters: vec![
                rebecca_red,
                dianna_robinson,
                rufus_red,
                judy_woolridge,
                duncan_moss,
            ],
            events: vec![
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rufus_red, rebecca_red],
                        to: Some(boxon),
                    }],
                    action: None,
                },
                Event {
                    time: Some("2018"),
                    movement: vec![Movement {
                        characters: vec![rebecca_red],
                        to: Some(greenfield),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red],
                        name: "Rebeca goes to the university in Greenfield",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rebecca_red, dianna_robinson],
                        to: Some(greenfield),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red, dianna_robinson],
                        name: "Rebeca and Dianna Meet for the first time",
                    }),
                },
                Event {
                    time: Some("Aug 2023"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red, dianna_robinson],
                        name: "Rebeca and Dianna develop the advanced material science degree",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![judy_woolridge],
                        to: Some(boxon),
                    }],
                    action: Some(Action {
                        characters: vec![rufus_red, judy_woolridge],
                        name: "Judy and Rufus start dating",
                    }),
                },
                Event {
                    time: Some("Oct 2023"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![dianna_robinson],
                        name: "Dianna gets expelled for plagarism",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![
                        Movement {
                            characters: vec![duncan_moss],
                            to: Some(greenfield),
                        },
                        Movement {
                            characters: vec![dianna_robinson],
                            to: None,
                        },
                    ],
                    action: Some(Action {
                        characters: vec![rebecca_red, duncan_moss],
                        name: "Rebecca and Duncan start dating",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![judy_woolridge],
                        to: Some(boxon),
                    }],
                    action: Some(Action {
                        characters: vec![rufus_red, judy_woolridge],
                        name: "Judy and Rufus break us",
                    }),
                },
                Event {
                    time: Some("Nov 2023"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red],
                        name: "Investors contact Rebecca about funding the research",
                    }),
                },
                Event {
                    time: Some("4 Jan 2024"),
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![],
                        name: "Greyham Red Dies",
                    }),
                },
                Event {
                    time: Some("8 Jan 2024"),
                    movement: vec![Movement {
                        characters: vec![rebecca_red],
                        to: Some(boxon),
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red, rufus_red, judy_woolridge],
                        name: "Greyham Red's Funeral",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![rebecca_red],
                        to: Some(greenfield),
                    }],
                    action: None,
                },
                Event {
                    time: Some("Fri, 12 Jan 2024"),
                    movement: vec![Movement {
                        characters: vec![rufus_red, judy_woolridge],
                        to: Some(greenfield),
                    }],
                    action: None,
                },
            ],
        },
        Scene {
            characters: vec![
                rebecca_red,
                dianna_robinson,
                rufus_red,
                judy_woolridge,
                duncan_moss,
            ],
            locations: vec![
                rebeccas_appartment,
                restaurant,
                hotel,
                other,
            ],
            events: vec![
                Event {
                    time: Some("Fri, 12 Jan 2024 15:00"),
                    movement: vec![Movement {
                        characters: vec![
                            rebecca_red,
                            judy_woolridge,
                            duncan_moss,
                            dianna_robinson,
                            rufus_red,
                        ],
                        to: Some(rebeccas_appartment),
                    }],
                    action: None,
                },
                Event {
                    time: Some("Fri, 12 Jan 2024, 22:00"),
                    movement: vec![
                        Movement {
                            characters: vec![rufus_red],
                            to: Some(hotel),
                        },
                        Movement {
                            characters: vec![dianna_robinson],
                            to: Some(other),
                        },
                    ],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![
                            rebecca_red,
                            dianna_robinson,
                            rufus_red,
                            judy_woolridge,
                            duncan_moss,
                        ],
                        name: "Sleeping arangement friday night",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 11:00"),
                    movement: vec![Movement {
                        characters: vec![duncan_moss],
                        to: Some(other)
                    }],
                    action: Some(Action{
                        characters: vec![duncan_moss],
                        name: "Duncan Moss Goes to Work"
                    })
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 18:00"),
                    movement: vec![Movement {
                        characters: vec![rebecca_red, judy_woolridge, dianna_robinson, rufus_red],
                        to: Some(restaurant),
                    }],
                    action: None,
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red, rufus_red],
                        name: "Rufus and Rebecca start arguing",
                    }),
                },

                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![judy_woolridge],
                        name: "Judy joins the argument",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 18:30"),
                    movement: vec![Movement {
                        characters: vec![duncan_moss],
                        to: Some(restaurant),
                    }],
                    action: Some(Action {
                        characters: vec![duncan_moss],
                        name: "Duncan Moss arrives",
                    }),
                },
                Event {
                    time: Some("Sat, 13 Jan 2024, 18:45"),
                    movement: vec![Movement {
                        characters: vec![judy_woolridge, rufus_red],
                        to: Some(hotel),
                    }],
                    action: Some(Action {
                        characters: vec![judy_woolridge, rufus_red],
                        name: "Rufus drags Judy away",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![Movement {
                        characters: vec![dianna_robinson],
                        to: Some(other),
                    }],
                    action: Some(Action {
                        characters: vec![dianna_robinson],
                        name: "Dianna also leaves",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: Some(Action {
                        characters: vec![rebecca_red, duncan_moss],
                        name: "Rebecca and Duncan eat dinner",
                    }),
                },

                Event {
                    time: Some("Sat, 13 Jan 2024, 19:30"),
                    movement: vec![Movement {
                        characters: vec![rebecca_red, duncan_moss],
                        to: Some(rebeccas_appartment)
                    }],
                    action: Some(Action {
                        characters: vec![rebecca_red, duncan_moss],
                        name: "Rebecca and Duncan go home",
                    }),
                },
                Event {
                    time: None,
                    movement: vec![],
                    action: None
                }
            ],
        },
        Scene {
            locations: vec![dining_room, garage],
            characters: vec![rufus_red, dianna_robinson],
            events: vec![
                Event {
                    movement: vec![Movement {
                        characters: vec![rufus_red],
                        to: Some(dining_room),
                    }],
                    action: None,
                    time: Some("14:00"),
                },
                Event {
                    action: Some(Action {
                        characters: vec![rufus_red],
                        name: "Rufus finds the note",
                    }),
                    movement: vec![],
                    time: None,
                },
                Event {
                    movement: vec![Movement {
                        characters: vec![rufus_red, dianna_robinson],
                        to: Some(garage),
                    }],
                    action: None,
                    time: None,
                },
                Event {
                    action: Some(Action {
                        characters: vec![rufus_red, dianna_robinson],
                        name: "Dianna and Rufus Argue",
                    }),
                    movement: vec![],
                    time: None,
                },
                Event {
                    movement: vec![Movement {
                        characters: vec![rufus_red, dianna_robinson],
                        to: Some(dining_room),
                    }],
                    action: None,
                    time: None,
                },
            ],
        },
    ];
}
