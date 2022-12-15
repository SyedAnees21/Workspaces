use std::mem::size_of_val;
const MAX_HEALTH_BITS: u8 = 3;
const MAX_SCORE_BITS: u8 = 8;
#[allow(dead_code)]
const MAX_ID_BITS: u8 = 4;

pub struct DashboardInfo {
    health: i32,
    score: i32,
    id: i32
}

pub fn run() {
    let dashboard = DashboardInfo {
        health: 5,
        score: 200,
        id: 10,
    };

    let current_size = size_of_val(&dashboard);
    println!("{current_size}");

    let compressed_data = compression(&dashboard);

    println!("{:b}",compressed_data);
    println!("{}", size_of_val(&compressed_data));
}

pub fn compression(pack: &DashboardInfo) -> u16 {
    let mut compressed_half_word: u16;

    let mut bytes = pack.health.to_le_bytes();
    compressed_half_word = (bytes[0] as u16 ) << 0;
    
    bytes = pack.score.to_le_bytes();
    compressed_half_word |= (bytes[0] as u16) << MAX_HEALTH_BITS;

    bytes = pack.id.to_le_bytes();
    compressed_half_word |= (bytes[0] as u16) << (MAX_HEALTH_BITS + MAX_SCORE_BITS);

    compressed_half_word
}