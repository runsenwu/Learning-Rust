enum Animals {
    Cats(u32, usize),
    Dogs(String),
    Elephants {
        foot: usize,
        trunk: String
    },
} 

fn main() {
    Animals::Cats(18, 32);

    let some_test = Some(5);
    let test: Option<i32> = None;

    some_test
}


fn animal_type(animal: Animals) {
    match {
        Animals::Cats() => 1,
        Animals::Dogs() => 2,
        Animals::Elephants() =>
    }
}