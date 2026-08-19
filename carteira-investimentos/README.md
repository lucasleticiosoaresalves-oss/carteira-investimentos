 Carteira de Investimentos — Fullstack em Rust

Aplicação fullstack para cadastrar, acompanhar e editar ativos de uma
carteira de investimentos, com autenticação de usuários e páginas
web renderizadas no servidor.

O que o projeto faz?

- Permite criar uma conta e fazer login (sessão via cookie + JWT);
- Cadastra, lista, edita e remove ativos de investimento (nome,
  ticker, categoria, quantidade e preço unitário);
- Calcula automaticamente o **valor total da carteira** (soma de
  `quantidade × preço unitário` de todos os ativos) e exibe no
  dashboard, junto com a quantidade de ativos cadastrados;
- Protege as rotas de dashboard e ativos, exigindo autenticação.

Tecnologias usadas

- Rust + Axum — servidor web e roteamento
- SQLx + PostgreSQL — persistência e migrations
- JWT (`jsonwebtoken`) + cookies HTTP-only — autenticação
- Argon2 — hash de senhas
- Askama — templates HTML renderizados no servidor
- Docker Compose — banco de dados local

Como executar a aplicação?

Pré-requisitos: Rust (`rustup`), Docker e Docker Compose.

```bash
# 1. Subir o banco de dados
docker compose up -d

# 2. Rodar a aplicação (as migrations rodam automaticamente ao subir)
cargo run
```

A aplicação sobe em `http://localhost:3000`. As variáveis de
ambiente (`DATABASE_URL`, `JWT_SECRET`, `APP_PORT`) ficam no `.env`.

Melhoria implementada:

Adicionei o cálculo do valor total da carteira no dashboard:
cada ativo já mostra seu próprio subtotal (`quantidade × preço`), e
o topo da página exibe a soma de todos os ativos do usuário logado,
além do número total de ativos cadastrados. O cálculo é feito em
Rust, iterando sobre os ativos já carregados do banco
(`assets.iter().map(|a| a.total_value()).sum()`), e existe também
uma versão alternativa que soma direto no banco via `SUM(quantity *
unit_price)`, disponível em `db::portfolio_total_value`.

Também adicionei validações simples nos formulários (nome/ticker
obrigatórios, quantidade e preço não podem ser negativos) com
mensagens de erro exibidas na própria página.

Como testar?

1. Acesse `/register` e crie uma conta;
2. Faça login em `/login`;
3. No dashboard, cadastre alguns ativos em `/assets/new` com
   diferentes quantidades e preços;
4. Confira se o "Valor total da carteira" no topo do dashboard
   corresponde à soma manual de `quantidade × preço` de cada ativo;
5. Edite um ativo e confirme que o total é recalculado;
6. Exclua um ativo e confirme que ele some da lista e o total é
   atualizado.

O que aprendi durante o desafio:

- Como estruturar uma aplicação Axum em módulos (config, banco,
  autenticação, extractors, handlers e rotas) em vez de deixar tudo
  em `main.rs`;
- Como implementar um extractor customizado (`AuthUser`) para
  proteger rotas lendo o JWT de um cookie;
- Como usar `SQLx` com `query_as` e migrations versionadas;
- Diferença entre calcular um agregado (valor total) em Rust versus
  delegar para o banco com `SUM(...)`, e quando cada abordagem faz
  mais sentido;
- Como usar `Askama` para renderizar HTML de forma tipada,
  reaproveitando um template base com blocos.
