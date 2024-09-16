mod calculate;
mod graphix;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Выборка данных
    let data: Vec<f64> = vec![
        5.905, 2.089, -0.54, -1.76, -0.32, 2.653, 2.591, -0.77, -8.72, -6.09, 2.752, 8.509, 3.658, 
        3.583, -0.91, 0.96, -3.7, 6.593, -2.09, 0.058, 1.013, -9.79, 0.862, 1.67, 4.889, 1.696, 
        6.856, -6.17, 1.606, 4.305, -2.78, -0.23, 1.763, -6.66, -5.1, 12.8, -2.02, -1.87, -5.89, 
        4.256, 5.554, -2.16, 4.859, -0.39, 3.725, -2.08, 5.382, -0.66, 5.4, 5.062, 0.673, -1.57, 
        5.876, 4.409, 3.775, 1.657, -0.52, -7.42, 11.83, 9.073, 7.574, 5.853, 12.65, 1.74, 0.566, 
        6.627, 2.047, 3.326, -6.31, 1.257, 4.689, -1.38, -3.85, 9.836, 6.439, 6.108, 6.025, -0.55, 
        6.035, -3.47, 2.015, -1.23, 5.246, 3.273, 7.755, 4.351, -0.19, 6.378, 4.582, 5.176, 13.0, 
        -2.34, -7.11, 4.14, -2.56, -1.05, 17.58, 3.33, 2.854, 0.502
    ];
    // Объем выборки
    let n = data.len();

    // Размах выборки
    let range = utils::range(&data);
    println!("Размах выборки: {:.3}", range);

    // Интервалы
    let num_intervals = utils::sturges_intervals(n);
    let intervals = utils::create_intervals(&data, num_intervals);
    let delta = range / num_intervals as f64; 
    println!("Начальный шаг интервалов: {:.3}", delta);
    let frequencies = calculate::calculate_frequencies(&data, &intervals);
    let rel_frequencies = calculate::calculate_relative_frequencies(&frequencies, n);
    println!("Изначальные равные интервалы:");
    utils::print_intervals(&intervals, &frequencies, &rel_frequencies);
    let (merged_intervals, merged_frequencies) = utils::merge_intervals(&intervals, &frequencies);
    let new_rel_frequencies = calculate::calculate_relative_frequencies(&merged_frequencies, n);
    println!("Объединенные интервалы:");
    utils::print_intervals(&merged_intervals, &merged_frequencies, &new_rel_frequencies);

    // Визуализация
    // (тут странно)
    // let heights = calculate::calculate_histogram_heights(&merged_intervals, &new_rel_frequencies);
    // let offset_midpoints = calculate::calculate_interval_midpoints(&merged_intervals);
    // graphix::plot_histogram_and_polygon(&heights, &merged_intervals, &offset_midpoints)?;

    let old_hs = calculate::calculate_histogram_heights(&intervals, &rel_frequencies);
    let midpoints = calculate::calculate_interval_midpoints(&intervals);
    graphix::plot_histogram_and_polygon(&old_hs, &intervals, &midpoints)?;

    Ok(())
}
