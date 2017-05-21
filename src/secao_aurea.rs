/* recebe a funcao, o limite inferior e o limite superior do intervalo e o intervalo de incerteza.
   Retorna o ponto onde o valor da funcao eh minimo */
fn secao_aurea(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, intervalo_incerteza: f64) -> f64 {
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