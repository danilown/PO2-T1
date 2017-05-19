/**
 * Recebe um n que eh o comprimento da sequincia de fibonacci
 * Retorna um vetor contendo a sequencia de fibonacci com um determinado comprimento n (não aceita valores menores que 1)
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

	for i in 2..10 { 
		let ant = fibo_seq[i - 1];
		let ant_ant = fibo_seq[i - 2];
		fibo_seq.push(ant + ant_ant);
	}

	fibo_seq
} 

/**
 * Recebe uma string (funcao), um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada primeira da funcao dada no ponto
 */
fn get_first_derivative(){

}

/**
 * Recebe uma string (funcao), um erro, e um ponto de dimencao n (n >=1), o valor de n, e h que eh o intervalo de 
 * avaliacao(nesta aplicacao ele sera fixo, mas por motivos de generalizacao, sera um parametro da funcao). A funcao sempre 
 * recebera da chamadora um h = 1000 * erro 
 * Retorna o valor da derivada segunda da funcao dada no ponto
 */
fn get_second_derivative(){

}
