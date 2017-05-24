extern crate meval;

// Fucao criada pela meval passada por parametro
/*fn function_param(f: &Fn(f64, f64) -> f64) -> f64 {
    println!("4 * 4 + 1 = {}", f(4 as f64, 1 as f64));
	f(4 as f64, 1 as f64)

}


fn main() {
	let string: &str = "x1 * x1 + y2";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind2("x1", "y2").unwrap();

    // let vs: Vec<_> = (0..100+1).map(|i| func(i as f64 / 100.)).collect();
    println!("4 * 4 + 1 = {}", function_param(&*func));
}*/

/// Testes de derivadas
/*fn get_first_derivative1(func: &Fn(f64) -> f64, x: f64, mut h: f64, err: f64) -> f64 {
	let mut p: f64;
	let mut q: f64;

	p = (func(x + h) - func(x - h)) / (2. * h);
    for _ in 0..10 {
    	q = p;
    	h = h / 2.;
		p = (func(x + h) - func(x - h )) / (2. * h);
		if (p - q).abs() < err {
			break;
		}
    }
    p
}

fn get_second_derivative1(func: &Fn(f64) -> f64, x: f64, mut h: f64, err: f64) -> f64 {
    let mut p: f64;
    let mut q: f64;

    p = (func(x + 2. * h) - 2. * func(x) + func(x - 2. * h)) / (2. * h).powi(2);
    for _ in 0..10 {
        q = p;
        h = h / 2.;
        p = (func(x + 2. * h) - 2. * func(x) + func(x - 2. * h)) / (2. * h).powi(2);
        if (p - q).abs() < err {
            break;
        }
    }
    p
}

fn main() {
	let string: &str = "sin(x)";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    let h: f64 = 1 as f64;
    // let vs: Vec<_> = (0..100+1).map(|i| func(i as f64 / 100.)).collect();
    println!("a derivada segunda de  = {}", get_second_derivative1(&*func, 0.5 as f64, h, 0.001 as f64));
    println!("O valor de h eh: {}", h);
}*/

// Testes fibonacci
/*fn get_fibo_seq(n: u64) -> Vec<u64> {
    let mut fibo_seq: Vec<u64> = Vec::new();
    let mut next_element: u64;
    let mut i: u8 = 2; // a sequencia de fibonacci nao sera maior que 93 (overflow) e 93 cabe em 8 bits

    // colocando 1 em fibo_seq[0]
    fibo_seq.push(1);
    // se n eh 0, o maior elemento da sequencia apos ele eh o 1
    if n == 0{
        // eliminando o espaco extra alocado e mantendo somente o necessario
        fibo_seq.shrink_to_fit();
        return fibo_seq;
    }
    
    // colocando 1 em fibo_seq[1]
    fibo_seq.push(1); 

    next_element = fibo_seq[(i - 1) as usize] + fibo_seq[(i - 2) as usize];
    while next_element <= n {
        fibo_seq.push(next_element);
        i += 1;
        next_element = fibo_seq[(i - 1) as usize] + fibo_seq[(i - 2) as usize];
    }
    fibo_seq.push(next_element);
    // eliminando o espaco extra alocado e mantendo somente o necessario
    fibo_seq.shrink_to_fit();
    fibo_seq
}

fn main() {
    let fibo = get_fibo_seq(90 as u64);

    for i in &fibo{
        print!("{} ", i);
    }
    println!("");
    println!("O vector tem {} de tamanho", fibo.len());

}*/

// testes secao_aurea
/*fn secao_aurea(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, intervalo_incerteza: f64) -> f64 {
    let razao_area: f64 = (- 1. + (5 as f64).sqrt() ) / 2.;
    let mut me = lower_bound + (1. - razao_area) * (upper_bound - lower_bound);
    let mut lambda = lower_bound + razao_area * (upper_bound - lower_bound);

    loop {
        if func(me) > func(lambda) {
            lower_bound = me;
            me = lambda;
            lambda = lower_bound + razao_area * (upper_bound - lower_bound);
        }
        else {
            if func(me) < func(lambda) {
                upper_bound = lambda;
                lambda = me;
                me = lower_bound + (1. - razao_area) * (upper_bound - lower_bound);
            }
        }
        if(upper_bound - lower_bound) < intervalo_incerteza {
            break;
        }
    }
    (lower_bound + upper_bound) / 2.
}

fn main() {
    let string: &str = "x^2 + 2*x";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    println!("O ponto de minimo da funcao eh: {}" ,secao_aurea(&*func, -3 as f64, 6 as f64, 0.1 as f64));

}*/

