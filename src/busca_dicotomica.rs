/* recebe a funcao, o limite inferior e o limite superior do intervalo, o tamanho do passo(epsilon) e o intervalo de incerteza(l) .
   Retorna o ponto onde o valor da funcao eh minimo */
fn busca_dicotomica(func: &Fn(f64) -> f64, mut lower_bound: f64, mut upper_bound: f64, passo: f64, intervalo_incerteza: f64) -> f64 {  
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