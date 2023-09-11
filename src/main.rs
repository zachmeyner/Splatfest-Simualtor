use rand::prelude::*;
use std::io;

/*
  For the time being only wins and losses matter to this simulation.
  if it can be shown that clout does have a significant impact and isn't just a prettier
  way of dressing up winning and losing I will add clout simulation.
*/

#[derive(PartialEq)]
enum MatchType {
    X1,
    X10,
    X100,
    X333,
}

fn main() {
    println!("Select simulation type");
    println!("1) Basic (basic implementation complete)");
    println!("2) Basic+ (very early stages)");
    println!("3) Tricolor (not implemented)");

    loop {
        let mut select = String::new();
        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        let select = select.trim();

        // println!("{:?}", select);
        break match select {
            "1" => basic_sim(),
            "2" => {
                println!("Not yet implemented");
                continue;
            }
            "3" => {
                println!("Not yet implemented");
                continue;
            }
            _ => {
                println!("Please select simulation type");
                continue;
            }
        };
    }
}

/// Sets up the basic simulation for splatfests, calling store votes to create the votes for all 3 teams.
fn basic_sim() {
    let mut votes: [f32; 3] = [0.0; 3];
    let mut win_count: [u64; 3] = [0; 3];
    let mut mirror_matches: u32 = 0;
    let loop_count: usize;
    println!("Enter population of each team (% to second decimal)");

    store_votes(&mut votes);

    println!("Enter number of matches to simulate.");

    loop {
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit = unit.trim();

        let tmp = unit.parse::<usize>();

        break match tmp {
            Ok(_) => loop_count = tmp.unwrap(),
            Err(_) => {
                println!("Enter a positive integer");
                continue;
            }
        };
    }

    for _i in 0..loop_count {
        turf_sim(&votes, &mut win_count, &mut mirror_matches, MatchType::X1);
    }

    // println!("{:?}", win_count);
    print!("{:8}|", "");
    println!(
        "{:^15}|{:^15}|{:^15}|",
        "Team One", "Team Two", "Team Three"
    );
    println!("{:-<60}", "");
    println!(
        "{:8}|{:^15.2}|{:^15.2}|{:^15.2}|",
        "Votes", votes[0], votes[1], votes[2]
    );
    println!("{:-<60}", "");
    println!(
        "{:8}|{:^15}|{:^15}|{:^15}|",
        "Wins", win_count[0], win_count[1], win_count[2]
    );
}

/// Simluate a single match of turf war.
/// Mirror matches are rerolled 4 times upon being made to simulate the matchmaking system attempting to prioritize non-mirror matches
fn turf_sim(
    votes: &[f32; 3],
    win_count: &mut [u64; 3],
    mirror_matches: &mut u32,
    match_type: MatchType,
) {
    let team_one = pick_teams(&votes);
    let mut team_two = pick_teams(&votes);
    let mut mult: u64 = 1;

    let mut i = 0;
    while i <= 3 && team_two == team_one {
        team_two = pick_teams(&votes);
        i += 1;
    }

    if match_type == MatchType::X10 {
        mult = 10;
    } else if match_type == MatchType::X100 {
        mult = 100;
    } else if match_type == MatchType::X333 {
        mult = 333;
    }

    if team_one == team_two {
        *mirror_matches += 1;
        return;
    }

    if rand::random() {
        win_count[team_one] += mult;
    } else {
        win_count[team_two] += mult;
    }

    // println!("t1: {}, t2: {}", team_one, team_two);
}

/// Randomly picks team to battle
fn pick_teams(votes: &[f32; 3]) -> usize {
    let mut rng = rand::thread_rng();

    let team_rng: f32 = rng.gen::<f32>() * 100.;

    if team_rng <= votes[0] {
        return 0;
    } else if votes[0] < team_rng && team_rng <= votes[0] + votes[1] {
        return 1;
    } else {
        return 2;
    }
}

fn store_votes(votes: &mut [f32; 3]) {
    let mut perc_left: f32 = 100.0;
    let mut cur = 0;

    loop {
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let unit = unit.trim();

        let tmp = unit.parse::<f32>();

        match tmp {
            Ok(_) => {
                if tmp.clone().unwrap() >= 100.0 {
                    println!("Must be less than {perc_left}");
                    continue;
                }

                votes[cur] = tmp.unwrap();
                perc_left -= votes[cur];
                cur += 1;

                if cur == 2 {
                    votes[cur] = perc_left;
                    break;
                }
            }
            Err(_) => {
                println!("Enter a float");
                continue;
            }
        }
    }
}
