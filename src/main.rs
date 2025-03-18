use std::fmt::{self, Display};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = 20)]
    weight: u16,
}
fn main() {
    let args = Args::parse();
    let tournament = Tournament::new(args.weight);
    tournament.simulate_tournament();
}

struct Tournament {
    weight: u16,
}

#[derive(Debug, Clone, Copy)]
enum Region {
    East,
    Midwest,
    South,
    West,
}

impl Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Region::East => "E",
                Region::Midwest => "M",
                Region::South => "S",
                Region::West => "W",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Team {
    region: Region,
    seed: u16,
}

impl Team {
    fn new(region: Region, seed: u16) -> Self {
        Self { region, seed }
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}", self.region, self.seed)
    }
}

impl Tournament {
    fn new(weight: u16) -> Self {
        Self { weight }
    }

    fn simulate_tournament(&self) {
        println!("EAST:");
        let east = self.simulate_region(Region::East);
        println!("MIDWEST:");
        let midwest = self.simulate_region(Region::Midwest);
        println!("SOUTH:");
        let south = self.simulate_region(Region::South);
        println!("WEST:");
        let west = self.simulate_region(Region::West);

        println!("\nFINAL FOUR:");
        // Create lines for the semifinals and final
        let mut lines = vec![String::new(); 4];
        let spacing = 4;

        // First semifinal
        lines[0] = format!("{}{} ─┐", " ".repeat(spacing), east);
        lines[1] = format!("{}{} ─┘", " ".repeat(spacing), midwest);
        let final_1 = self.play_game(east, midwest);
        lines[0] += &format!(" {} {:2} ─┐", "─".repeat(spacing / 2), final_1);

        // Second semifinal
        lines[2] = format!("{}{} ─┐", " ".repeat(spacing), south);
        lines[3] = format!("{}{} ─┘", " ".repeat(spacing), west);
        let final_2 = self.play_game(south, west);
        lines[2] += &format!(" {} {:2} ─┘", "─".repeat(spacing / 2), final_2);

        // Championship
        let champion = self.play_game(final_1, final_2);
        lines[1] += &format!("{:>8} {} {:2}", "", "─".repeat(spacing / 2), champion);

        // Print the Final Four bracket
        for line in &lines {
            println!("{}", line);
        }
        println!("\nNATIONAL CHAMPION: {}", champion);
    }

    fn play_game(&self, team_1: Team, team_2: Team) -> Team {
        let t1_odds = self.weight - team_1.seed;
        let t2_odds = self.weight - team_2.seed;
        let result = rand::random_range(0..t1_odds + t2_odds);
        if result <= t1_odds { team_1 } else { team_2 }
    }

    fn simulate_region(&self, region: Region) -> Team {
        let mut current_round = vec![
            Team::new(region, 1),
            Team::new(region, 16),
            Team::new(region, 8),
            Team::new(region, 9),
            Team::new(region, 5),
            Team::new(region, 12),
            Team::new(region, 4),
            Team::new(region, 13),
            Team::new(region, 6),
            Team::new(region, 11),
            Team::new(region, 3),
            Team::new(region, 14),
            Team::new(region, 7),
            Team::new(region, 10),
            Team::new(region, 2),
            Team::new(region, 15),
        ];
        let mut next_round = vec![];
        let mut lines = vec![String::new(); current_round.len()];
        let mut spacing = 4;

        while current_round.len() > 1 {
            // Print teams and collect winners
            for i in (0..current_round.len()).step_by(2) {
                let team_1 = current_round[i];
                let team_2 = current_round[i + 1];
                let winner = self.play_game(team_1, team_2);

                // Format the current matchup
                lines[i] = format!("{}{:2} ─┐", " ".repeat(spacing), team_1);
                lines[i + 1] = format!("{}{:2} ─┘", " ".repeat(spacing), team_2);

                // Add winner to the right
                let winner_pos = (i / 2) * 2;
                if lines.len() > winner_pos {
                    lines[winner_pos] += &format!(" {} {:2}", "─".repeat(spacing / 2), winner);
                }

                next_round.push(winner);
            }

            // Print all lines
            for line in &lines {
                println!("{}", line);
            }
            println!();

            // Prepare for next round
            current_round = next_round;
            next_round = vec![];
            lines = vec![String::new(); current_round.len()];
            spacing *= 2;
        }

        current_round[0]
    }
}
