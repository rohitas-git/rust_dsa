
/// computed as target % table_capacity
pub fn modular_hash(target: &u32, table_capacity: usize) -> usize {
    *target as usize % table_capacity
}


/// computed as floor[m * frac(ka)]
/// m = table_capacity
/// ka = target * a
/// a = 0.314159
pub fn multiplicative_hash(target: &u32, table_capacity: usize) -> usize {
    let a = 0.314159_f32;
    let ka = *target as f32 * a;
    let frac_part = ka % 1 as f32;
    return (table_capacity as f32 * frac_part).floor() as usize;
}
