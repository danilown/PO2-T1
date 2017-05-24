/* Busca uniforme com uso do refinamento
   recebe a funcao, o limite inferior e o limite superior do intervalo e o tamanho do passo.
   Retorna o ponto onde o valor da funcao eh minimo */
fn busca_uniforme(func: &Fn(f64) -> f64, lower_bound: f64, upper_bound: f64, mut passo: f64) -> f64 { 
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