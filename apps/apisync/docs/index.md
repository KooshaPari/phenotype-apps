---
layout: home
title: Apisync - Universal API Toolkit
titleTemplate: false
---

# Apisync

Universal API toolkit with REST, GraphQL, and WebSocket support.

## Overview

`Apisync` is a Rust API toolkit providing REST, GraphQL, and WebSocket support. It is the foundation library for building Phenotype HTTP services.

## Features

- **REST Router**: Type-safe routing and handler abstractions
- **GraphQL**: Schema and resolver helpers
- **WebSocket**: Connection management
- **Middleware**: Auth, logging, CORS as tower layers
- **Async-first**: Built on tokio and hyper

## Architecture

```
src/
├── rest.rs       # REST router and handler abstractions
├── graphql.rs     # GraphQL schema and resolver helpers
├── ws.rs         # WebSocket connection management
├── middleware.rs  # Auth, logging, CORS middleware
└── lib.rs        # Crate root
```

## Quick Start

```rust
use apikit::{rest::Router, middleware::Logger};

let router = Router::new()
    .layer(Logger::new())
    .route("/api/users", get_users);
```

## Links

- [Repository](https://github.com/KooshaPari/apikit)