// testes bissecao
/*fn get_first_derivative1(func: &Fn(f64) -> f64, x: f64, mut h: f64, err: f64) -> f64 {
    let mut p: f64;
    let mut q: f64;

    p = (func(x + h) - func(x - h)) / (2. * h);
    for _ in 0..10 {
        q = p;
        h = h / 2.;
        p = (func(x + h) - func(x - h )) / (2. * h);
        if (p - q).abs() < err {
            break;
        }
    }
    p
}

fn bissecao(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, err: f64) -> f64 {
    let mut x: f64 = (upper_bound + lower_bound) / 2.;
    let mut derivada: f64 = get_first_derivative1(func, x, 1 as f64, err / 100.);

    loop {
        x = (upper_bound + lower_bound) / 2.;
        derivada = get_first_derivative1(func, x, 1 as f64, err / 100.);
        // A derivada no ponto eh igual a zero?
        if derivada.abs() < (err / 100.) {
            // x eh ponto de minimo
            return (lower_bound + upper_bound) / 2.;
        }
        else {
            if derivada > 0. {
                // minimo ocorre a esquerda >> x eh o novo limite superior do intervalo
                upper_bound = x;
            }
            else {
                // minimo ocorre a direita >> x eh o novo limite inferior do intervalo
                lower_bound = x;
            }
        }
    }
}

fn main() {
    let string: &str = "x^2 + 2*x";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    println!("O ponto de minimo da funcao eh: {}" ,bissecao(&*func, -3 as f64, 6 as f64, 0.3 as f64));

}*/ 

// testes newton
/*fn get_first_derivative1(func: &Fn(f64) -> f64, x: f64, mut h: f64, err: f64) -> f64 {
    let mut p: f64;
    let mut q: f64;

    p = (func(x + h) - func(x - h)) / (2. * h);
    for _ in 0..10 {
        q = p;
        h = h / 2.;
        p = (func(x + h) - func(x - h )) / (2. * h);
        if (p - q).abs() < err {
            break;
        }
    }
    p
}

fn get_second_derivative1(func: &Fn(f64) -> f64, x: f64, mut h: f64, err: f64) -> f64 {
    let mut p: f64;
    let mut q: f64;

    p = (func(x + 2. * h) - 2. * func(x) + func(x - 2. * h)) / (2. * h).powi(2);
    for _ in 0..10 {
        q = p;
        h = h / 2.;
        p = (func(x + 2. * h) - 2. * func(x) + func(x - 2. * h)) / (2. * h).powi(2);
        if (p - q).abs() < err {
            break;
        }
    }
    p
}

fn newton(func: &Fn(f64) -> f64, lower_bound: f64, upper_bound: f64, err: f64) -> f64 {
    let mut x_atual: f64 = lower_bound; // xk
    let mut x_prox: f64; // xk+1
    // derivada primeira no ponto xk
    let mut derivada_primera: f64;
    // derivada segunda no ponto xk
    let mut derivada_segunda: f64;

    loop {
        derivada_primera = get_first_derivative1(func, x_atual, 1 as f64, err / 100.);
        derivada_segunda = get_second_derivative1(func, x_atual, 1 as f64, err / 100.);
        x_prox = x_atual - (derivada_primera / derivada_segunda);
        /* se a derivada primera da funcao no ponto atual eh menor que o erro aceitavel ou 
           se o modulo da diferenca entre o x anterior e o x atual dividido pelo maior entre 1 e o ponto atual eh menor que o erro aceitavel
        */
        if (get_first_derivative1(func, x_prox, 1 as f64, err / 100.).abs() < err) || ((x_prox - x_atual).abs() / x_prox.max(1.) < err){
            return x_prox;
        }
        x_atual = x_prox;
    }
}

fn main() {
    let string: &str = "e^x - x^3 + 1";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    println!("O ponto de minimo da funcao eh: {}" ,newton(&*func, -3 as f64, 6 as f64, 0.0001 as f64));

}*/

