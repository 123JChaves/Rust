use std::f64;

fn eh_primo(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn calcular_logaritmo(base: f64, numero: f64) -> f64 {
    numero.log(base)
}

fn calcular_potenciacao(base: f64, expoente: f64) -> f64 {
    base.powf(expoente)
}

fn calcular_equacao_primeiro_grau(a: f64, b: f64) -> f64 {
    -b / a
}

fn calcular_equacao_segundo_grau(a: f64, b: f64, c: f64) {
    let delta = b * b - 4.0 * a * c;
    if delta < 0.0 {
        println!("Não há raízes reais.");
    } else if delta == 0.0 {
        let raiz = -b / (2.0 * a);
        println!("Raiz única: {}", raiz);
    } else {
        let raiz1 = (-b + delta.sqrt()) / (2.0 * a);
        let raiz2 = (-b - delta.sqrt()) / (2.0 * a);
        println!("Raízes: {} e {}", raiz1, raiz2);
    }
}

fn main() {
    println!("Digite o início do intervalo:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let n1: i32 = input.trim().parse().expect("Por favor, digite um número");

    println!("Digite o fim do intervalo:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let n2: i32 = input.trim().parse().expect("Por favor, digite um número");

    println!("Números primos no intervalo [{}, {}]:", n1, n2);
    for i in n1..=n2 {
        if eh_primo(i) {
            print!("{} ", i);
        }
    }
    println!();

    println!("Digite a base para o logaritmo:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let base: f64 = input.trim().parse().expect("Por favor, digite um número");

    println!("Digite o número para o logaritmo:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let numero: f64 = input.trim().parse().expect("Por favor, digite um número");
    println!("Logaritmo de {} na base {}: {}", numero, base, calcular_logaritmo(base, numero));

    println!("Digite a base para a potenciação:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let base_potenciacao: f64 = input.trim().parse().expect("Por favor, digite um número");

    println!("Digite o expoente para a potenciação:");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let expoente: f64 = input.trim().parse().expect("Por favor, digite um número");
    println!("{} elevado a {}: {}", base_potenciacao, expoente, calcular_potenciacao(base_potenciacao, expoente));

    println!("Digite o coeficiente a da equação de primeiro grau (ax + b = 0):");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let a: f64 = input.trim().parse().expect("Por favor, digite um número");

    println!("Digite o coeficiente b da equação de primeiro grau (ax + b = 0):");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let b: f64 = input.trim().parse().expect("Por favor, digite um número");
    println!("Raiz da equação de primeiro grau: {}", calcular_equacao_primeiro_grau(a, b));

    println!("Digite o coeficiente a da equação de segundo grau (ax^2 + bx + c = 0):");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let a_segundo_grau: f64 = input.trim().parse().expect("Por favor, digite um número");

    println!("Digite o coeficiente b da equação de segundo grau (ax^2 + bx + c = 0):");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let b_segundo_grau: f64 = input.trim().parse().expect("Por favor, digite um número");

    println!("Digite o coeficiente c da equação de segundo grau (ax^2 + bx + c = 0):");
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler linha");
    let c_segundo_grau: f64 = input.trim().parse().expect("Por favor, digite um número");
    
    calcular_equacao_segundo_grau(a_segundo_grau, b_segundo_grau, c_segundo_grau);
}