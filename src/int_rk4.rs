use serde::{Deserialize, Serialize};

use alloc::vec::Vec;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionState {
    mass: u64,
    x: i64,
    y: i64,
    vel_x: i64,
    vel_y: i64,
}

impl MotionState {
    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn get_vel_x(&self) -> i64 {
        self.vel_x
    }

    pub fn get_vel_y(&self) -> i64 {
        self.vel_y
    }

    pub fn new(mass: u64, x: i64, y: i64, vel_x: i64, vel_y: i64) -> Self {
        Self {
            mass,
            x,
            y,
            vel_x,
            vel_y,
        }
    }

    pub fn apply<F>(self: &MotionState, func: F) -> MotionState
    where
        F: Fn(i64) -> i64,
    {
        let new_x = func(self.x);
        let new_y = func(self.y);
        let new_vel_x = func(self.vel_x);
        let new_vel_y = func(self.vel_y);

        MotionState {
            mass: self.mass,
            x: new_x,
            y: new_y,
            vel_x: new_vel_x,
            vel_y: new_vel_y,
        }
    }

    pub fn apply_other<F>(self: &MotionState, other_state: &MotionState, func: F) -> MotionState
    where
        F: Fn(i64, i64) -> i64,
    {
        let new_x = func(self.x, other_state.x);
        let new_y = func(self.y, other_state.y);
        let new_vel_x = func(self.vel_x, other_state.vel_x);
        let new_vel_y = func(self.vel_y, other_state.vel_y);

        MotionState {
            mass: self.mass,
            x: new_x,
            y: new_y,
            vel_x: new_vel_x,
            vel_y: new_vel_y,
        }
    }
}

fn sqrt_heron(x: i64) -> i64 {
    // https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method

    // CHRIS: TODO: dont accept negative in here, throw an error
    if x < 0 {
        return -1;
    }

    let mut x_old = x;
    let mut x_new = (x_old + 1) / 2;
    while x_new < x_old {
        x_old = x_new;
        x_new = (x_old + (x / x_old)) / 2;
    }

    x_new
}

// CHRIS: TODO: better documentation and safety checks throughout

pub const PRECISION: i64 = 100000000;

pub fn gravitational_acceleration(grav_g: i64, mass: u64, d: i64, r: i64) -> i64 {
    (((((-grav_g * mass as i64 * d) * PRECISION) / r) * PRECISION) / r) / r
}

pub fn grav_rate_func(state: &MotionState, system: &Vec<MotionState>) -> MotionState {
    let mut acc_x = 0;
    let mut acc_y = 0;

    for s in system {
        // use mass as an identifier
        if s.mass != state.mass {
            let dx = state.x - s.x;
            let dy = state.y - s.y;
            let r = sqrt_heron((dx * dx) + (dy * dy));

            // assume gravity of 1
            acc_x += gravitational_acceleration(1, s.mass, dx, r);
            acc_y += gravitational_acceleration(1, s.mass, dy, r);
        }
    }

    let d_vx = acc_x;
    let d_vy = acc_y;

    let d_x = state.vel_x;
    let d_y = state.vel_y;

    MotionState {
        mass: state.mass,
        x: d_x,
        y: d_y,
        vel_x: d_vx,
        vel_y: d_vy,
    }
}

pub fn rk4<F>(time_period_sec: i64, state: &MotionState, func: F) -> MotionState
where
    F: Fn(&MotionState) -> MotionState,
{
    let k1 = &func(&state).apply(|k| (k * time_period_sec) / PRECISION);
    let k2 = &func(&state.apply_other(&k1, |s, k| s + k / 2))
        .apply(|k| (k * time_period_sec) / PRECISION);
    let k3 = &func(&state.apply_other(&k2, |s, k| s + k / 2))
        .apply(|k| (k * time_period_sec) / PRECISION);
    let k4 =
        &func(&state.apply_other(&k2, |s, k| s + k)).apply(|k| (k * time_period_sec) / PRECISION);

    let k1_k2 = &k1.apply_other(&k2, |k1, k2| k1 + 2 * k2);
    let k3_k4 = &k3.apply_other(&k4, |k3, k4| 2 * k3 + k4);
    let k1_k2_k3_k4 = &k1_k2.apply_other(&k3_k4, |k1_k2, k3_k4| (k1_k2 + k3_k4) / 6);

    return state.apply_other(k1_k2_k3_k4, |s, k| s + k);
}

pub fn tick(time_period_sec: i64, system: &Vec<MotionState>) -> Vec<MotionState> {
    let mut next_system = Vec::new();

    for state in system {
        next_system.push(rk4(time_period_sec, &state, |s| grav_rate_func(s, &system)));
    }

    next_system
}

pub fn tick_many(ticks: u32, time_period_sec: i64, system: &Vec<MotionState>) -> Vec<MotionState> {
    let mut next_system = system.clone();
    for _ in 0..ticks {
        next_system = tick(time_period_sec, &next_system);
    }
    next_system
}
