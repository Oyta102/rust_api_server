use rand::Rng;

pub fn generate_random_string(length:usize) ->String{
    const CHARSET: &str = "123456789abcdefghjkmnpqrstuvwxyzABCDEFGHJKMNPQRSTUVWXYZ";
    let mut rng = rand::thread_rng();
    let mut string = String::with_capacity(length);

    for _ in 0..length {
        let random_index = rng.gen_range(0..CHARSET.len());
        string.push(CHARSET.chars().nth(random_index).unwrap());
    }
    string
}