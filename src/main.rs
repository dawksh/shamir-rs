use rand::Rng;

#[derive(Debug)]
struct Key {
    x: u64,
    y: u64,
}

fn generateSecrets(secret: u64) -> Vec<Key> {
    let mut rng = rand::thread_rng();
    let factor: u64 = rng.gen();
    let mut shared_keys: Vec<Key> = Vec::new();
    let mut i: u64 = 1;
    loop {
        let temp_keys: Key = Key {
            x: i,
            y: secret + factor * i,
        };
        shared_keys.push(temp_keys);
        i += 1;
        if i == 5 {
            break;
        };
    }
    return shared_keys;
}

fn main() {
    print!("{:?}", generateSecrets(64));
}
