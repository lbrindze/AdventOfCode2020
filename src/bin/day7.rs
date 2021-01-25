use core::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Bag {
    name: String,
    parents: HashSet<String>,
    children: HashSet<String>,
}

impl FromStr for Bag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, bag) = s.split(' ').fold((vec![], Bag{..Default::default()}), |acc, w| {
            let (mut buffer, mut bag) = acc;
            match w {
                "contain" => {
                    bag.name = buffer.join(" ");
                    buffer = vec![];
                }
                "bag," | "bags," | "bag." | "bags." => {
                    bag.children.insert(buffer.join(" "));
                    buffer = vec![];
                }
                "bag" | "bags" => {}
                word if word.parse::<u8>().is_ok() => {}
                word => buffer.push(word),
            }

            (buffer, bag)
        });

        Ok(bag)
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Default)]
struct BagIndex {
    idx: HashMap::<String, Bag>,
}

impl BagIndex {
    fn new() -> Self {
        BagIndex{..Default::default()}
    }

    fn update_parents(&mut self, bag: &Bag) {
        for bag_name in bag.children.iter() {
            match self.idx.get(bag_name) {
                Some(old_bag) => {
                    let mut new_bag = old_bag.clone();
                    new_bag.parents.insert(bag.name.clone());
                    self.idx.insert(bag_name.to_string(), new_bag);
                },
                None => {
                    self.idx.insert(bag_name.to_string(), Bag{
                        name: bag_name.to_string(),
                        parents: [bag.name.clone()].iter().cloned().collect(), 
                        ..Default::default()
                    });
                }
            };
        }
    }

    fn insert_bag(&mut self, bag: &mut Bag) {
        self.update_parents(bag);

        match self.idx.get(&bag.name) {
            Some(existing_bag) => {
                bag.parents = existing_bag.parents.union(&bag.parents).cloned().collect();
                bag.children = existing_bag.children.union(&bag.children).cloned().collect();
            }
            None => {}
        };
        self.idx.insert(bag.name.clone(), bag.clone());
    }
}

fn collect_parents<'a>(bag: &'a Bag, bags: &'a BagIndex) -> HashSet<&'a Bag>{
    let mut count = HashSet::new();
    let parents = &bag.parents;
    if parents.len() == 0 {
        return count;
    } 

    count.insert(bag);

    for parent_name in parents {
        let parent = bags.idx.get(parent_name).unwrap();
        count.insert(&parent);
        count.extend(&collect_parents(parent, bags));
    }

    count
}

fn count_parents(bag: &Bag, bags: &BagIndex) -> usize {
    collect_parents(bag, bags).len() - 1 // also counts itself...
}

fn main() -> std::io::Result<()> {
    let filename = aoc_2020::parse_simple_args();
    let mut bags = BagIndex::new();

    if let Ok(lines) = aoc_2020::read_lines(filename) {
        for line in lines {
            let mut bag: Bag = line.unwrap().parse().unwrap();
            bags.insert_bag(&mut bag);
        }
    }

    let shiny_gold = bags.idx.get("shiny gold").unwrap();

    println!("Shiny Gold: {:?}", shiny_gold);
    println!("Count all parents: {}", count_parents(shiny_gold, &bags));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_str = "shiny tan bags contain 1 shiny gold bag.";
        let expected_bag = Bag {
            name: "shiny tan".to_string(),
            children: ["shiny gold".to_string()].iter().cloned().collect(),
            ..Default::default()
        };

        let bag: Bag = test_str.parse().unwrap();
        assert_eq!(expected_bag, bag);
    }

    #[test]
    fn test_insert_bag() {
        let inputs = vec![
            Bag{
                name: "1".to_string(),
                children: ["2".to_string()].iter().cloned().collect(),
                ..Default::default()
            },
            Bag{
                name: "2".to_string(),
                children: ["4".to_string(), "3".to_string()].iter().cloned().collect(),
                ..Default::default()
            },
            Bag{
                name: "3".to_string(),
                children: ["4".to_string()].iter().cloned().collect(),
                ..Default::default()
            },
        ];

        let mut bags = BagIndex::new();

        for mut bag in inputs {
            bags.insert_bag(&mut bag);
        }

        for name in 1..4 {
            let expected = match name {
                1 => Bag{
                    name: "1".to_string(),
                    children: ["2".to_string()].iter().cloned().collect(),
                    ..Default::default()
                },
                2 => Bag{
                    name: "2".to_string(),
                    parents: ["1".to_string()].iter().cloned().collect(),
                    children: ["3".to_string(), "4".to_string()].iter().cloned().collect(),
                },
                3 => Bag{
                    name: "3".to_string(),
                    parents: ["2".to_string()].iter().cloned().collect(),
                    children: ["4".to_string()].iter().cloned().collect(),
                },
                4 => Bag{
                    name: "4".to_string(),
                    parents: ["3".to_string(), "4".to_string()].iter().cloned().collect(),
                    ..Default::default()
                },
                _ =>Bag{..Default::default()}
            };
            assert_eq!(Some(&expected), bags.idx.get(&expected.name));
        }

        assert_eq!(count_parents(bags.idx.get("4").unwrap(), &bags), 3)
    }
}
