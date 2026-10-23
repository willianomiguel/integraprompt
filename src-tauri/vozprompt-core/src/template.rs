/// Variavel substituida pela transcricao dentro do modelo de prompt.
pub const TRANSCRIPT_VAR: &str = "{{transcricao}}";

/// Mensagens prontas para enviar ao provedor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Messages {
    pub system: Option<String>,
    pub user: String,
}

/// Com a variavel no modelo: o modelo inteiro (com a transcricao no lugar) vira a mensagem do usuario.
/// Sem a variavel: o modelo e o system prompt e a transcricao e a mensagem do usuario.
/// A substituicao e feita em uma passada, entao uma transcricao que contenha a variavel nao se expande.
pub fn build_messages(template: &str, transcript: &str) -> Messages {
    if template.contains(TRANSCRIPT_VAR) {
        Messages {
            system: None,
            user: template.replace(TRANSCRIPT_VAR, transcript),
        }
    } else {
        Messages {
            system: Some(template.trim().to_owned()),
            user: transcript.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_variable_uses_system_and_user() {
        let m = build_messages("  Seja util.  ", "oi tudo bem");
        assert_eq!(m.system.as_deref(), Some("Seja util."));
        assert_eq!(m.user, "oi tudo bem");
    }

    #[test]
    fn with_variable_replaces_all_occurrences_in_user_message() {
        let m = build_messages("A: {{transcricao}}\nB: {{transcricao}}", "ola");
        assert_eq!(m.system, None);
        assert_eq!(m.user, "A: ola\nB: ola");
    }

    #[test]
    fn transcript_containing_variable_is_not_expanded_again() {
        let m = build_messages("X {{transcricao}}", "{{transcricao}}");
        assert_eq!(m.user, "X {{transcricao}}");
    }
}
