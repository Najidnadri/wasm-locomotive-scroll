

const _NEWTON_ITERATIONS: u32 = 4;
const _NEWTON_MIN_SLOPE: f64 = 0.001;
const _SUBDIVISION_PRECISION: f64 = 0.0000001;
const _SUBDIVISION_MAX_ITERATIONS: u32 = 10;

const K_SPLINE_TABLE_SIZE: u32 = 11;
const K_SAMPLE_STEP_SIZE: f64 = 1.0 / (K_SPLINE_TABLE_SIZE as f64 - 1.0);

fn a(a_1: f64, a_2: f64) -> f64 {
    1.0 - 3.0 * a_2 + 3.0 * a_1
}

fn b(a_1: f64, a_2: f64) -> f64 {
    3.0 * a_2 - 6.0 * a_1
}

fn c(a_1: f64) -> f64 {
    3.0 * a_1
}

fn calc_bezier(t: f64, a_1: f64, a_2: f64) -> f64 {
    ((a(a_1, a_2) * t + b(a_1, a_2)) * t + c(a_1)) * t
}

fn _get_slope(t: f64, a_1: f64, a_2: f64) -> f64 {
    3.0 * a(a_1, a_2) * t * t + 2.0 * b(a_1, a_2) * t + c(a_1)
}

fn _binary_subdivide(x: f64, mut a: f64, mut b: f64, m_x1: f64, m_x2: f64) -> f64 {
    let mut current_t;
    let mut current_x;
    let mut i = 0;
    loop {
        current_t = a + (b - a) / 2.0;
        current_x = calc_bezier(current_t, m_x1, m_x2) - x;
        if current_x > 0.0 {
            b = current_t;
        } else {
            a = current_t;
        }
        if (current_x.abs() <= _SUBDIVISION_PRECISION) || (i >= _SUBDIVISION_MAX_ITERATIONS) {
            break;
        }
        i += 1;
    }
    current_t
}


fn _newton_raphson_iterate(x: f64, guess_t: f64, m_x1: f64, m_x2: f64) -> f64 {
    let mut a_guess_t = guess_t;
    for _ in 0.._NEWTON_ITERATIONS {
        let current_slope = _get_slope(a_guess_t, m_x1, m_x2);
        if current_slope == 0.0 {
            return a_guess_t;
        }
        let current_x = calc_bezier(a_guess_t, m_x1, m_x2) - x;
        a_guess_t -= current_x / current_slope;
    }
    a_guess_t
}

fn linear_easing(x: f64) -> f64 {
    x
}
pub fn bezier2(m_x1: f64, m_y1: f64, m_x2: f64, m_y2: f64) -> Box<dyn Fn(f64) -> f64> {
    if !((0.0 <= m_x1 && m_x1 <= 1.0) && (0.0 <= m_x2 && m_x2 <= 1.0)) {
        panic!("bezier x values must be in [0, 1] range");
    }

    if (m_x1 == m_y1) && (m_x2 == m_y2) {
        return Box::new(|x| linear_easing(x));
    }

    let mut sample_values = [0.0; K_SPLINE_TABLE_SIZE as usize];
    for i in 0..K_SPLINE_TABLE_SIZE {
        sample_values[i as usize] = calc_bezier(i as f64 * K_SAMPLE_STEP_SIZE, m_x1, m_x2);
    }

    Box::new(move |x| {
        if x == 0.0 || x == 1.0 {
            return x;
        }

        let mut _interval_start = 0.0;
        let mut current_sample = 1;
        let mut last_sample = K_SPLINE_TABLE_SIZE - 1;

        loop {
            if current_sample == last_sample || x == sample_values[current_sample as usize] {
                return sample_values[current_sample as usize];
            }

            if x > sample_values[current_sample as usize] {
                _interval_start = current_sample as f64;
                current_sample += 1;
            } else {
                last_sample = current_sample;
            }
        }
    })
}

