use std::collections::HashSet;

fn check_anyone_yes(lines: aoc_2020::InputLines) -> u16 {
    let mut char_set = HashSet::<char>::new();
    let mut count = 0;

    lines
        .map(|line| match line {
            Ok(line) if line != *"" => {
                line.chars().map(|c| char_set.insert(c)).for_each(drop);
            }
            _ => {
                count += char_set.len();
                char_set = HashSet::<char>::new();
            }
        })
        .for_each(drop);

    count as u16
}

fn check_everyone_yes(lines: aoc_2020::InputLines) -> u16 {
    let mut in_all_lines = None;
    let mut count = 0;

    lines
        .map(|line| match line {
            Ok(line) if line != *"" => {
                let mut char_set = HashSet::<char>::new();
                line.chars().map(|c| char_set.insert(c)).for_each(drop);
                in_all_lines = match &in_all_lines {
                    Some(set) => Some(char_set.intersection(&set).copied().collect()),
                    None => Some(char_set.iter().cloned().collect()),
                };
            }
            _ => {
                count += match &in_all_lines {
                    Some(set) => set.len(),
                    None => 0,
                };

                in_all_lines = None;
            }
        })
        .for_each(drop);

    count as u16
}

fn main() -> std::io::Result<()> {
    let filename = aoc_2020::parse_simple_args();

    if let Ok(lines) = aoc_2020::read_lines(filename.clone()) {
        let some_count = check_anyone_yes(lines);
        println!(
            "Total sum of counts where someone answewred yes: {}",
            some_count
        );
    }

    if let Ok(lines) = aoc_2020::read_lines(filename) {
        let all_count = check_everyone_yes(lines);
        println!(
            "Total sum of counts where everyone answered yes: {}",
            all_count
        );
    };

    Ok(())
}
