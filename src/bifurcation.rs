const ALPHA_COUNT: usize = 1001;
const X0_COUNT: usize = 1001;
const BUCKET_COUNT: usize = 1000;
const ITERATIONS: usize = 1000;


// Generate a vector of linearly spaced values between start and end, inclusive.
fn linspace(start: f64, end: f64, count: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(count);

    // Base case: if count is 1, just return the start value (and avoid division by zero later).
    if count == 1 {
        result.push(start);
        return result;
    }

    // Calculate the step size between each value.
    let step = (end - start) / (count - 1) as f64;

    // Generate the values and push them into the result vector.
    for i in 0..count {
        result.push(start + i as f64 * step);
    }

    result
}

fn iterate_map(mut x: f64, alpha: f64) -> f64 {
    // e^(-5x²)
    for _ in 0..ITERATIONS {
        x = (-5.0 * x * x).exp() + alpha;
    }

    return x;
}

fn histogram_for_alpha(
    alpha: f64,
    initial_conditions: &[f64],
) -> Vec<usize> {
    let min_x = -2.0;
    let max_x = 2.0;

    let bucket_width =
        (max_x - min_x) / BUCKET_COUNT as f64;

    let mut histogram = vec![0usize; BUCKET_COUNT];

    for &x0 in initial_conditions {
        let final_x = iterate_map(x0, alpha);

        let bucket =
            ((final_x - min_x) / bucket_width) as usize;

        if bucket < BUCKET_COUNT {
            histogram[bucket] += 1;
        }
    }

    return histogram;
}

fn print_histogram(alpha: f64, histogram: &[usize]) {
    println!("Histogram for alpha = {}", alpha);

    let min_x = -2.0;
    let max_x = 2.0;
    let bucket_width =
        (max_x - min_x) / BUCKET_COUNT as f64;

    for (i, &count) in histogram.iter().enumerate() {
        if count > 0 {
            let start = min_x + i as f64 * bucket_width;
            let end = start + bucket_width;

            println!(
                "[{:.4}, {:.4}): {}",
                start, end, count
            );
        }
    }

    println!();
}

pub fn solve() {
    let alphas = linspace(-1.0, 1.0, ALPHA_COUNT);
    let initial_conditions =
        linspace(-2.0, 2.0, X0_COUNT);

    // Store histogram for every alpha.
    let mut all_histograms =
        Vec::with_capacity(ALPHA_COUNT);

    for &alpha in &alphas {
        let histogram =
            histogram_for_alpha(alpha, &initial_conditions);

        all_histograms.push(histogram);
    }

    // Find the exact grid positions for -1, -0.5 and 0.
    let histogram_minus_1 =
        &all_histograms[0];

    let histogram_minus_half =
        &all_histograms[250];

    let histogram_zero =
        &all_histograms[500];

    print_histogram(-1.0, histogram_minus_1);
    print_histogram(-0.5, histogram_minus_half);
    print_histogram(0.0, histogram_zero);
}