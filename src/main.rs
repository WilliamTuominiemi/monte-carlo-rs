use plotters::prelude::*;
use std::iter::zip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_size = 5000;
    let x = generate_list_of_random_values(sample_size);
    let y = generate_list_of_random_values(sample_size);

    let root = BitMapBackend::new("plotters-doc-data/1.png", (640, 640)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(10, 10, 10, 10);

    let mut chart = ChartBuilder::on(&root)
        .caption("Monte Carlo", ("sans-serif", 40).into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let within_circle: Vec<(f32, f32)> = x
        .iter()
        .zip(y.iter())
        .filter(|&(i, j)| i * i + j * j <= 1.0)
        .map(|(i, j)| (*i, *j))
        .collect();

    let outside_circle: Vec<(f32, f32)> = x
        .iter()
        .zip(y.iter())
        .filter(|&(i, j)| i * i + j * j > 1.0)
        .map(|(i, j)| (*i, *j))
        .collect();

    chart.draw_series(PointSeries::of_element(
        within_circle.clone(),
        3,
        &RED,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    chart.draw_series(PointSeries::of_element(
        outside_circle.clone(),
        3,
        &BLUE,
        &|c, s, st| EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()),
    ))?;

    root.present()?;

    println!(
        "{:?}",
        monte_carlo_pi(sample_size, within_circle.iter().count())
    );

    Ok(())
}

fn generate_random_float() -> f32 {
    rand::random_range(-1.0..=1.0)
}

fn generate_list_of_random_values(size: usize) -> Vec<f32> {
    (0..size).map(|_| generate_random_float()).collect()
}

fn monte_carlo_pi(num_samples: usize, points_inside_circle: usize) -> f32 {
    4.0 * (points_inside_circle as f32) / (num_samples as f32)
}
