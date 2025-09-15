use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MiniDb {
    pub database: HashMap<String, String>,
    pub length: usize,
}

impl MiniDb {
    pub fn new() -> Self {
        MiniDb {
            database: HashMap::new(),
            length: 0,
        }
    }

    fn cpf_validation(&mut self, entry: String) -> bool {
        let cleaned: String = entry.trim().replace(".", "").replace("-", "");

        if cleaned.len() != 11 {
            println!("O cpf deve conter 11 digitos");
        }

        let digits: Vec<u32> = cleaned
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut sum = 0;
        for i in 0..9 {
            sum += digits[i] * (10 - i as u32);
        }
        let resto = sum % 11;
        let first_verifier = if resto < 2 { 0 } else { 11 - resto };

        sum = 0;
        for i in 0..10 {
            sum += digits[i] * (11 - i as u32);
        }
        let resto2 = sum % 11;
        let second_verifier = if resto2 < 2 { 0 } else { 11 - resto2 };

        if first_verifier == digits[9] && second_verifier == digits[10] {
            println!("CPF válido");
            true
        } else {
            println!("CPF inválido");
            false
        }
    }

    fn format_cpf(&self,cpf: &str) -> String {
        format!(
            "{}.{}.{}-{}",
            &cpf[0..3],
            &cpf[3..6],
            &cpf[6..9],
            &cpf[9..11]
        )
    }

    pub fn add(&mut self, entry: String) {
        let entrys: Vec<&str> = entry.split_whitespace().collect();

        if entrys.len() == 2 {
            if entrys[0].contains("cpf") {
                let valido: bool = self.cpf_validation(entrys[1].to_string());
                if !valido {
                    println!("Cpf Inválido");
                    return;
                }

                let formatted_cpf = self.format_cpf(&entrys[1].replace(".", "").replace("-", ""));
                self.database.insert(entrys[0].to_string(), formatted_cpf);
                self.length += 1;
                return;
            }

            self.database.insert(entrys[0].to_string(), entrys[1].to_string());
            self.length += 1;
        } else {
            println!("Erro ao cadastrar informação: '{}'", entry);
            println!("A formatação correta para a inserção --> (Chave Valor)")
        }
    }

    pub fn get(&self, key: &str) -> String {
        match self.database.get(key) {
            Some(value) => value.to_string(),
            None => "Esta chave não está presente no banco".to_string(),
        }
    }
}
