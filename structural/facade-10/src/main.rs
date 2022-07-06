pub mod rock_band;

fn main() {
    println!("first act -> BLACK SABBATH");
    let black_sabbath = rock_band::black_sabbath::BlackSabbath::new();
    black_sabbath.play_cool_song();
    
    println!("---------------------------------------------------------");
    println!("second act -> ROLLING STONES");
    let rolling_stones = rock_band::rolling_stones::RollingStones::new();
    rolling_stones.play_cool_song();
}