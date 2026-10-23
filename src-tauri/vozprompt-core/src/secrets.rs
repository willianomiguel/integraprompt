use std::collections::HashMap;
use std::sync::Mutex;

const SERVICE: &str = "com.integracore.vozprompt";

/// Cofre de segredos (API keys). Implementacao real: cofre do sistema.
pub trait SecretStore: Send + Sync {
    fn get(&self, name: &str) -> Option<String>;
    fn set(&self, name: &str, value: &str) -> Result<(), String>;
    fn delete(&self, name: &str) -> Result<(), String>;
}

/// Credential Manager (Windows), Keychain (macOS), Secret Service (Linux).
pub struct KeyringStore;

impl SecretStore for KeyringStore {
    fn get(&self, name: &str) -> Option<String> {
        keyring::Entry::new(SERVICE, name).ok()?.get_password().ok()
    }
    fn set(&self, name: &str, value: &str) -> Result<(), String> {
        keyring::Entry::new(SERVICE, name)
            .and_then(|e| e.set_password(value))
            .map_err(|e| e.to_string())
    }
    fn delete(&self, name: &str) -> Result<(), String> {
        match keyring::Entry::new(SERVICE, name).and_then(|e| e.delete_credential()) {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

/// Para testes.
#[derive(Default)]
pub struct MemoryStore(Mutex<HashMap<String, String>>);

impl SecretStore for MemoryStore {
    fn get(&self, name: &str) -> Option<String> {
        self.0.lock().unwrap().get(name).cloned()
    }
    fn set(&self, name: &str, value: &str) -> Result<(), String> {
        self.0.lock().unwrap().insert(name.into(), value.into());
        Ok(())
    }
    fn delete(&self, name: &str) -> Result<(), String> {
        self.0.lock().unwrap().remove(name);
        Ok(())
    }
}

/// Move as chaves em texto puro de `keys` para o cofre e as esvazia (vazio = removida).
/// Chaves que o cofre recusar continuam em `keys` (nao perde a chave do usuario).
/// Devolve true se `keys` mudou (precisa regravar o arquivo).
pub fn vault_keys(store: &dyn SecretStore, keys: &mut HashMap<String, String>) -> bool {
    let mut changed = false;
    for (provider, value) in keys.iter_mut() {
        if value.is_empty() {
            continue;
        }
        if store.set(provider, value).is_ok() {
            value.clear();
            changed = true;
        }
    }
    changed
}

/// Preenche chaves vazias a partir do cofre (para o resto do app seguir lendo `keys`).
pub fn hydrate_keys(store: &dyn SecretStore, keys: &mut HashMap<String, String>) {
    for (provider, value) in keys.iter_mut() {
        if value.is_empty() {
            if let Some(v) = store.get(provider) {
                *value = v;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
    }

    #[test]
    fn vaulting_empties_plaintext_and_hydrate_restores() {
        let s = MemoryStore::default();
        let mut k = keys(&[("openai", "sk-1"), ("groq", "")]);
        assert!(vault_keys(&s, &mut k));
        assert_eq!(k["openai"], "");
        assert_eq!(s.get("openai").as_deref(), Some("sk-1"));
        hydrate_keys(&s, &mut k);
        assert_eq!(k["openai"], "sk-1");
        assert_eq!(k["groq"], "");
        assert!(!vault_keys(&s, &mut keys(&[("a", "")])));
    }

    #[test]
    fn failing_vault_keeps_the_key() {
        struct Broken;
        impl SecretStore for Broken {
            fn get(&self, _: &str) -> Option<String> { None }
            fn set(&self, _: &str, _: &str) -> Result<(), String> { Err("no".into()) }
            fn delete(&self, _: &str) -> Result<(), String> { Ok(()) }
        }
        let mut k = keys(&[("openai", "sk-1")]);
        assert!(!vault_keys(&Broken, &mut k));
        assert_eq!(k["openai"], "sk-1");
    }
}
