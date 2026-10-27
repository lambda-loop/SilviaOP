

use rand::{*, rngs::SmallRng};
use grid::Grid;
use crate::problem::Point;

// TODO: improve it!
/*

AI sugestions:
- Avaliação cara: recalcula a rota inteira por tentativa. Com B=\texttt{max_iters} e \(n=\texttt{tour.len()}\), custa \(O(Bn)\).
- Pode repetir pares: gasta tentativas avaliando o mesmo movimento, inclusive quando a rota não mudou.
- Sorteio não uniforme: begin é uniforme, mas cada um tem quantidades diferentes de possíveis end. Assim, pares com begin perto do final têm maior probabilidade individual.
- Não detecta estagnação: termina apenas pelo orçamento. O comentário descreve melhor uma amostragem do que uma varredura.
Sugestões, em ordem de prioridade:
1. Para matriz simétrica, avaliar pelo delta \(O(1)\):
   delta = cost[a,c] + cost[b,d]
         - cost[a,b] - cost[c,d]
   
   Aceite se delta < 0. Isso elimina current_cost e a avaliação completa. A reversão aceita ainda custa \(O(n)\) no pior caso.
2. Se quiser pares uniformes, sorteie dois índices distintos em toda a rota e ordene-os para obter begin < end. Seu sorteio atual é válido; só tem esse viés.
3. Mantenha o orçamento fixo por enquanto. É simples e previsível em número de tentativas. Parar após várias rejeições pode economizar trabalho, mas não comprova estagnação.
*/

// fixed-budget-scan
pub fn two_opt (
    tour : &mut [Point],
    mut current_cost: f64,
    rng  : &mut SmallRng,
    costs: &Grid<f64>,
    max_iters: u64,
) -> f64 {
    let tour_len  = tour.len();
    if tour_len < 2 || max_iters == 0 { return current_cost }

    for _ in 0..max_iters {
        // AI SUGESTED
        let i = rng.random_range(0..tour_len);
        let j = rng.random_range(0..tour_len - 1);
        let j = if j >= i { j + 1 } else { j };
        
        let (begin, end) = if i < j {
            (i, j)
        } else {
            (j, i)
        };

        let new_cost = eval_two_opted_cost(
            tour, costs,
            begin, end,
        );

        if new_cost < current_cost {
            current_cost = new_cost; 
            tour[begin..=end].reverse();
        }
    }

    current_cost
}

fn eval_two_opted_cost(
    tour : &[Point],
    costs: &Grid<f64>,
    begin: usize,
    end  : usize,
) -> f64 {
    // if tour.is_empty() {
    //     return costs[(0, 1)];
    // }

    let points = tour[..begin].iter()
        .chain(tour[begin..=end].iter().rev())
        .chain(tour[end + 1..].iter());

    let mut cost     = 0.;
    let mut previous = 0usize;

    for &point in points {
        cost += costs[(previous, point as usize)];
        previous = point as usize;
    }

    cost + costs[(previous, 1)]
}




