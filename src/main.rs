use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

#[allow(dead_code)]
fn cin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim_end().to_string()
}

#[allow(dead_code)]
fn cin_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    cin()
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn cin_vertical<T>(n: usize) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut res = Vec::new();
    for _ in 0..n {
        res.push(cin().parse().unwrap());
    }
    res
}

trait ToTuple<T> {
    fn to_2(self) -> (T, T);
    fn to_3(self) -> (T, T, T);
}

impl<T> ToTuple<T> for Vec<T>
where
    T: Clone,
{
    fn to_2(self) -> (T, T) {
        (self.get(0).unwrap().clone(), self.get(1).unwrap().clone())
    }

    fn to_3(self) -> (T, T, T) {
        (
            self.get(0).unwrap().clone(),
            self.get(1).unwrap().clone(),
            self.get(2).unwrap().clone(),
        )
    }
}

#[derive(Debug)]
struct Schedule {
    t: usize,
    position: Position,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn distance_from(&self, other: &Position) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = cin().parse().unwrap();
    let schedules: Vec<Schedule> = {
        let mut schedules = vec![Schedule {
            t: 0,
            position: Position { x: 0, y: 0 },
        }];
        schedules.extend::<Vec<Schedule>>(
            (0..n)
                .map(|_| {
                    let txy = cin_vec::<usize>();
                    Schedule {
                        t: txy[0],
                        position: Position {
                            x: txy[1] as i64,
                            y: txy[2] as i64,
                        },
                    }
                })
                .collect(),
        );
        schedules
    };

    let is_possible = {
        let mut is_possible = true;
        for i in 0..n {
            let schedule = &schedules[i];
            let next_schedule = &schedules[i + 1];

            if schedule.position.distance_from(&next_schedule.position)
                > next_schedule.t - schedule.t
                || schedule.position.distance_from(&next_schedule.position) % 2
                    != (next_schedule.t - schedule.t) % 2
            {
                is_possible = false;
                break;
            }
        }
        is_possible
    };

    println!("{}", if is_possible { "Yes" } else { "No" });

    Ok(())
}
