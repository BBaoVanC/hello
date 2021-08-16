use rand::seq::SliceRandom;

fn main() {
    let mut hello = gen_hello();
    let mut i = 1;

    while hello != "hello" {
        println!("{}", hello);
        i += 1;
        hello = gen_hello();
    }

    println!("{}", hello);
    println!("done in {} iterations", i);
}

fn gen_hello() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = "hello".to_string().into_bytes();
    bytes.shuffle(&mut rng);
    String::from_utf8(bytes).unwrap()
}
