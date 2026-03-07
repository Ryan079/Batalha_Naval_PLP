use crate::domain::jogador::Jogador;
use crate::domain::tabuleiro::FROTA_PADRAO;

pub struct FasePosicionamento {
    fila_navios: Vec<(String, usize)>,
    indice_atual: usize,
    orientacao_horizontal: bool,
}

impl FasePosicionamento {
    pub fn nova() -> Self {
        let mut fila_navios = Vec::new();
        for config in FROTA_PADRAO.iter() {
            for _ in 0..config.quantidade {
                fila_navios.push((config.nome.to_string(), config.tamanho));
            }
        }

        Self {
            fila_navios,
            indice_atual: 0,
            orientacao_horizontal: true,
        }
    }

    pub fn alternar_orientacao(&mut self) {
        self.orientacao_horizontal = !self.orientacao_horizontal;
    }

    pub fn orientacao_texto(&self) -> &'static str {
        if self.orientacao_horizontal {
            "Horizontal"
        } else {
            "Vertical"
        }
    }

    pub fn navio_atual(&self) -> Option<(&str, usize)> {
        self.fila_navios
            .get(self.indice_atual)
            .map(|(nome, tamanho)| (nome.as_str(), *tamanho))
    }

    pub fn tentar_posicionar_navio(
        &mut self,
        jogador: &mut Jogador,
        x: usize,
        y: usize,
    ) -> Result<bool, String> {
        let Some((nome, tamanho)) = self.fila_navios.get(self.indice_atual).cloned() else {
            return Ok(true);
        };

        jogador
            .tabuleiro_mut()
            .posicionar_navio(&nome, x, y, tamanho, self.orientacao_horizontal)?;

        self.indice_atual += 1;
        Ok(self.terminou())
    }

    pub fn terminou(&self) -> bool {
        self.indice_atual >= self.fila_navios.len()
    }
}
