
/////////////////////////////////////// FIBONACCI ///////////////////////////////////////////////////////////
/**
 * Recebe um n que eh o valor anterior ao mais alto da sequencia de fibonacci
 * Ex.: se n = 90 , a sequencia tera tamanho 11 pois o elemento 11 da sequencia eh 144, que eh maior que 90 e o elemento 10 eh 89 que eh menor que 90
 * Retorna um vetor contendo a sequencia de fibonacci com um determinado comprimento n (não aceita valores menores que 1)
 * NOTA: em sistemas 64 bits, n <= 93, valores maiores causam overflow.
 */
// Testes fibonacci
fn get_fibo_seq(n: u64) -> Vec<u64> {
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

/////////////////////////////////////// DERIVADA PRIMEIRA /////////////////////////////////////////////////
/**
 * Recebe uma funcao de uma variavel, um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada primeira para um funcao de uma variavel
 */
fn get_first_derivative1(func: &Fn(f64) -> f64, x: f64, mut h: f64, err: f64) -> f64 {
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

/////////////////////////////////////// DERIVADA SEGUNDA //////////////////////////////////////////////////
/**
 * Recebe uma string (funcao), um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada segunda da funcao dada no ponto
 */
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
