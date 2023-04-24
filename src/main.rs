use rand::Rng;

fn generateSecrets(secret: u64) {
    let mut rng = rand::thread_rng();
    let factor: u64 = rng.gen();
}

fn main() {
    generateSecrets(64);
}
