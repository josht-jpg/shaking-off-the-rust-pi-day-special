fn pi_estimation(iterations: u32) -> f64 {
    let mut k = 1;
    let mut result = 0.;

    for i in 0..iterations {
        result += if i % 2 == 0 {
            4. / k as f64
        } else {
            -4. / k as f64
        };
        k += 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn estimation_test() {
        assert!((PI - pi_estimation(31400000)).abs() < 0.0000001);
    }
}
