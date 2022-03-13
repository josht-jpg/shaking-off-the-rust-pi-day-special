use std::f64::consts::PI;

fn estimate_pi(decimal_places: u32) -> f64 {
    assert!(
        decimal_places < 9,
        "It's not worth freezing your computer over this, my friend."
    );

    let target_accuracy = 1. / 10u32.pow(decimal_places + 1) as f64;
    let is_accurate_enough =
        |current_estimate: f64| (current_estimate - PI).abs() < target_accuracy;

    let mut result = 0.;
    let mut k = 1;
    let mut subtract_fraction = false;

    while !is_accurate_enough(result) {
        result += if subtract_fraction {
            -4. / k as f64
        } else {
            4. / k as f64
        };
        k += 2;
        subtract_fraction = !subtract_fraction;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimation_test() {
        assert!((PI - estimate_pi(1)).abs() < 0.01);
        assert!((PI - estimate_pi(3)).abs() < 0.0001);
        assert!((PI - estimate_pi(5)).abs() < 0.000001);
        assert!((PI - estimate_pi(8)).abs() < 0.000000001)
    }
}
