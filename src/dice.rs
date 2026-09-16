const N: usize = 100;

fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;

    // This avoids taking square roots 
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }

    return true
}


// position: current board position
// prob: probability of reaching this situation
// q: transient transition matrix
// finish: probability of finishing during this turn
fn process_roll(
    position: usize,
    prob: f64,
    q: &mut [[f64; N]; N],
    finish: &mut [f64; N],
) {
    for roll in 1..=6 {
        let p = prob / 6.0;

        let mut new_position = position + roll;

        // Passing 99 means winning.
        if new_position > 99 {
            finish[position] += p;
            continue;
        }

        // Landing on a prime moves one additional step.
        if is_prime(new_position) {
            new_position += 1;

            // The additional step itself can take us past 99.
            if new_position > 99 {
                finish[position] += p;
                continue;
            }
        }

        // Multiple of 13 means we lose the rest
        // of this turn. The next turn starts at this position.
        if new_position % 13 == 0 {
            q[position][new_position] += p;
            continue;
        }

        // A 6 gives another roll during the same turn.
        if roll == 6 {
            process_roll(new_position, p, q, finish);
        } else {
            q[position][new_position] += p;
        }
    }
}

fn build_transition_matrix() -> ([[f64; N]; N], [f64; N]) {
    let mut q = [[0.0; N]; N];
    let mut finish = [0.0; N];

    for position in 0..N {
        process_roll(position, 1.0, &mut q, &mut finish);
    }

    (q, finish)
}

// I know this is not very readable, but I couldn't find a better way to print a large matrix in a nice format. 
fn print_matrix(q: &[[f64; N]; N]) {
    for row in q {
        for value in row {
            print!("{:.6} ", value);
        }
        println!();
    }
}

fn next_distribution(
    current: &[f64; N],
    q: &[[f64; N]; N],
) -> [f64; N] {
    let mut next = [0.0; N];

    for i in 0..N {
        if current[i] == 0.0 {
            continue;
        }

        for j in 0..N {
            next[j] += current[i] * q[i][j];
        }
    }

    next
}

fn probability_finish_on_turn(
    turn: usize,
    q: &[[f64; N]; N],
    finish: &[f64; N],
) -> f64 {
    let mut distribution = [0.0; N];

    // Beginning of turn 1:
    distribution[0] = 1.0;

    // Advance through turns 1 ... turn-1.
    for _ in 1..turn {
        distribution = next_distribution(&distribution, q);
    }

    // Now distribution describes players who have not finished at the beginning of the requested turn.
    // Calculate probability of finishing during this turn.
    let mut result = 0.0;

    for i in 0..N {
        result += distribution[i] * finish[i];
    }

    result
}

pub fn solve() {
    let (q, finish) = build_transition_matrix();

    print_matrix(&q);

    let probability = probability_finish_on_turn(40, &q, &finish);

    println!(
        "Probability of finishing exactly on turn 40: {:.12}",
        probability
    );
}