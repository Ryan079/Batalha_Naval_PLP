use godot::prelude::*;
use rand::Rng;

/// IA simples para o jogo de Batalha Naval
pub struct IASimples {
    posicoes_atacadas: Vec<(i32, i32)>,
}

impl IASimples {
    /// Cria uma nova IA
    pub fn nova() -> Self {
        godot_print!("🤖 IA Simples inicializada");
        Self {
            posicoes_atacadas: Vec::new(),
        }
    }

    /// Escolhe uma posição aleatória para atacar que ainda não foi atacada
    /// Retorna None se todas as posições já foram atacadas
    pub fn escolher_ataque(&mut self, tamanho_tabuleiro: i32) -> Option<(i32, i32)> {
        // Se já atacou todas as posições possíveis, retorna None
        let total_posicoes = (tamanho_tabuleiro * tamanho_tabuleiro) as usize;
        if self.posicoes_atacadas.len() >= total_posicoes {
            godot_print!("⚠️ IA já atacou todas as posições!");
            return None;
        }

        let mut rng = rand::thread_rng();
        
        // Tenta encontrar uma posição não atacada (máximo 100 tentativas)
        for _ in 0..100 {
            let x = rng.gen_range(0..tamanho_tabuleiro);
            let y = rng.gen_range(0..tamanho_tabuleiro);
            
            if !self.posicoes_atacadas.contains(&(x, y)) {
                self.posicoes_atacadas.push((x, y));
                godot_print!("🤖 IA escolheu atacar: ({}, {})", x, y);
                return Some((x, y));
            }
        }

        // Se não encontrou depois de 100 tentativas, faz busca exaustiva
        for y in 0..tamanho_tabuleiro {
            for x in 0..tamanho_tabuleiro {
                if !self.posicoes_atacadas.contains(&(x, y)) {
                    self.posicoes_atacadas.push((x, y));
                    godot_print!("🤖 IA escolheu atacar: ({}, {})", x, y);
                    return Some((x, y));
                }
            }
        }

        None
    }

    /// Posiciona navios automaticamente no tabuleiro da IA
    /// Esta é uma função mockada que apenas simula o posicionamento
    pub fn posicionar_navios(&self) {
        godot_print!("🤖 IA posicionando navios automaticamente...");
        // TODO: Implementar lógica real de posicionamento de navios
        // Por enquanto apenas simula o posicionamento
        godot_print!("✅ IA finalizou posicionamento dos navios");
    }

    /// Reseta o histórico de ataques da IA
    pub fn resetar(&mut self) {
        self.posicoes_atacadas.clear();
        godot_print!("🔄 IA resetada");
    }
}
