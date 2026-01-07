# Guia de Contribuição

Obrigado por considerar contribuir para o VMS Enterprise! Este documento fornece diretrizes para contribuir com o projeto.

## 🚀 Começando

1. **Fork o repositório** e clone localmente
2. **Configure o ambiente** executando `./scripts/setup-dev.sh` (Linux/Mac) ou `.\scripts\setup-dev.ps1` (Windows)
3. **Crie um branch** para sua feature/fix: `git checkout -b feature/minha-feature`
4. **Faça suas alterações** seguindo as diretrizes abaixo
5. **Execute os testes** e certifique-se de que passam
6. **Faça commit** das suas mudanças
7. **Envie um Pull Request**

## 📋 Diretrizes de Código

### Estilo de Código

Seguimos as convenções padrão do Rust:

```bash
# Formatação
cargo fmt

# Linting
cargo clippy --all-targets -- -D warnings

# Testes
cargo test --all
```

### Commits

Use mensagens de commit descritivas seguindo o padrão:

```
tipo(escopo): descrição curta

Descrição mais detalhada se necessário.

Closes #123
```

Tipos comuns:
- `feat`: Nova funcionalidade
- `fix`: Correção de bug
- `docs`: Documentação
- `style`: Formatação, missing semicolons, etc
- `refactor`: Refatoração de código
- `test`: Adição de testes
- `chore`: Manutenção

### Documentação

- **Documente funções públicas** com `///`
- **Use exemplos** em docstrings quando apropriado
- **Atualize o README** se adicionar features significativas
- **Adicione ADRs** (Architecture Decision Records) para decisões arquiteturais importantes

```rust
/// Cria um novo pipeline de ingestão
///
/// # Arguments
///
/// * `config` - Configuração da câmera
///
/// # Examples
///
/// ```
/// let config = CameraConfig::new("Camera 1".to_string(), "rtsp://...".to_string());
/// let pipeline = IngestPipeline::new(config)?;
/// ```
///
/// # Errors
///
/// Retorna erro se o GStreamer não conseguir criar os elementos
pub fn new(config: CameraConfig) -> Result<Self> {
    // ...
}
```

## 🧪 Testes

### Tipos de Testes

- **Unit Tests**: Testam componentes isolados
- **Integration Tests**: Testam interação entre componentes
- **Benchmarks**: Medem performance

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_config_creation() {
        let config = CameraConfig::new("Test".to_string(), "rtsp://test".to_string());
        assert_eq!(config.name, "Test");
    }
}
```

### Executando Testes

```bash
# Todos os testes
cargo test --all

# Testes de um pacote específico
cargo test -p vms-common

# Testes de integração
cargo test --features integration

# Com output
cargo test -- --nocapture

# Benchmarks
cargo bench
```

## 🔒 Segurança

- **Nunca commite** credenciais ou secrets
- **Use ferramentas** de análise: `cargo audit`, `cargo deny`
- **Reporte vulnerabilidades** via security@example.com (não em issues públicas)

## 📊 Performance

- **Meça antes de otimizar** - use benchmarks
- **Evite alocações** desnecessárias em hot paths
- **Use zero-copy** quando possível
- **Profile com** `cargo flamegraph` ou `perf`

## 🐛 Reportando Bugs

Use o template de issue e inclua:

- **Descrição clara** do bug
- **Passos para reproduzir**
- **Comportamento esperado** vs atual
- **Ambiente** (OS, versão do Rust, etc)
- **Logs relevantes**

## ✨ Sugerindo Features

- **Verifique** se já não existe uma issue
- **Descreva o caso de uso**
- **Explique o benefício**
- **Considere alternativas**

## 🎯 Áreas que Precisam de Ajuda

- [ ] Testes de integração com câmeras reais
- [ ] Documentação de APIs
- [ ] Dashboards do Grafana
- [ ] Suporte a novos codecs
- [ ] Otimizações de performance
- [ ] Integrações com sistemas externos

## 📞 Contato

- **Issues**: Para bugs e features
- **Discussions**: Para perguntas e ideias
- **Email**: dev@example.com

## 📜 Código de Conduta

Seja respeitoso e inclusivo. Veja CODE_OF_CONDUCT.md para detalhes.

---

Obrigado por contribuir! 🙏
