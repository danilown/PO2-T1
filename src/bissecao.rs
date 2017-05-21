fn bissecao(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, err: f64) -> f64 {

    loop {
        let x: f64 = (upper_bound + lower_bound) / 2.;
        let derivada = get_first_derivative1(func, x, 1 as f64, err / 100.);
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
