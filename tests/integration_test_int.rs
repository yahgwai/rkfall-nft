#[cfg(test)]
mod tests {
    const PRECISION_FLOAT: f64 = PRECISION as f64;

    use plotters::{prelude::*, style::full_palette::ORANGE};
    use rk_fall::int_rk4::{tick, MotionState, PRECISION};

    pub fn get_orbit_data(
        initial_states: Vec<MotionState>,
        time_periods: u32,
    ) -> Vec<Vec<(f64, f64)>> {
        let time_period_sec = (0.001 * PRECISION_FLOAT) as i64;
        let mut data: Vec<Vec<(f64, f64)>> = Vec::new();

        let mut states = initial_states.to_vec();
        for s in &states {
            data.push(vec![(s.get_x() as f64, s.get_y() as f64)]);
        }

        println!("{:?} {:?}", time_periods, time_period_sec);

        println!(
            "masses: {:?}",
            initial_states
                .iter()
                .map(|i| i.get_mass())
                .collect::<Vec<_>>()
        );
        println!(
            "x: {:?}",
            initial_states.iter().map(|i| i.get_x()).collect::<Vec<_>>()
        );
        println!(
            "y: {:?}",
            initial_states.iter().map(|i| i.get_y()).collect::<Vec<_>>()
        );
        println!(
            "vel_x: {:?}",
            initial_states
                .iter()
                .map(|i| i.get_vel_x())
                .collect::<Vec<_>>()
        );
        println!(
            "vel_y: {:?}",
            initial_states
                .iter()
                .map(|i| i.get_vel_y())
                .collect::<Vec<_>>()
        );

        for _ in 0..time_periods {
            states = tick(time_period_sec, &states);
            for (i, s) in states.iter().enumerate() {
                data[i].push((s.get_x() as f64, s.get_y() as f64))
            }
        }

        println!(
            "masses: {:?}",
            states.iter().map(|i| i.get_mass()).collect::<Vec<_>>()
        );
        println!(
            "x: {:?}",
            states.iter().map(|i| i.get_x()).collect::<Vec<_>>()
        );
        println!(
            "y: {:?}",
            states.iter().map(|i| i.get_y()).collect::<Vec<_>>()
        );
        println!(
            "vel_x: {:?}",
            states.iter().map(|i| i.get_vel_x()).collect::<Vec<_>>()
        );
        println!(
            "vel_y: {:?}",
            states.iter().map(|i| i.get_vel_y()).collect::<Vec<_>>()
        );

        data
    }

    fn init_stable_figure_eight() -> Vec<MotionState> {
        let state0 = MotionState::new(
            (1.0 * PRECISION_FLOAT) as u64,
            (-0.97000436 * PRECISION_FLOAT) as i64,
            (0.24208753 * PRECISION_FLOAT) as i64,
            (0.4662036850 * PRECISION_FLOAT) as i64,
            (0.4323657300 * PRECISION_FLOAT) as i64,
        );
        let state1 = MotionState::new(
            (1.0001 * PRECISION_FLOAT) as u64,
            (0.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
            (-0.933249737 * PRECISION_FLOAT) as i64,
            (-0.86473146 * PRECISION_FLOAT) as i64,
        );
        let state2 = MotionState::new(
            (1.0002 * PRECISION_FLOAT) as u64,
            (0.97000436 * PRECISION_FLOAT) as i64,
            (-0.24208753 * PRECISION_FLOAT) as i64,
            (0.4662036850 * PRECISION_FLOAT) as i64,
            (0.4323657300 * PRECISION_FLOAT) as i64,
        );

        vec![state0, state1, state2]
    }

    fn single_orbit() -> Vec<MotionState> {
        let state0 = MotionState::new(
            (1.0 * PRECISION_FLOAT) as u64,
            (0.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
        );
        let state1 = MotionState::new(
            (0.0001 * PRECISION_FLOAT) as u64,
            (0.0 * PRECISION_FLOAT) as i64,
            (1.0 * PRECISION_FLOAT) as i64,
            (1.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
        );

        vec![state0, state1]
    }

    fn double_orbit() -> Vec<MotionState> {
        let state0 = MotionState::new(
            (4.0001 * PRECISION_FLOAT) as u64,
            (0.0 * PRECISION_FLOAT) as i64,
            (1.0 * PRECISION_FLOAT) as i64,
            (1.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
        );
        let state1 = MotionState::new(
            (4.0 * PRECISION_FLOAT) as u64,
            (0.0 * PRECISION_FLOAT) as i64,
            (-1.0 * PRECISION_FLOAT) as i64,
            (-1.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
        );

        vec![state0, state1]
    }

    fn our_separate_ways() -> Vec<MotionState> {
        let state0 = MotionState::new(
            (1.0 * PRECISION_FLOAT) as u64,
            (0.0 * PRECISION_FLOAT) as i64,
            (1.0 * PRECISION_FLOAT) as i64,
            (0.3 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
        );
        let state1 = MotionState::new(
            (1.0001 * PRECISION_FLOAT) as u64,
            (-1.0 * PRECISION_FLOAT) as i64,
            (-1.0 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
            (0.3 * PRECISION_FLOAT) as i64,
        );
        let state2 = MotionState::new(
            (1.0002 * PRECISION_FLOAT) as u64,
            (1.0 * PRECISION_FLOAT) as i64,
            (-1.0 * PRECISION_FLOAT) as i64,
            (-0.3 * PRECISION_FLOAT) as i64,
            (0.0 * PRECISION_FLOAT) as i64,
        );

        vec![state0, state1, state2]
    }

    fn draw_chart(file_name: &str, data: Vec<Vec<(f64, f64)>>) {
        let root = BitMapBackend::new(file_name, (480, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-2f64..2f64, -2f64..2f64)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        let colors = vec![RED, BLUE, GREEN, ORANGE];

        for (i, series) in data.iter().enumerate() {
            chart
                .draw_series(series.iter().map(|&(x, y)| {
                    Circle::new(
                        (x / PRECISION_FLOAT, y / PRECISION_FLOAT),
                        1,
                        colors[i].filled(),
                    )
                }))
                .unwrap();
        }
    }

    #[test]
    fn test_single_orbit() {
        draw_chart(
            "./test_plots/single_orbit.png",
            get_orbit_data(single_orbit(), 4000),
        );
    }

    #[test]
    fn test_double_orbit() {
        draw_chart(
            "./test_plots/double_orbit.png",
            get_orbit_data(double_orbit(), 4000),
        );
    }

    #[test]
    fn test_our_separate_ways() {
        draw_chart(
            "./test_plots/our_separate_ways.png",
            get_orbit_data(our_separate_ways(), 4000),
        );
    }

    #[test]
    fn test_fig_eight() {
        draw_chart(
            "./test_plots/fig_eight.png",
            get_orbit_data(init_stable_figure_eight(), 4000),
        );
    }
}
