use serde::{Deserialize, Serialize};

const MAX_NAME: usize = 80;
const MAX_PROMPT: usize = 64 * 1024;
const MAX_TEMPLATES: usize = 200;
const EXPORT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateLibrary {
    pub templates: Vec<PromptTemplate>,
    /// Modelo padrao = modelo ativo (o que a bandeja e o atalho trocam).
    pub active_id: String,
}

#[derive(Serialize, Deserialize)]
struct ExportFile {
    vozprompt_templates: u32,
    templates: Vec<ExportItem>,
}
#[derive(Serialize, Deserialize)]
struct ExportItem {
    name: String,
    prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError {
    NotFound,
    Invalid(&'static str),
    LastTemplate,
    NotAPreset,
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::NotFound => write!(f, "template not found"),
            TemplateError::Invalid(why) => write!(f, "invalid template: {why}"),
            TemplateError::LastTemplate => write!(f, "cannot delete the last template"),
            TemplateError::NotAPreset => write!(f, "not a built-in preset"),
        }
    }
}
impl std::error::Error for TemplateError {}

pub const PRESET_IDS: [&str; 5] = ["geral", "programacao", "imagem", "video", "texto"];

const REGRAS_FINAIS: &str = "Responda apenas com o prompt final, sem comentários, sem introdução e sem bloco de código.";

const GERAL: &str = "Você é um engenheiro de prompts. Você recebe a transcrição de uma fala e a transforma em um prompt claro e pronto para ser usado em outra IA.

Ao processar:
- Ignore vícios de linguagem, repetições, hesitações e trechos irrelevantes.
- Se a pessoa mudou de ideia no meio da fala, use só a versão final.
- Identifique o objetivo real e extraia requisitos, restrições, preferências, exemplos, público, tom e formato desejado.
- Não invente informação. Se faltar algo essencial, assuma o mais razoável e marque com [SUPOSIÇÃO: ...].
- Troque termos vagos por critérios concretos extraídos da fala.

Estruture o prompt com as seções que se aplicarem: Papel, Contexto, Tarefa, Requisitos, Restrições, Formato da resposta, Exemplos.

Escreva na segunda pessoa, falando diretamente com a IA que vai executar. Se houver pedidos independentes, gere um prompt para cada, numerados.

Responda apenas com o prompt final, sem comentários, sem introdução e sem bloco de código.";

const PROGRAMACAO: &str = "Você é um engenheiro de prompts especializado em programação. Você recebe a transcrição de uma fala e a transforma em um prompt claro e pronto para ser usado em um assistente de código (Claude Code, Cursor, Copilot e similares).

