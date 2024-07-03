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

/// computed: (str[0]*(x^0) + str[1]*(x^1) + str[2]*(x^2) + str[3]*(x^4)) % m ,
/// where x = 33
pub fn weighted_string_hash(target: &str, table_capacity: usize) -> usize {
    let mut sum = 0;
    let x = 33_usize;
    for (i, c) in target.chars().enumerate() {
        sum += c as usize * x.pow(i as u32) ;
    }
    return sum % table_capacity;
}
