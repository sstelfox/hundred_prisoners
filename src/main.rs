use rand::prelude::*;

const ATTEMPTS: usize = 100_000;
const PRISONER_COUNT: usize = 100;
const PRISONER_ATTEMPTS: usize = 50;

fn attempt_random(drawers: &Vec<Option<usize>>) -> bool {
    let mut rng = rand::thread_rng();

    for prisoner_id in 0..PRISONER_COUNT {
        let mut succeeded = false;

        for _ in 0..PRISONER_ATTEMPTS {
            let checked_box: usize = rng.gen_range(0, PRISONER_COUNT);

            if drawers[checked_box] == Some(prisoner_id) {
                succeeded = true;
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
    let mut drawers: Vec<Option<usize>> = (0..PRISONER_COUNT).map(|i| Some(i)).collect();
    let mut random_successes = 0;

    for _ in 0..ATTEMPTS {
        drawers.shuffle(&mut rng);

        if attempt_random(&drawers) {
            random_successes += 1;
        }
    }

    println!("The prisoners randomly succeeded {} out of {} times", random_successes, ATTEMPTS);
}
