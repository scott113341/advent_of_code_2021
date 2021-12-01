mod input;

use input::*;

fn main() {
    println!("day: 01");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Count the number of times a depth measurement increases from the previous measurement
fn part_1(readings: Vec<usize>) -> usize {
    let mut previous_reading = None;
    let mut increase_count = 0;

    for reading in readings {
        if let Some(previous_reading) = previous_reading {
            if reading > previous_reading {
                increase_count += 1;
            }
        }

        previous_reading = Some(reading);
    }

    increase_count
}

// Sum three-measurement sliding windows. How many sums are larger than the previous sum?
fn part_2(readings: Vec<usize>) -> usize {
    let mut previous_window_sum = None;
    let mut increase_count = 0;

    for (idx, reading) in readings.iter().enumerate() {
        // Stop when there's no longer a window of 3 readings remaining
        if readings.get(idx + 2).is_none() {
            break;
        }

        let window_sum = reading + readings[idx + 1] + readings[idx + 2];

        if let Some(previous_window_sum) = previous_window_sum {
            if window_sum > previous_window_sum {
                increase_count += 1;
            }
        }

        previous_window_sum = Some(window_sum);
    }

    increase_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input()), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input()), 5);
    }
}