// teste busca_fibo
/*fn get_fibo_seq(n: u64) -> Vec<u64> {
    let mut fibo_seq: Vec<u64> = Vec::new();
    let mut next_element: u64;
    let mut i: u8 = 2; // a sequencia de fibonacci nao sera maior que 93 (overflow) e 93 cabe em 8 bits

    // colocando 1 em fibo_seq[0]
    fibo_seq.push(1);
    // se n eh 0, o maior elemento da sequencia apos ele eh o 1
    if n == 0{
        // eliminando o espaco extra alocado e mantendo somente o necessario
        fibo_seq.shrink_to_fit();
        return fibo_seq;
    }
    
    // colocando 1 em fibo_seq[1]
    fibo_seq.push(1); 

    next_element = fibo_seq[(i - 1) as usize] + fibo_seq[(i - 2) as usize];
    while next_element <= n {
        fibo_seq.push(next_element);
        i += 1;
        next_element = fibo_seq[(i - 1) as usize] + fibo_seq[(i - 2) as usize];
    }
    fibo_seq.push(next_element);
    // eliminando o espaco extra alocado e mantendo somente o necessario
    fibo_seq.shrink_to_fit();
    fibo_seq
}

fn busca_fibo(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, intervalo_incerteza: f64) -> f64 {
    let fibo_seq = get_fibo_seq( ((upper_bound - lower_bound) / intervalo_incerteza).ceil() as u64);
    let n = fibo_seq.len() - 1; // numero do maior indice
    let k_max: usize = n - 1; // numero maximo de iteracoes
    let mut me: f64;
    let mut lambda: f64;

    for k in 0..k_max {
        me = lower_bound + (fibo_seq[(n - k - 2) as usize] as f64 / fibo_seq[(n - k) as usize] as f64) * (upper_bound - lower_bound);
        lambda = lower_bound + (fibo_seq[(n - k - 1) as usize] as f64 / fibo_seq[(n - k) as usize] as f64) * (upper_bound - lower_bound);

        if func(me) > func(lambda) {
            lower_bound = me;
            me = lambda;
            lambda = lower_bound + (fibo_seq[(n - k - 1) as usize] as f64 / fibo_seq[(n - k) as usize] as f64) * (upper_bound - lower_bound);
        }
        else {
            if func(me) < func(lambda) {
                upper_bound = lambda;
                lambda = me;
                me = lower_bound + (fibo_seq[(n - k - 2) as usize] as f64 / fibo_seq[(n - k) as usize] as f64) * (upper_bound - lower_bound);
            }
        }
        if(upper_bound - lower_bound) < intervalo_incerteza {
            break;
        }
    }
    (lower_bound + upper_bound) / 2.
} 

fn main() {
    let string: &str = "x^2 - 3*x + 2";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    println!("O ponto de minimo da funcao eh: {}" ,busca_fibo(&*func, -1 as f64, 3 as f64, 0.1 as f64));

}
*/

// Testes busca_uniforme
/*fn busca_uniforme(func: &Fn(f64) -> f64, lower_bound: f64, upper_bound: f64, mut passo: f64) -> f64 { 
    let mut x_atual: f64 = lower_bound; // x = a
    let mut x_anter: f64 = lower_bound;

    // passos de tamanho normal
    // enquanto a nova aproximacao eh melhor que antiga 
    while (func(x_atual) > func(x_atual + passo)) && ((x_atual + passo) <= upper_bound) {
        x_anter = x_atual;
        x_atual += passo;
    }

    // voltando uma iteracao
    x_atual = x_anter; 
    // passos de tamanho passo/10
    // enquanto a nova aproximacao eh melhor que antiga 
    passo = passo / 10.;
    while (func(x_atual) > func(x_atual + passo)) && ((x_atual + passo) <= upper_bound) {
        x_atual += passo;
    }

    x_atual
}

fn main() {
    let string: &str = "x^2 + 2*x 2";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    println!("O ponto de minimo da funcao eh: {}" ,busca_uniforme(&*func, -3 as f64, 6 as f64, 0.1 as f64));

}
*/

// testes busca dicotomica
/*fn busca_dicotomica(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, passo: f64, intervalo_incerteza: f64) -> f64 {  
    let mut x: f64;
    let mut z: f64;

    while (upper_bound - lower_bound).abs() >= intervalo_incerteza {
        x = (lower_bound + upper_bound) / 2. - passo;
        z = (lower_bound + upper_bound) / 2. + passo;
        if func(x) > func(z){
            lower_bound = x; // raiz a direita (discarte a parte a esquerda do intervalo)
        }
        else {
            upper_bound = z; // raiz a esquerda (discarte a parte a direita do intervalo)
        }
    }
    (lower_bound + upper_bound) / 2.
}

fn main() {
    let string: &str = "x^2 + 2";
    let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    println!("O ponto de minimo da funcao eh: {}" ,busca_dicotomica(&*func, -3 as f64, 6 as f64, 0.01 as f64, 0.1 as f64));

}
*/