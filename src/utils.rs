

pub(crate) fn range(data: &[f64]) -> f64 {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    println!("Минимальное: {}", min);
    println!("Максимальное: {}", max);
    max - min
}

pub(crate) fn sturges_intervals(n: usize) -> usize {
    (1.0 + 3.32 * (n as f64).log10()).ceil() as usize
}

pub(crate) fn create_intervals(data: &[f64], num_intervals: usize) -> Vec<(f64, f64)> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let step = (max - min) / num_intervals as f64;
    
    let mut intervals = Vec::new();
    for i in 0..num_intervals {
        intervals.push((min + i as f64 * step, min + (i + 1) as f64 * step));
    }
    
    intervals
}

pub(crate) fn empirical_distribution_function(relative_frequencies: &[f64]) -> Vec<f64> {
    let mut accumulated = 0.0;
    let mut edf = Vec::new();
    
    for &rel_freq in relative_frequencies {
        accumulated += rel_freq;
        edf.push(accumulated);
    }
    
    edf
}

pub(crate) fn merge_intervals(intervals: &[(f64, f64)], frequencies: &[usize]) -> (Vec<(f64, f64)>, Vec<usize>) {
    let mut merged_intervals = Vec::new();
    let mut merged_frequencies = Vec::new();

    let mut current_interval = intervals[0];
    let mut current_frequency = frequencies[0];

    for i in 1..intervals.len() {
        let next_interval = intervals[i];
        let next_frequency = frequencies[i];

        // Проверяем, нужно ли объединять интервалы
        if current_frequency < 5 || next_frequency < 5 {
            // Объединяем интервалы
            current_interval = (current_interval.0, next_interval.1);
            current_frequency += next_frequency;
        } else {
            // Добавляем текущий интервал и частоту в результирующие векторы
            merged_intervals.push(current_interval);
            merged_frequencies.push(current_frequency);

            // Переходим к следующему интервалу
            current_interval = next_interval;
            current_frequency = next_frequency;
        }
    }

    // Добавляем последний интервал и частоту
    merged_intervals.push(current_interval);
    merged_frequencies.push(current_frequency);

    (merged_intervals, merged_frequencies)
}


//для дебага
pub(crate) fn print_intervals(intervals: &Vec<(f64, f64)>, frequencies: &Vec<usize>, rel_freq: &Vec<f64>) {
    // Проверяем, что количество интервалов, частот и относительных частот совпадает
    if intervals.len() != frequencies.len() || intervals.len() != rel_freq.len() {
        println!("Ошибка: количество интервалов, частот или относительных частот не совпадает.");
        return;
    }

    // Заголовок таблицы
    println!("{:<20} {:<15} {:<15}", "Интервал", "Частота", "Отн. частота");
    println!("{:-<50}", ""); // Разделительная линия

    // Перебираем интервалы, частоты и относительные частоты
    for (interval, (frequency, rel_frequency)) in intervals.iter().zip(frequencies.iter().zip(rel_freq.iter())) {
        let (start, end) = interval;
        println!("{:<20} {:<15} {:<15.2}", format!("[{:.2}, {:.2}]", start, end), frequency, rel_frequency);
    }
}
