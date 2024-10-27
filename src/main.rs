// 2. Função de Soma com Empréstimo Imutável

fn soma(vec_slice: &[i32]) -> i32 {
    vec_slice.iter().sum()
}

// 4. Empréstimo Mutável
fn adicionar_prefixo(s: &mut String) {
    s.insert_str(0, "Prefixo: ")
}

// 5. Multiplicação de Vetores com Empréstimo Imutável
fn mutiplic_vec(vec1: &[i32], vec2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for (a, b) in vec1.iter().zip(vec2.iter()) {
        result.push(a * b);
    }

    result
}
// 6. Número de Caracteres com Empréstimo Imutável

fn contar_carcacteres(s: &String) -> usize {
    s.chars().count()
}

//7. Split de String com Empréstimo Imutável
fn split_string(str7: &String) -> Vec<&str> {
    str7.split_whitespace().collect()
}

//8. Função com Várias Referências Imutáveis
fn concatenar(str81: &String, str82: &String) -> String {
    format!("{} {}", str81, str82)
}

//9. Função com Referência Mutável e Imutável
fn adicionar_elemento(vetor9: &mut Vec<i32>, n: &i32) -> Vec<i32> {
    let mut result = Vec::new();
    for a in vetor9.iter() {
        result.push(a + n);
    }
    result
}

//10. Ciclo de Empréstimo Mutável
fn double_vector(numeros: &mut Vec<i32>) {
    for n in numeros.iter_mut() {
        *n *= 2;
    }
}

fn main() {
    //1. Clonando e Movendo Strings
    println!("Exercícios 1");
    let s1 = String::from("Rust");
    let s2 = String::from("Blockchian");

    //Clonado, é feita uma cópia da ára de memória alocada e ambas as variáveis
    // são validas
    let s3 = s2.clone();
    println!("String original: {}, String clonada: {}", s2, s3);
    //movido, no caso o valor do s1 não será mais valido pois foi movido
    // para s4
    let s4 = s1;
    //--> error: value borrowed here after move
    //println!("String original: {}", s1);
    println!("String movida: {}", s4);
    println!();

    // 2.Função de Soma com Empréstimo Imutável
    println!("Exercício 2");

    let vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let soma = soma(&vec1[..]);

    println!("Vetor original: {:?}", vec1);
    println!("Soma dos números: {}", soma);
    println!();

    // 4. Empréstimo Mutável
    println!("Exercício 4");
    let mut s = String::from("Sou uma String");
    adicionar_prefixo(&mut s);
    println!("String modificada: {}", s);
    println!();

    // 5. Multiplicação de Vetores com Empréstimo Imutável
    println!("Exercício 5");
    let v1 = vec![1, 4, 5, 6, 8];
    let v2 = vec![3, 8, 12, 3, 7];
    println!("Vetor Resultante: {:?}", mutiplic_vec(&v1, &v2));
    println!();

    // 6. Número de Caracteres com Empréstimo Imutável
    println!("Exercício 6");

    let str1 = String::from("Sou uma String");
    let result6 = contar_carcacteres(&str1);
    println!("Número de caracteres: {}", result6);
    println!();

    //7. Split de String com Empréstimo Imutável
    println!("Exercício 7");

    let str7 = String::from("String com espaços em branco");
    let result7 = split_string(&str7);
    println!("O vector resultante é: {:?}", result7);
    println!();

    //8. Função com Várias Referências Imutáveis
    println!("Exercício 8");

    let str81 = String::from("String número um");
    let str82 = String::from("concatenada com string número 2");
    let result8 = concatenar(&str81, &str82);
    println!("O vector resultante é: {:?}", result8);
    println!();

    //9. Função com Referência Mutável e Imutável
    println!("Exercício 9");
    let mut vec9 = vec![8, 10, 11, 18, 25];
    let num = 5;
    let result9 = adicionar_elemento(&mut vec9, &num);
    println!(
        "O vetor resultante da soma {} com {:?} é: {:?}",
        num, vec9, result9
    );
    println!();

   //10. Ciclo de Empréstimo Mutável
    println!("Exercício 10");
    let mut vector10 = vec![8, 10, 11, 18, 25];
    double_vector(&mut vector10);
    println!("O vetor resultante é: {:?}", vector10);

    println!();
}
