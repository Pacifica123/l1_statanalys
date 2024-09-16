
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
