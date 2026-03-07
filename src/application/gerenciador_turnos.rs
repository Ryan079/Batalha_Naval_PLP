use godot::prelude::*;

/// Estado atual do turno no jogo
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EstadoTurno {
    /// Aguardando o jogador posicionar navios
    PosicionamentoJogador,
    /// Aguardando a IA posicionar navios (pode ser automático)
    PosicionamentoIA,
    /// Vez do jogador atacar
    TurnoJogador,
    /// Vez da IA atacar
    TurnoIA,
    /// Jogo finalizado - jogador venceu
    VitoriaJogador,
    /// Jogo finalizado - IA venceu
    VitoriaIA,
}

/// Gerenciador de turnos do jogo
pub struct GerenciadorTurnos {
    /// Estado atual do turno
    estado_atual: EstadoTurno,
    /// Número do turno atual (começa em 1)
    numero_turno: u32,
    /// Total de navios do jogador ainda intactos
    navios_jogador: u32,
    /// Total de navios da IA ainda intactos
    navios_ia: u32,
}

impl GerenciadorTurnos {
    /// Cria um novo gerenciador de turnos
    pub fn novo(total_navios_por_jogador: u32) -> Self {
        godot_print!("🎮 Sistema de Turnos inicializado");
        godot_print!("📊 Cada jogador tem {} navios", total_navios_por_jogador);
        
        Self {
            estado_atual: EstadoTurno::PosicionamentoJogador,
            numero_turno: 0,
            navios_jogador: total_navios_por_jogador,
            navios_ia: total_navios_por_jogador,
        }
    }

    /// Retorna o estado atual do turno
    pub fn estado_atual(&self) -> EstadoTurno {
        self.estado_atual
    }

    /// Retorna o número do turno atual
    pub fn numero_turno(&self) -> u32 {
        self.numero_turno
    }

    /// Retorna quantos navios o jogador ainda tem
    pub fn navios_jogador(&self) -> u32 {
        self.navios_jogador
    }

    /// Retorna quantos navios a IA ainda tem
    pub fn navios_ia(&self) -> u32 {
        self.navios_ia
    }

    /// Finaliza o posicionamento do jogador e avança para o posicionamento da IA
    pub fn finalizar_posicionamento_jogador(&mut self) {
        if self.estado_atual == EstadoTurno::PosicionamentoJogador {
            godot_print!("✅ Jogador finalizou posicionamento dos navios");
            self.estado_atual = EstadoTurno::PosicionamentoIA;
        }
    }

    /// Finaliza o posicionamento da IA e inicia o jogo (turno do jogador)
    pub fn iniciar_jogo(&mut self) {
        if self.estado_atual == EstadoTurno::PosicionamentoIA {
            godot_print!("🚀 JOGO INICIADO!");
            self.numero_turno = 1;
            self.estado_atual = EstadoTurno::TurnoJogador;
            godot_print!("🎯 Turno {} - Vez do JOGADOR", self.numero_turno);
        }
    }

    /// Processa o ataque do jogador
    /// Retorna true se foi um acerto que afundou um navio
    pub fn processar_ataque_jogador(&mut self, acertou: bool, afundou_navio: bool) -> bool {
        if self.estado_atual != EstadoTurno::TurnoJogador {
            godot_print!("⚠️ Não é turno do jogador!");
            return false;
        }

        if acertou {
            godot_print!("💥 ACERTOU!");
            
            if afundou_navio {
                self.navios_ia -= 1;
                godot_print!("⚓ Navio inimigo AFUNDADO! Restam {} navios da IA", self.navios_ia);
                
                // Verifica vitória
                if self.navios_ia == 0 {
                    self.estado_atual = EstadoTurno::VitoriaJogador;
                    godot_print!("🏆 VITÓRIA DO JOGADOR!");
                    return true;
                }
            }
        } else {
            godot_print!("💧 Água! Errou o tiro.");
        }

        // Passa para o turno da IA
        self.avancar_para_turno_ia();
        true
    }

    /// Processa o ataque da IA
    /// Retorna true se foi um acerto que afundou um navio
    pub fn processar_ataque_ia(&mut self, acertou: bool, afundou_navio: bool) -> bool {
        if self.estado_atual != EstadoTurno::TurnoIA {
            godot_print!("⚠️ Não é turno da IA!");
            return false;
        }

        if acertou {
            godot_print!("💥 IA ACERTOU!");
            
            if afundou_navio {
                self.navios_jogador -= 1;
                godot_print!("⚓ Seu navio foi AFUNDADO! Restam {} navios seus", self.navios_jogador);
                
                // Verifica vitória da IA
                if self.navios_jogador == 0 {
                    self.estado_atual = EstadoTurno::VitoriaIA;
                    godot_print!("💀 DERROTA! A IA venceu!");
                    return true;
                }
            }
        } else {
            godot_print!("💧 IA errou o tiro.");
        }

        // Passa para o próximo turno do jogador
        self.avancar_para_turno_jogador();
        true
    }

    /// Avança para o turno da IA
    fn avancar_para_turno_ia(&mut self) {
        self.estado_atual = EstadoTurno::TurnoIA;
        godot_print!("🤖 Turno {} - Vez da IA", self.numero_turno);
    }

    /// Avança para o próximo turno do jogador
    fn avancar_para_turno_jogador(&mut self) {
        self.numero_turno += 1;
        self.estado_atual = EstadoTurno::TurnoJogador;
        godot_print!("🎯 Turno {} - Vez do JOGADOR", self.numero_turno);
    }

    /// Verifica se o jogo terminou
    pub fn jogo_terminou(&self) -> bool {
        matches!(self.estado_atual, EstadoTurno::VitoriaJogador | EstadoTurno::VitoriaIA)
    }

    /// Retorna uma mensagem descritiva do estado atual
    pub fn mensagem_estado(&self) -> String {
        match self.estado_atual {
            EstadoTurno::PosicionamentoJogador => "Posicione seus navios".to_string(),
            EstadoTurno::PosicionamentoIA => "Aguarde... IA posicionando navios".to_string(),
            EstadoTurno::TurnoJogador => format!("Turno {} - Sua vez! Escolha onde atacar", self.numero_turno),
            EstadoTurno::TurnoIA => format!("Turno {} - IA está atacando...", self.numero_turno),
            EstadoTurno::VitoriaJogador => "🏆 VOCÊ VENCEU!".to_string(),
            EstadoTurno::VitoriaIA => "💀 VOCÊ PERDEU!".to_string(),
        }
    }
}
