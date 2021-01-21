use std::ops::BitXorAssign;
use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
struct PlaneTicket {
    id: u16,
    row: u16,
    seat: u16,
}

impl BitXorAssign for PlaneTicket {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.id ^= rhs.id;
        self.row ^= rhs.row;
        self.seat ^= rhs.seat;
    }
}

impl FromStr for PlaneTicket {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let acc = s.chars().fold((0u16, 0u16), |a, c| match c {
            'F' => (a.0 << 1, a.1),
            'B' => ((a.0 ^ 1) << 1, a.1),
            'L' => (a.0, a.1 << 1),
            'R' => (a.0, (a.1 ^ 1) << 1),
            _ => (a.0, a.1),
        });

        let (row, seat) = (acc.0 >> 1, acc.1 >> 1);
        Ok(PlaneTicket {
            id: row * 8 + seat,
            row,
            seat,
        })
    }
}

fn main() -> std::io::Result<()> {
    let filename = aoc_2020::parse_simple_args();
    let mut max = 0;
    let mut missing: PlaneTicket = Default::default();

    if let Ok(lines) = aoc_2020::read_lines(filename) {
        for line in lines {
            let ticket: PlaneTicket = line.unwrap().parse().unwrap();

            if max < ticket.id {
                max = ticket.id;
            }

            missing ^= ticket;
        }
    }

    println!("Max ticket id: {}", max);
    println!("Missing ticket: {:?}", missing);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let test_inputs = vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
        let expected = vec![
            PlaneTicket {
                id: 567,
                row: 70,
                seat: 7,
            },
            PlaneTicket {
                id: 119,
                row: 14,
                seat: 7,
            },
            PlaneTicket {
                id: 820,
                row: 102,
                seat: 4,
            },
        ];
        for (input, res) in test_inputs.iter().zip(expected) {
            assert_eq!(input.parse::<PlaneTicket>().unwrap(), res);
        }
    }
}
