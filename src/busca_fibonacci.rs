/* recebe a funcao, o limite inferior e o limite superior do intervalo e o intervalo de incerteza.
   Retorna o ponto onde o valor da funcao eh minimo */
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
