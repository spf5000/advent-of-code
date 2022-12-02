use std::ops::RangeInclusive;
use std::collections::HashSet;

pub struct Day17 { }

impl Default for Day17 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day17 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>
}

const TARGET: Target = Target {
    // Test Input
    // x: 20..=30,
    // y: -10..=-5

    // My Input
    x: 137..=171,
    y: -98..=-73
};

fn main() -> anyhow::Result<()> {
    let y_candidates = find_valid_y_velocities();
    let y_max = y_candidates.iter().fold(0, |acc, (y_vel, _)| {
        let y_max = calculate_y_vel_max(*y_vel);
        if acc > y_max { acc } else { y_max }
    });

    let valid_velocities: HashSet<(i32, i32)> = y_candidates.iter().map(|(y_vel, times)| {
        times.into_iter().map(|time| {
            find_valid_x_velocity_for_t(*time).into_iter().map(|x_vel| (x_vel, *y_vel))
        }).flatten()
    }).flatten().collect();

    println!("The Answer to problem 1 is {:?}", y_max);
    println!("The Answer to problem 2 is {:?}", valid_velocities.len());

    Ok(())
}

fn find_valid_y_velocities() -> Vec<(i32, Vec<u32>)> {
    let mut valid = Vec::new();
    // There is probably a way to determine when to stop trying. Going to just go with 1000 and
    // hope that works :)
    for i in *TARGET.y.start()..1000 {
        let mut dist = 0;
        let mut vel = i;
        let mut steps = 0;
        let mut steps_vec = Vec::new();
        while dist > *TARGET.y.start() {
            dist += vel;
            vel -= 1;
            steps += 1;
            if TARGET.y.contains(&dist) {
                steps_vec.push(steps);
            }
        }
        if steps_vec.len() > 0 {
            valid.push((i, steps_vec));
        }
    }

    valid
}

fn find_valid_x_velocity_for_t(time: u32) -> Vec<i32> {
    let mut output = Vec::new();
    for i in 1..=*TARGET.x.end() {
        let mut dist = 0;
        let mut vel = i;
        for _ in 0..time {
            dist += vel;
            if vel > 0 { vel -= 1 }
        }

        if TARGET.x.contains(&dist) {
            output.push(i);
        }
    }

    output
}


fn calculate_y_vel_max(mut y_vel: i32) -> i32 {
    let mut height = 0;
    loop {
        let new_height = height + y_vel;
        y_vel -= 1;
        if new_height > height {
            height = new_height;
        } else {
            break;
        }
    }
    height
}
