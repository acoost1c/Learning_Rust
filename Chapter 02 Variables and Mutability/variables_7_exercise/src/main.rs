#![allow(unused_variables)]

const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "winter";
    let mut points_scored: i32 = 28;
    points_scored = 35;
    let event_time: &str = "06:00";
    let event_time: i32 = 6;
    println!(
        "My favorite season is {0}. So far the team has scored {1} points. Touchdowns in american rugby gives {TOUCHDOWN_POINTS} points. The game starts at {event_time} o'clock.",
        season, points_scored
    );
    let favorite_beverage: &str = "Vanilla Coke";
}
