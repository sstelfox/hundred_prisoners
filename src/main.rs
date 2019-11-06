use rand::prelude::*;

// The actual probabilty of the random attempt succeeding is 0.5^100 or 8.0^-31, I'd be surprised
// to ever see a success on that one.
const ATTEMPTS: usize = 100_000;

const PRISONER_COUNT: usize = 100;
const PRISONER_ATTEMPTS: usize = 50;

fn attempt_optimal_naive(drawers: &Vec<usize>) -> bool {
    for prisoner_id in 0..PRISONER_COUNT {
        let mut succeeded = false;
        let mut box_to_check = prisoner_id;

        for _ in 0..PRISONER_ATTEMPTS {
            if drawers[box_to_check] == prisoner_id {
                succeeded = true;
                break;
            }

            box_to_check = drawers[box_to_check];
        }

        // Any failure is a complete failure
        if !succeeded {
            return false;
        }
    }

    true
}

fn attempt_optimal_random_init(drawers: &Vec<usize>) -> bool {
    let mut rng = rand::thread_rng();

    for prisoner_id in 0..PRISONER_COUNT {
        let mut succeeded = false;
        let mut box_to_check = rng.gen_range(0, PRISONER_COUNT);

        for _ in 0..PRISONER_ATTEMPTS {
            if drawers[box_to_check] == prisoner_id {
                succeeded = true;
                break;
            }

            box_to_check = drawers[box_to_check];
        }

        // Any failure is a complete failure
        if !succeeded {
            return false;
        }
    }

    true
}

fn attempt_optimal_tracked(drawers: &Vec<usize>) -> bool {
    let mut rng = rand::thread_rng();

    for prisoner_id in 0..PRISONER_COUNT {
        let mut box_to_check = prisoner_id;
        let mut succeeded = false;
        let mut viewed_boxes: Vec<usize> = Vec::new();

        for _ in 0..PRISONER_ATTEMPTS {
            if drawers[box_to_check] == prisoner_id {
                succeeded = true;
                break;
            } else {
                viewed_boxes.push(box_to_check);
                box_to_check = drawers[box_to_check];

                loop {
                    if viewed_boxes.iter().any(|i| i == &box_to_check) {
                        box_to_check = rng.gen_range(0, PRISONER_COUNT);
                        continue;
                    }

                    break;
                }
            }
        }

        // Any failure is a complete failure
        if !succeeded {
            return false;
        }
    }

    true
}

fn attempt_naive_random(drawers: &Vec<usize>) -> bool {
    let mut rng = rand::thread_rng();

    for prisoner_id in 0..PRISONER_COUNT {
        let mut box_to_check: usize = rng.gen_range(0, PRISONER_COUNT);
        let mut succeeded = false;

        for _ in 0..PRISONER_ATTEMPTS {
            if drawers[box_to_check] == prisoner_id {
                succeeded = true;
                break;
            }

            box_to_check = rng.gen_range(0, PRISONER_COUNT);
        }

        // Any failure is a complete failure
        if !succeeded {
            return false;
        }
    }

    true
}

fn attempt_tracked_random(drawers: &Vec<usize>) -> bool {
    let mut rng = rand::thread_rng();

    for prisoner_id in 0..PRISONER_COUNT {
        let mut box_to_check: usize = rng.gen_range(0, PRISONER_COUNT);
        let mut succeeded = false;
        let mut viewed_boxes: Vec<usize> = Vec::new();

        for _ in 0..PRISONER_ATTEMPTS {
            if drawers[box_to_check] == prisoner_id {
                succeeded = true;
                break;
            }

            viewed_boxes.push(box_to_check);
            box_to_check = rng.gen_range(0, PRISONER_COUNT);

            loop {
                if viewed_boxes.iter().any(|i| i == &box_to_check) {
                    box_to_check = rng.gen_range(0, PRISONER_COUNT);
                    continue;
                }

                break;
            }
        }

        // Any failure is a complete failure
        if !succeeded {
            return false;
        }
    }

    true
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut drawers: Vec<usize> = (0..PRISONER_COUNT).collect();

    let mut optimal_naive_successes = 0;
    let mut optimal_random_init_successes = 0;
    let mut optimal_tracked_successes = 0;
    let mut random_naive_successes = 0;
    let mut random_tracked_successes = 0;

    for _ in 0..ATTEMPTS {
        drawers.shuffle(&mut rng);

        if attempt_optimal_naive(&drawers) {
            optimal_naive_successes += 1;
        }

        if attempt_optimal_random_init(&drawers) {
            optimal_random_init_successes += 1;
        }

        if attempt_optimal_tracked(&drawers) {
            optimal_tracked_successes += 1;
        }

        if attempt_naive_random(&drawers) {
            random_naive_successes += 1;
        }

        if attempt_tracked_random(&drawers) {
            random_tracked_successes += 1;
        }
    }

    println!("The prisoners optimally (naive) succeeded {} out of {} times", optimal_naive_successes, ATTEMPTS);
    println!("The prisoners optimally (random_init) succeeded {} out of {} times", optimal_random_init_successes, ATTEMPTS);
    println!("The prisoners optimally (tracked) succeeded {} out of {} times", optimal_tracked_successes, ATTEMPTS);
    println!("The prisoners randomly (naive) succeeded {} out of {} times", random_naive_successes, ATTEMPTS);
    println!("The prisoners randomly (tracked) succeeded {} out of {} times", random_tracked_successes, ATTEMPTS);
}
