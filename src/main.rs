use rand::Rng;

#[derive(Debug)]
struct Key {
    x: i64,
    y: i64,
}

fn generate_seeds(secret: i64) -> Vec<Key> {
    let mut rng = rand::thread_rng();
    let factor: i64 = rng.gen_range(0..100);
    let mut shared_keys: Vec<Key> = Vec::new();
    let mut i: i64 = 1;
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

fn recover_secret(seeds: Vec<Key>) -> i64 {
    let l0 = (seeds[0].y * seeds[1].x) / (seeds[1].x - seeds[0].x);
    let l1 = (seeds[1].y * seeds[0].x) / (seeds[0].x - seeds[1].x);

    return l0 + l1;
}

fn main() {
    let mut rng = rand::thread_rng();
    let secret = rng.gen_range(1..100);
    print!("secret: {}\n", secret);
    let mut seeds: Vec<Key> = generate_seeds(secret);
    seeds.remove(2);
    seeds.remove(2);
    let rec_secret: i64 = recover_secret(seeds);

    print!("recovered secret: {}", rec_secret);
}
