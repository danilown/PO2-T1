/* recebe a funcao, o limite inferior e o limite superior do intervalo e o erro.
   Retorna o ponto onde o valor da funcao eh minimo 
   neste metodo x inicial (chute inicial) eh definido como lower_bound */
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
        if (get_first_derivative1(func, x_prox, 1 as f64, err / 100.).abs() < err) || ((x_prox - x_atual).abs() / x_prox.max(1.) < err) {
            return x_prox;
        }
        x_atual = x_prox;
    }
}