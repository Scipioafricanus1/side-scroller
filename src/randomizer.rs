use crate::prelude::*;
use std::{cmp::max, cmp::min};

use rand::Rng;

pub fn do_roll( stat: u32) -> u32{
    let mut rng = rand::thread_rng();
    let lower_bound = max(0u32, stat-RAND_VARIANCE);
    let higher_bound = min(100u32, stat+RAND_VARIANCE);
    rng.gen_range(lower_bound..higher_bound)
}