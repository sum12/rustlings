// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    let mut x = -3;
    call_me(&mut x);
    println!("And after the call {}, mutated successfully", x);
}

fn call_me(num: &mut i32) {
    for i in *num..0 {
        println!("Ring! Call number {}", i + 1);
    }
    *num = *num + 3;
    println!("Ring! Call number {}", *num);
}
