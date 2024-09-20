use full_palette::GREY_A100;
use plotters::prelude::*;

use crate::calculate::calculate_normal_distr_y;


fn format_label(v: &f64) -> String {
    format!("{:.2}", v)
}

pub(crate) fn plot_histogram_and_polygon(
    histogram: &[f64],
    intervals: &[(f64, f64)],
    midpoints: &[f64],
    // для теоретического:
    mean: f64,   
    std_dev: f64 
) -> Result<(), Box<dyn std::error::Error>> {
    // Вычисляем минимальные и максимальные значения по интервалам
    let min_x = intervals.iter().map(|&(lower, _)| lower).fold(f64::INFINITY, f64::min);
    let max_x = intervals.iter().map(|&(_, upper)| upper).fold(f64::NEG_INFINITY, f64::max);


    let root = BitMapBackend::new("histogram.png", (1240, 880)).into_drawing_area();
    root.fill(&WHITE)?;


    
    // Создаем вектор для меток по оси X (границы и середины интервалов)
    let mut x_labels: Vec<f64> = intervals.iter()
        .flat_map(|&(lower, upper)| vec![lower, upper])
        .chain(midpoints.iter().cloned())
        .collect();
    
    // Удаляем дубликаты и сортируем
    x_labels.sort_by(|a, b| a.partial_cmp(b).unwrap());
    x_labels.dedup();

    // Создаем вектор для меток по оси Y (высоты из histogram)
    let y_labels: Vec<f64> = histogram.to_vec();

    let mut chart = ChartBuilder::on(&root)
        .caption("", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(min_x..max_x, 0.0..*y_labels.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;


    // Plot histogram
    for (i, &height) in histogram.iter().enumerate() {
        let (lower, upper) = intervals[i];
        chart.draw_series(vec![
            Rectangle::new([(lower, 0.0), (upper, height)], GREY_A100.filled())
        ])?;
    }

    // Настройка сетки с отметками
    chart.configure_mesh()
        .x_labels(0) // Скрываем все метки по оси X
        .y_labels(0) // Скрываем все метки по оси Y
        .draw()?;

    // Добавляем только нужные метки по оси X
    for &label in &x_labels {
        chart.draw_series(std::iter::once(
            Text::new(format_label(&label), (label, -0.05), ("sans-serif", 15).into_font())
        ))?;
    }

    // Добавляем только нужные метки по оси Y
    for &label in &y_labels {
        chart.draw_series(std::iter::once(
            Text::new(format_label(&label), (min_x - 0.1, label), ("sans-serif", 15).into_font())
        ))?;
    }

    
    // Plot polygon with thicker line
    let polygon_points: Vec<(f64, f64)> = midpoints.iter()
        .zip(histogram.iter())
        .map(|(&x, &height)| (x, height))
        .collect();

    chart.draw_series(LineSeries::new(
        polygon_points.iter().cloned(),
        ShapeStyle {
            color: RED.to_rgba(),
            filled: false,
            stroke_width: 3, // Увеличиваем толщину линии
        },
    ))?;

    // Построение теоретической кривой нормального распределения
    let normal_points: Vec<(f64, f64)> = midpoints.iter()
        .map(|&x| {
            let y = calculate_normal_distr_y(x, mean, std_dev); // Используем функцию для нормального распределения
            (x, y)
        })
        .collect();
    chart.draw_series(LineSeries::new(
        normal_points.iter().cloned(),
        ShapeStyle {
            color: BLACK.to_rgba(),
            filled: false,
            stroke_width: 3,
        },
    ))?;
    chart.draw_series(PointSeries::of_element(
        normal_points.iter().cloned(),
        5, // Размер маркера
        BLACK.to_rgba(),
        &|c, s, st| {
            Circle::new(c, s, ShapeStyle {
                color: BLACK.to_rgba(),
                filled: true,
                stroke_width: 2,
            })
        },
    ))?;

    
    chart.draw_series(PointSeries::of_element(
        polygon_points.iter().cloned(),
        5, // Размер маркера
        RED.to_rgba(),
        &|c, s, st| {
            Circle::new(c, s, ShapeStyle {
                color: RED.to_rgba(),
                filled: true,
                stroke_width: 2,
            })
        },
    ))?;


    root.present()?;
    Ok(())
}