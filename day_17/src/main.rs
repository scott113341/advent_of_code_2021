mod data;
mod input;

use data::*;
use input::*;

fn main() {
    println!("day: 17");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// For the probe to successfully make it into the trench, the probe must be on some trajectory that
// causes it to be within a target area after any step. The submarine computer has already
// calculated this target area (your puzzle input). If you're going to fire a highly scientific
// probe out of a super cool probe launcher, you might as well do it with style. How high can you
// make the probe go while still reaching the target area?
fn part_1(start_pos: ProbeAndTarget) -> isize {
    let mut global_max_y = 0;

    for x in 0..=*start_pos.target_x.end() {
        for y in *start_pos.target_y.start()..=*start_pos.target_x.end() {
            let mut try_max_y = 0;
            let mut pos = start_pos.clone();
            pos.velocity = XY { x, y };

            while !pos.confirmed_will_never_hit_target() {
                if pos.position.y > try_max_y {
                    try_max_y = pos.position.y;
                }

                if pos.in_target_area() {
                    if try_max_y > global_max_y {
                        global_max_y = try_max_y;
                    }
                    break;
                } else {
                    pos.step();
                }
            }
        }
    }

    global_max_y
}

// Maybe a fancy trick shot isn't the best idea; after all, you only have one probe, so you had
// better not miss. How many distinct initial velocity values cause the probe to be within the
// target area after any step?
fn part_2(start_pos: ProbeAndTarget) -> usize {
    let mut count = 0;

    for x in 0..=*start_pos.target_x.end() {
        for y in *start_pos.target_y.start()..=*start_pos.target_x.end() {
            let mut pos = start_pos.clone();
            pos.velocity = XY { x, y };

            while !pos.confirmed_will_never_hit_target() {
                if pos.in_target_area() {
                    count += 1;
                    break;
                } else {
                    pos.step();
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 45);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 112);
    }
}
