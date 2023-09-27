use crate::int_rk4::{tick, MotionState};
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn tick_wasm(time_period_sec: i64, system: &JsValue) -> Result<JsValue, JsError> {
    let system_ms: Vec<MotionState> = from_value(system.clone())?;
    let res = tick(time_period_sec, &system_ms);
    Ok(to_value(&res)?)
}
