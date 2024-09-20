use statrs::distribution::{Normal, ContinuousCDF};

// Функция для вычисления вероятности попадания в интервал
pub(crate)fn calculate_interval_probability(a: f64, b: f64, mean: f64, std_dev: f64) -> f64 {
    // Создаем нормальное распределение с заданными средним и стандартным отклонением
    let normal_dist = Normal::new(mean, std_dev).unwrap();
    
    // Вычисляем F(b) - F(a), чтобы получить вероятность попадания в интервал
    let probability = normal_dist.cdf(b) - normal_dist.cdf(a);
    
    probability
}

pub(crate)fn calculate_frequencies(data: &[f64], intervals: &[(f64, f64)]) -> Vec<usize> {
    let mut frequencies = vec![0; intervals.len()];
    
    for &value in data {
        for (i, &(lower, upper)) in intervals.iter().enumerate() {
            if value > lower && value <= upper {
                frequencies[i] += 1;
                break;
            }
        }
    }
    
    frequencies
}

pub(crate)fn calculate_relative_frequencies(frequencies: &[usize], total: usize) -> Vec<f64> {
    frequencies.iter().map(|&freq| freq as f64 / total as f64).collect()
}


// Связанное с графикой
pub(crate) fn calculate_histogram_heights(intervals: &[(f64, f64)], relative_frequencies: &[f64]) -> Vec<f64> {
    // Проверяем, что количество интервалов и относительных частот совпадает
    if intervals.len() != relative_frequencies.len() {
        panic!("Количество интервалов должно совпадать с количеством относительных частот.");
    }

    // Вычисляем высоты гистограммы
    let histogram_heights: Vec<f64> = intervals.iter()
        .zip(relative_frequencies.iter())
        .map(|(&(lower, upper), &rel_freq)| {
            let delta = upper - lower; // Длина интервала
            rel_freq / delta // Высота гистограммы
        })
        .collect();

    histogram_heights
}


pub(crate)fn calculate_interval_midpoints(intervals: &[(f64, f64)]) -> Vec<f64> {
    intervals.iter().map(|&(lower, upper)| (lower + upper) / 2.0).collect()
}


pub(crate) fn calculate_normal_distr_y(x: f64, mean: f64, std_dev: f64) -> f64 {
    let exponent = -((x - mean).powi(2)) / (2.0 * std_dev.powi(2));
    (1.0 / (std_dev * (2.0 * std::f64::consts::PI).sqrt())) * exponent.exp()
}

// Функция для вычисления критерия Пирсона
pub(crate) fn calculate_hi_square(
    observed_frequencies: &[usize], // кол-во попаданий в i-й интервал
    intervals: &[(f64, f64)],       // границы интервалов
    n: usize,                     // объем выборки
    mean: f64,
    std_dev: f64
) -> f64 {
    // Шаг 1: Вычисление теоретических вероятностей pi для каждого интервала
    let theoretical_probabilities: Vec<f64> = intervals.iter()
        .map(|&(a, b)| calculate_interval_probability(a, b, mean, std_dev)) // Используем новую функцию
        .collect();

    // Шаг 2: Вычисление критерия Пирсона
    let hi_square: f64 = observed_frequencies.iter()
        .zip(theoretical_probabilities.iter())
        .map(|(&observed, &p_i)| {
            let expected = n as f64 * p_i; // n * pi — ожидаемое число попаданий в интервал
            (observed as f64 - expected).powi(2) / expected
        })
        .sum();

    hi_square
}
