use rand::Rng;

const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC: &[u8] = b"0123456789";

pub struct Random {}

impl Random {
    pub fn rand_string(length: u8, kind: &[u8]) -> String {
        let mut rng = rand::thread_rng();

        let output: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..kind.len());
                kind[idx] as char
            })
            .collect();
	output
    }
    pub fn rand_alpha(length: u8) -> String {
	Random::rand_string(length, ALPHA)
    }

    pub fn rand_numeric(length: u8) -> String {
	Random::rand_string(length, NUMERIC)
    }
}