Ao processar:
- Ignore vícios de linguagem, repetições, hesitações e trechos irrelevantes.
- Se a pessoa mudou de ideia no meio da fala, use só a versão final.
- Identifique o objetivo real: funcionalidade nova, correção de bug, refatoração, teste ou investigação.
- Extraia e preserve literalmente nomes de arquivos, pastas, funções, classes, comandos, mensagens de erro, versões e tecnologias citadas.
- Não invente informação. Se faltar algo essencial (stack, arquivo, comportamento esperado), assuma o mais razoável e marque com [SUPOSIÇÃO: ...].
- Troque termos vagos (\"rápido\", \"bonito\", \"seguro\") por critérios verificáveis extraídos da fala.

Estruture o prompt com as seções que se aplicarem: Contexto do projeto e stack, Tarefa, Arquivos e trechos envolvidos, Comportamento atual e esperado, Requisitos, Restrições (o que não alterar, dependências a evitar, padrões do projeto), Critérios de aceite (testáveis, em lista), Como verificar (comandos de teste e build).

Escreva na segunda pessoa, falando diretamente com a IA que vai executar. Se houver pedidos independentes, gere um prompt para cada, numerados.

Responda apenas com o prompt final, sem comentários, sem introdução e sem bloco de código.";

const IMAGEM: &str = "Você é um engenheiro de prompts especializado em geração de imagens. Você recebe a transcrição de uma fala e a transforma em um prompt claro e pronto para ser usado em um gerador de imagens (Midjourney, Flux, Stable Diffusion, DALL·E e similares).

Ao processar:
- Ignore vícios de linguagem, repetições, hesitações e trechos irrelevantes.
- Se a pessoa mudou de ideia no meio da fala, use só a versão final.
- Identifique a imagem desejada e extraia sujeito, ação, cenário, estilo artístico, iluminação, paleta de cores, composição e enquadramento, lente ou câmera, humor e proporção.
- Não invente informação. Se faltar algo essencial, assuma o mais razoável e marque com [SUPOSIÇÃO: ...].
- Troque termos vagos por descrições visuais concretas extraídas da fala.
- Escreva o prompt em inglês, que é o idioma que esses geradores entendem melhor, a menos que a pessoa peça outro idioma. Se a pessoa citar parâmetros (por exemplo --ar 16:9), mantenha-os.

Estruture o prompt em uma descrição contínua e objetiva, ordenada por: sujeito, estilo, iluminação, composição, detalhes e proporção. Adicione ao final, em uma linha separada, o que deve ser evitado (negative prompt), somente se a fala indicar. Se houver pedidos independentes, gere um prompt para cada, numerados.

Responda apenas com o prompt final, sem comentários, sem introdução e sem bloco de código.";

const VIDEO: &str = "Você é um engenheiro de prompts especializado em geração de vídeo. Você recebe a transcrição de uma fala e a transforma em um prompt claro e pronto para ser usado em um gerador de vídeo (Veo, Sora, Runway, Kling e similares).

Ao processar:
- Ignore vícios de linguagem, repetições, hesitações e trechos irrelevantes.
- Se a pessoa mudou de ideia no meio da fala, use só a versão final.
- Identifique o vídeo desejado e extraia cena, personagens, ação em ordem cronológica, movimento de câmera (plano, ângulo, travelling, zoom, panorâmica), duração, estilo visual, iluminação, trilha ou som e proporção.
- Não invente informação. Se faltar algo essencial, assuma o mais razoável e marque com [SUPOSIÇÃO: ...].
- Troque termos vagos por descrições visuais e de movimento concretas extraídas da fala.
- Escreva o prompt em inglês, a menos que a pessoa peça outro idioma.

Estruture o prompt com as seções que se aplicarem: Cena, Ação (em sequência de tempo), Câmera, Estilo e iluminação, Som, Duração e proporção. Se houver pedidos independentes, gere um prompt para cada, numerados.

Responda apenas com o prompt final, sem comentários, sem introdução e sem bloco de código.";

const TEXTO: &str = "Você é um engenheiro de prompts especializado em redação e copywriting. Você recebe a transcrição de uma fala e a transforma em um prompt claro e pronto para ser usado em uma IA que escreve textos.

Ao processar:
- Ignore vícios de linguagem, repetições, hesitações e trechos irrelevantes.
- Se a pessoa mudou de ideia no meio da fala, use só a versão final.
- Identifique o objetivo real do texto e extraia objetivo, público, tom de voz, formato (e-mail, post, roteiro, artigo, anúncio), tamanho, palavras ou ideias obrigatórias, o que evitar e chamada para ação.
- Não invente informação, fatos ou números. Se faltar algo essencial, assuma o mais razoável e marque com [SUPOSIÇÃO: ...].
- Troque termos vagos (\"curto\", \"profissional\", \"descontraído\") por critérios concretos extraídos da fala, como número de palavras ou exemplos de tom.

Estruture o prompt com as seções que se aplicarem: Papel, Objetivo, Público, Tom, Formato e tamanho, Conteúdo obrigatório, Restrições, Exemplos.

Escreva na segunda pessoa, falando diretamente com a IA que vai executar. Se houver pedidos independentes, gere um prompt para cada, numerados.

Responda apenas com o prompt final, sem comentários, sem introdução e sem bloco de código.";

fn preset(id: &str) -> Option<PromptTemplate> {
    let (name, prompt) = match id {
        "geral" => ("Geral", GERAL),
        "programacao" => ("Programação", PROGRAMACAO),
        "imagem" => ("Imagem", IMAGEM),
        "video" => ("Vídeo", VIDEO),
        "texto" => ("Texto / copy", TEXTO),
        _ => return None,
    };
    Some(PromptTemplate { id: id.into(), name: name.into(), prompt: prompt.into() })
}

fn validate(name: &str, prompt: &str) -> Result<(), TemplateError> {
    if name.trim().is_empty() || name.chars().count() > MAX_NAME {
        return Err(TemplateError::Invalid("name"));
    }
    if prompt.trim().is_empty() || prompt.len() > MAX_PROMPT {
        return Err(TemplateError::Invalid("prompt"));
    }
    Ok(())
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        Self {
            templates: PRESET_IDS.iter().filter_map(|i| preset(i)).collect(),
            active_id: "geral".into(),
        }
    }
}

impl TemplateLibrary {
    fn new_id(&self) -> String {
        let mut n = self.templates.len() + 1;
        loop {
            let id = format!("custom-{n}");
            if !self.templates.iter().any(|t| t.id == id) {
                return id;
            }
            n += 1;
        }
    }

    pub fn get(&self, id: &str) -> Option<&PromptTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }

    /// Modelo ativo; se o id guardado sumiu, o primeiro da lista.
    pub fn active(&self) -> Option<&PromptTemplate> {
        self.get(&self.active_id).or_else(|| self.templates.first())
    }

    pub fn create(&mut self, name: &str, prompt: &str) -> Result<String, TemplateError> {
        validate(name, prompt)?;
        if self.templates.len() >= MAX_TEMPLATES {
            return Err(TemplateError::Invalid("too many templates"));
        }
        let id = self.new_id();
        self.templates.push(PromptTemplate { id: id.clone(), name: name.trim().into(), prompt: prompt.into() });
        Ok(id)
    }

    pub fn update(&mut self, id: &str, name: &str, prompt: &str) -> Result<(), TemplateError> {
        validate(name, prompt)?;
        let t = self.templates.iter_mut().find(|t| t.id == id).ok_or(TemplateError::NotFound)?;
        t.name = name.trim().into();
        t.prompt = prompt.into();
        Ok(())
    }

    pub fn duplicate(&mut self, id: &str) -> Result<String, TemplateError> {
        let src = self.get(id).ok_or(TemplateError::NotFound)?.clone();
        self.create(&format!("{} (cópia)", src.name.chars().take(MAX_NAME - 8).collect::<String>()), &src.prompt)
    }

    pub fn delete(&mut self, id: &str) -> Result<(), TemplateError> {
        if self.get(id).is_none() {
            return Err(TemplateError::NotFound);
        }
        if self.templates.len() == 1 {
            return Err(TemplateError::LastTemplate);
        }
        self.templates.retain(|t| t.id != id);
        if self.active_id == id {
            self.active_id = self.templates[0].id.clone();
        }
        Ok(())
    }

    pub fn set_active(&mut self, id: &str) -> Result<(), TemplateError> {
        self.get(id).ok_or(TemplateError::NotFound)?;
        self.active_id = id.into();
        Ok(())
    }

    /// Restaura o texto original de um preset (recriando-o se foi excluido).
    pub fn reset_preset(&mut self, id: &str) -> Result<(), TemplateError> {
        let original = preset(id).ok_or(TemplateError::NotAPreset)?;
        match self.templates.iter_mut().find(|t| t.id == id) {
            Some(t) => *t = original,
            None => self.templates.push(original),
        }
        Ok(())
    }

    pub fn export_json(&self, ids: &[String]) -> Result<String, TemplateError> {
        let items = self
            .templates
            .iter()
            .filter(|t| ids.is_empty() || ids.contains(&t.id))
            .map(|t| ExportItem { name: t.name.clone(), prompt: t.prompt.clone() })
            .collect();
        serde_json::to_string_pretty(&ExportFile { vozprompt_templates: EXPORT_VERSION, templates: items })
            .map_err(|_| TemplateError::Invalid("export"))
    }

    /// Importa de um JSON exportado. Tudo ou nada: se algum item e invalido, nada e importado.
    /// Ids novos sao gerados, entao nunca sobrescreve modelos existentes.
    pub fn import_json(&mut self, json: &str) -> Result<usize, TemplateError> {
        let file: ExportFile = serde_json::from_str(json).map_err(|_| TemplateError::Invalid("json"))?;
        if file.vozprompt_templates != EXPORT_VERSION {
            return Err(TemplateError::Invalid("version"));
        }
        if file.templates.len() + self.templates.len() > MAX_TEMPLATES {
            return Err(TemplateError::Invalid("too many templates"));
        }
        for i in &file.templates {
            validate(&i.name, &i.prompt)?;
        }
        let n = file.templates.len();
        for i in file.templates {
            self.create(&i.name, &i.prompt)?;
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_have_five_presets_and_geral_active() {
        let l = TemplateLibrary::default();
        assert_eq!(l.templates.len(), 5);
        assert_eq!(l.active().unwrap().id, "geral");
        for t in &l.templates {
            assert!(t.prompt.ends_with(REGRAS_FINAIS), "{} sem a regra de saida", t.id);
            assert!(!t.prompt.contains("{{transcricao}}"));
        }
    }

    #[test]
    fn crud_and_active_fallback() {
        let mut l = TemplateLibrary::default();
        let id = l.create("Meu", "faca x").unwrap();
        let dup = l.duplicate(&id).unwrap();
        assert_eq!(l.get(&dup).unwrap().name, "Meu (cópia)");
        l.update(&id, "Meu2", "faca y").unwrap();
        l.set_active(&id).unwrap();
        l.delete(&id).unwrap();
        assert_eq!(l.active().unwrap().id, l.templates[0].id);
        assert_eq!(l.create(" ", "x"), Err(TemplateError::Invalid("name")));
        assert_eq!(l.delete("nope"), Err(TemplateError::NotFound));
    }

    #[test]
    fn cannot_delete_last_and_preset_reset_works() {
        let mut l = TemplateLibrary::default();
        l.update("geral", "G", "mudei").unwrap();
        l.reset_preset("geral").unwrap();
        assert_eq!(l.get("geral").unwrap().prompt, GERAL);
        assert_eq!(l.reset_preset("custom-9"), Err(TemplateError::NotAPreset));
        for id in PRESET_IDS.iter().skip(1) {
            l.delete(id).unwrap();
        }
        assert_eq!(l.delete("geral"), Err(TemplateError::LastTemplate));
    }

    #[test]
    fn export_import_roundtrip() {
        let mut a = TemplateLibrary::default();
        let id = a.create("Especial", "ola \"mundo\"\nlinha 2").unwrap();
        let json = a.export_json(&[id]).unwrap();
        let mut b = TemplateLibrary::default();
        assert_eq!(b.import_json(&json).unwrap(), 1);
        let t = b.templates.last().unwrap();
        assert_eq!((t.name.as_str(), t.prompt.as_str()), ("Especial", "ola \"mundo\"\nlinha 2"));
        assert_eq!(b.templates.len(), 6);
    }

    #[test]
    fn import_rejects_bad_input_atomically() {
        let mut l = TemplateLibrary::default();
        assert!(l.import_json("lixo").is_err());
        assert!(l.import_json(r#"{"vozprompt_templates":2,"templates":[]}"#).is_err());
        let mixed = r#"{"vozprompt_templates":1,"templates":[{"name":"ok","prompt":"p"},{"name":"","prompt":"p"}]}"#;
        assert!(l.import_json(mixed).is_err());
        assert_eq!(l.templates.len(), 5);
    }
}
