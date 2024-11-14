fn main() {
    println!("Medians and modes");
    let cases: Vec<Vec<i64>> = vec![
        vec![], // empty
        vec![1], // one element
        vec![1, 2], // even number of elements
        vec![1, 2, 3], // odd number of elements
        vec![5, 4, 3, 3, 3, 2, 1],
        vec!(1, 2, 2, 3, 4, 5, 5, 5, 5, 6, 6, 6, 6),
        vec!(6, 5, 1, 5, 6, 5, 2, 6, 2, 6, 3, 5, 4),
    ];

    for case in cases {
        println!("\nCase {:?}", case);
        let result = compute_stats(&case);
        println!("{:?}", result);
    }
}

#[derive(Debug)]
pub struct Result {
    median: f64,
    modes: Vec<i64>,
}

pub fn compute_stats(v: &Vec<i64>) -> Result {
    let mut r = Result{median: 0.0, modes: vec![],};
    if v.len() == 0 {
        return r;
    }

    // Sort a clone to get median
    let mut w = v.clone();
    w.sort();
    let n = w.len();
    // Because of the zero base of vectors the following gives
    // larger of the two central indexes, or the central index
    let m = n / 2;
    if n % 2 == 0 {
        r.median = (w[m - 1] + w[m]) as f64 / 2.0;
    }
    else {
        r.median = w[m] as f64;
    }

    // Map with count to get mode
    use std::collections::HashMap;
    let mut map: HashMap<i64, usize> = HashMap::new();
    for i in &w {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    println!("map: {:?}", map);

    let mut largest_count: usize = 0;
    // There may be multiple modes, so store them in a vector,
    // initially empty.
    for (key_ref, value_ref) in &map {
        if *value_ref > largest_count {
            // Discard all the current modes
            r.modes.clear();
            largest_count = *value_ref;
            r.modes.push(*key_ref);
            println!("{:?}", r.modes);
        }
        else if *value_ref == largest_count {
            r.modes.push(*key_ref);
        }
    }
    r.modes.sort();

    r
}
