use std::ops::RangeInclusive;
use std::str::FromStr;
use anyhow::anyhow;

use advent_of_code::parse_data_file;

struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>
}

const TARGET: Target = Target {
    x: 137..=171,
    y: -73..=98
};

fn main() -> anyhow::Result<()> {
    let x_steps = calculate_x_steps();
    println!("Potential Steps: {:?}", &x_steps);
    let y_max = calculate_y_max(x_steps);

    println!("The Answer is {:?}", y_max);

    Ok(())
}

fn calculate_y_max(step_options: RangeInclusive<u8>) -> i32 {
    
    let mut y_max = 0;

    // NOTE assuming y velocity is positive since we're looking for max height.
    for y_vel in 0..100000 {
        for steps in step_options.clone() {
            let (y_loc, max) = calculate_y_location(y_vel, steps);
            if TARGET.y.contains(&y_loc) && max > y_max { 
                println!("New max with velocity {}, steps {}: {}", y_vel, steps, max);
                y_max = max;
            }
        }
        /*
        if !hit_target {
            break;
        }
        */
    }
    y_max
}

fn calculate_y_location(mut velocity: i32, steps: u8) -> (i32, i32) {
    let mut dist = 0;
    let mut max = 0;
    for i in 0..steps {
        dist += velocity;
        velocity -= 1;
        if dist > max { max = dist }
    }
    (dist, max)
}

fn calculate_x_steps() -> RangeInclusive<u8> {
    let mut dist = 0;
    let mut steps = 0;
    let mut reached_target = false;
    let mut end = 0;
    for i in 0.. {
        dist += i;
        steps += 1;
        if TARGET.x.contains(&dist) {
            println!("Reached Target: {}, steps: {}", dist, steps);
            reached_target = true;
        } else {
            if reached_target == true {
                println!("Overshot Target: {}, steps: {}", dist, steps);
                end = steps-1;
                break;
            }
        }
    }
    1..=end
}
