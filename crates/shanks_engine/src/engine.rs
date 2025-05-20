use shanks_core::board::{Backend, Board, Color};

use crate::static_eval;

const SEARCH_DEPTH: usize = 4;

pub struct Engine {
    maximizing_color: Color,
}

impl Engine {
    pub fn evaluate(&self, board: &mut Board) -> f64 {
        self.search(board.get_backend_mut(), SEARCH_DEPTH)
    }

    pub fn search(&self, backend: &mut Box<dyn Backend>, depth: usize) -> f64 {
        if depth == 0 {
            return static_eval::static_eval(backend, self.maximizing_color);
        }

        let mut best_score = f64::MIN;
        let mut best_plies = Vec::new();

        for ply in backend.get_legal_plies(self.maximizing_color) {
            let mut backend_clone = backend.clone();
            backend_clone.ply(ply.clone());

            let score = self.alpha_beta(&backend_clone, depth - 1, f64::MIN, f64::MAX, false);

            if score > best_score {
                best_score = score;
                best_plies = vec![ply];
            } else if score == best_score {
                best_plies.push(ply);
            }
        }

        if best_plies.is_empty() {
            return f64::MIN;
        }

        if best_plies.len() == 1 {
            println!("Best move: {} ~ {}", best_plies[0], best_score);
        } else {
            println!("Best moves: {:?} ~ {}", best_plies, best_score);
        }

        best_score
    }

    fn alpha_beta(
        &self,
        backend: &Box<dyn Backend>,
        depth: usize,
        mut alpha: f64,
        mut beta: f64,
        maximizing: bool,
    ) -> f64 {
        todo!("Implement gameover edge case");

        // if depth == 0 {
        //     return static_eval::static_eval(backend);
        // }

        // let mut best_score = if maximizing { f64::MIN } else { f64::MAX };

        // for ply in
        //     backend.get_legal_plies(if maximizing { self.maximizing_color } else { self.maximizing_color.opposite() })
        // {
        //     let mut backend_clone = backend.clone();
        //     backend_clone.ply(ply.clone());

        //     let score = self.alpha_beta(&backend_clone, depth - 1, alpha, beta, !maximizing);

        //     if maximizing {
        //         best_score = best_score.max(score);
        //         alpha = alpha.max(score);
        //     } else {
        //         best_score = best_score.min(score);
        //         beta = beta.min(score);
        //     }

        //     if beta <= alpha {
        //         break;
        //     }
        // }
    }
}
