use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;

struct GrandDesign {
    sacrament: HashMap<String, Ritual>
}

impl GrandDesign {
    fn ritualistic_sacrifices(&self, ritual: &String) -> Option<i32> {
        if let Some(ritual) = self.sacrament.get(ritual) {
            let cultist_sacrifices: i32 = ritual.cultists.iter()
                                                    .map(|c| c.sacrifices)
                                                    .sum();
            let ritual_sacrifices: i32 = ritual.subrituals.iter()
                                                    .map(|sr| self.ritualistic_sacrifices(sr).unwrap())
                                                    .sum();

            Some(cultist_sacrifices + ritual_sacrifices)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ritual {
    cultists: Vec<Cultist>,
    subrituals: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct Cultist {
    sacrifices: i32,
    name: String,
}

impl FromStr for Cultist {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let metadata = s.split(" ").collect::<Vec<_>>();
        let sacrifices = metadata[0].parse::<i32>().unwrap();
        let name = metadata[1].to_string();

        Ok(Cultist { sacrifices: sacrifices, name: name})
    }
}

fn chant_the_words(input: &str) -> GrandDesign {
    let mut chant: Vec<_> = vec![];
    let mut sacrament: HashMap<String, Ritual> = HashMap::new();

    for command in input[2..].split("\n$ ") {        
        if command == "cd .." {
            chant.pop();
        }
        else if let Some(word) = command.strip_prefix("cd ") {
            if word == "/" {
                chant.clear();
                chant.push("");
            } else {
                chant.push(word);
            }
        }
        else if command.starts_with("ls") {
            let lines = command.lines().skip(1);
            let mut subrituals: Vec<_> = vec![];
            let mut cultists: Vec<Cultist> = vec![];

            for line in lines {
                if let Some(subrit_name) = line.strip_prefix("dir ") {
                    subrituals.push(format!("{}/{}", chant.join("/"), subrit_name));
                } else {
                    cultists.push(line.parse::<Cultist>().unwrap());
                }
            }
            sacrament.insert(chant.join("/"), Ritual { subrituals: subrituals, cultists: cultists });
        }
    }
    GrandDesign { sacrament: sacrament }
}

pub fn solve(input: &str) {
    let grand_design = chant_the_words(input);
    let mut part_one = 0;
    let available_sacs = 70_000_000 - grand_design.ritualistic_sacrifices(&String::from("")).unwrap();
    let req_capacity = 30_000_000;
    let mut rits_to_cancel: Vec<i32> = vec![];

    for ritual in grand_design.sacrament.keys() {
        let sacrifices = grand_design.ritualistic_sacrifices(ritual).unwrap();
        if sacrifices <= 100_000 {
            part_one += sacrifices;
        }
        if available_sacs + sacrifices >= req_capacity {
            rits_to_cancel.push(sacrifices);
        }
    }

    let part_two = rits_to_cancel.iter().min().unwrap();
    println!("{} {}", part_one, part_two);
}