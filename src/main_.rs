// Conceitos de Heao e Stack usando strings, uso do metodo clone

fn main() {
    let string_na_stack = "String na stack"; //hardcoded no executável, &str imutável

    // alocado na heap, String é mutável
    let mut string_na_heap = String::from("String na heap");
    string_na_heap.push_str(" com push_str");
    println!("{}", string_na_stack);

    //Rust não permite um outro dono, código vai dar erro
    let s2 = string_na_heap;

    //println!("{}", string_na_heap); // da erro se chamar o outro dono, que não é mais dono.
    println!("{}", s2);

    //o Clone copia todo o valor e associa a um outro dono, no entanto o método é custoso
    let s3 = s2.clone();
    println!("{} {}", s2, s3);

    for _ in 0..10 {
        let s = String::from("Outra String");
        println!("{}", s);
    }
}
