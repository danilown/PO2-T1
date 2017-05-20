
/////////////////////////////////////// FIBONACCI ///////////////////////////////////////////////////////////
/**
 * Recebe um n que eh o comprimento da sequincia de fibonacci
 * Retorna um vetor contendo a sequencia de fibonacci com um determinado comprimento n (não aceita valores menores que 1)
 * NOTA: em sistemas 64 bits, n <= 93, valores maiores causam overflow.
 */
fn get_fibo_seq(n: u64) -> Vec<u64> {
	let mut fibo_seq :Vec<u64> = Vec::with_capacity(n as usize);

	// colocando 1 em fibo_seq[0]
	fibo_seq.push(1);
	// comprimento da sequencia eh 1
	if n == 1{
		return fibo_seq;
	}

	// colocando 1 em fibo_seq[1]
	fibo_seq.push(1);
	// comprimento da sequencia eh 2
	if n == 2{
		return fibo_seq;
	}

	for i in 2..(n as usize) { 
		let ant = fibo_seq[i - 1];
		let ant_ant = fibo_seq[i - 2];
		fibo_seq.push(ant + ant_ant);
	}

	fibo_seq
} 

/////////////////////////////////////// DERIVADAS PRIMEIRAS /////////////////////////////////////////////////
/**
 * Recebe uma funcao de uma variavel, um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada primeira para um funcao de uma variavel
 */
fn get_first_derivative1(func: &Fn(f64) -> f64, x: f64, h: f64, err: f64) -> f64 {
	let mut p: f64;
	let mut q: f64;
	let mut local_h = h;

	p = (func(x + local_h) - func(x - local_h)) / (2. * local_h);
    for i in 0..10 {
    	q = p;
    	local_h = local_h / 2.;
		p = (func(x + local_h) - func(x - local_h )) / (2. * local_h);
		let dif = p - q;
		if dif.abs() < err {
			break;
		}
    }
    p
}

/**
 * Recebe uma funcao de duas variaveis, um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada primeira para um funcao de duas variaveis
 */
fn get_first_derivative2(funcao: &srt, ponto: Vec<f64>, err: f64, h: f64) -> f64 {
	let mut p: f64;
	let mut q: f64;
	let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    for i in 0..10 {
    	p = func()
    }
}


/**
 * Recebe uma funcao de 3 variaveis, um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada primeira para um funcao de tres variaveis
 */
fn get_first_derivative3(funcao: &srt, ponto: Vec<f64>, err: f64, h: f64) -> f64 {
	let mut p: f64;
	let mut q: f64;
	let expr: meval::Expr = string.parse().unwrap();
    let func = expr.bind("x").unwrap();

    for i in 0..10 {
    	p = func()
    }
}


/////////////////////////////////////// DERIVADAS SEGUNDAS //////////////////////////////////////////////////
/**
 * Recebe uma string (funcao), um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada segunda da funcao dada no ponto
 */
fn get_second_derivative() -> f64 {

}
