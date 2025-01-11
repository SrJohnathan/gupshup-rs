# gupshup-rs

`gupshup-rs` é uma biblioteca Rust projetada para desserializar dados recebidos dos webhooks da Gupshup relacionados ao WhatsApp. Ela fornece suporte estruturado para processar eventos e mensagens de forma eficiente.

## Recursos

- Desserializa payloads do webhook da Gupshup com facilidade.
- Suporte a múltiplos tipos de mensagens, incluindo texto, imagem, áudio, vídeo e muito mais.
- Integração com eventos do WhatsApp, como mensagens entregues, lidas, enviadas, etc.

## Instalação

Adicione `gupshup-rs` ao seu arquivo `Cargo.toml`:

```toml
[dependencies]
gupshup-rs = "0.0.1"
```
git

```toml
[dependencies]
gupshup-rs = { git = "https://github.com/SrJohnathan/gupshup-rs.git"}
```


## Exemplos de Uso

### Webhook Handler

Aqui está um exemplo de como usar a biblioteca para processar mensagens diretamente em uma função assíncrona, como parte de um servidor:

```rust
pub async fn web_hook(task: serde_json::Value) {
    match gupshup_rs::deserialize(&task) {
        MessageType::Enqueued(x) => {
            println!("Mensagem enfileirada: {:?}", x);
        }
        MessageType::Failed(x) => {
            println!("Mensagem falhou: {:?}", x);
        }
        MessageType::Sent(x) => {
            println!("Mensagem enviada: {:?}", x);
        }
        MessageType::Delivered(x) => {
            println!("Mensagem entregue: {:?}", x);
        }
        MessageType::Read(x) => {
            println!("Mensagem lida: {:?}", x);
        }
        MessageType::Text(x) => {
            println!("Mensagem de texto: {:?}", x);
        }
        MessageType::Image(x) => {
            println!("Mensagem de imagem: {:?}", x);
        }
        MessageType::File(x) => {
            println!("Mensagem de arquivo: {:?}", x);
        }
        MessageType::Audio(x) => {
            println!("Mensagem de áudio: {:?}", x);
        }
        MessageType::Video(x) => {
            println!("Mensagem de vídeo: {:?}", x);
        }
        MessageType::Location(x) => {
            println!("Mensagem de localização: {:?}", x);
        }
        MessageType::QuickReply(x) => {
            println!("Resposta rápida: {:?}", x);
        }
        MessageType::ButtonReply(x) => {
            println!("Resposta de botão: {:?}", x);
        }
        MessageType::ListReply(x) => {
            println!("Resposta de lista: {:?}", x);
        }
        MessageType::Unknown => {
            println!("Tipo de mensagem desconhecido");
        }
    }
}
```

### Recursos Suportados

#### Tipos de Mensagem
- Texto (`text`)
- Imagem (`image`)
- Áudio (`audio`)
- Vídeo (`video`)
- Localização (`location`)
- Resposta rápida (`quick_reply`)
- Resposta a botões (`button_reply`)
- Resposta a listas (`list_reply`)

#### Eventos do WhatsApp
- Mensagem enviada (`sent`)
- Mensagem entregue (`delivered`)
- Mensagem lida (`read`)
- Mensagem na fila (`enqueued`)
- Mensagem com falha (`failed`)

## Contribuições

Contribuições são bem-vindas! Siga os passos abaixo para colaborar:

1. Fork o repositório.
2. Crie uma branch para sua funcionalidade/correção: `git checkout -b minha-feature`.
3. Faça suas alterações e commite: `git commit -m 'Adiciona minha funcionalidade'`.
4. Envie sua branch: `git push origin minha-feature`.
5. Abra um Pull Request.

## Licença

Este projeto está licenciado sob as licenças MIT e Apache 2.0. Consulte os arquivos `LICENSE-MIT` e `LICENSE-APACHE` para mais detalhes.

