fn main() {
    let x = 5;
    copy_scalar(x);
    println!("x {}", x);

    let mut s = String::from("Rust");
    s = move_string(s);
    //println!("{}", s); //<--- erro: s não é mais valido
}

fn copy_scalar(n: i32) {
    println!("n {}", n);
}
//Área de memória não exite mais quando a função é finalizada
fn move_string(s: String) -> String {
    println!("{}", s);
    s
}
