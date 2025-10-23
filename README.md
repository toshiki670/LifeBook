# LifeBook 📚

A desktop application demonstrating the integration of **Tauri**, **SeaORM**, and **GraphQL**.

## 🚀 Features

- **Tauri**: Lightweight desktop application framework
- **SeaORM**: Async ORM for Rust with SQLite backend
- **async-graphql**: GraphQL server implementation in Rust
- **React Router v7**: Modern React routing
- **TypeScript**: Type-safe frontend development
- **Tailwind CSS**: Utility-first CSS framework

## 📦 Tech Stack

### Backend (Rust)

- Tauri 2.x
- SeaORM 1.x with SQLite
- async-graphql 7.x
- tokio (async runtime)

### Frontend (TypeScript)

- React 19
- React Router 7
- TypeScript 5.9
- Tailwind CSS 4

## 🎯 What This Demonstrates

This project showcases how to build a desktop application that:

1. **Full DDD Architecture**: Implements Domain-Driven Design with 4 layers (Domain, Application, Infrastructure, Presentation)
2. **Bounded Contexts as Crates**: Each bounded context is an independent Rust crate with clear boundaries
3. **GraphQL API**: Uses GraphQL as the API layer between frontend and backend
4. **SeaORM**: Leverages SeaORM for database operations with repository pattern
5. **Type Safety**: Maintains type safety across the entire stack
6. **Clean Architecture**: Clear separation of concerns with dependency inversion

### 📦 Crate Architecture

The backend is organized into multiple Rust crates:

- **`library`**: Library management bounded context (independent crate)
  - Contains: Domain, Application, Infrastructure, and Presentation layers
  - Provides: `BookQuery`, `BookMutation`, `BookService`, `BookRepository`
- **`shared`**: Common utilities shared across all contexts
  - Contains: Common error types, domain/application utilities
- **`entity`**: SeaORM entities (shared DB layer)
  - Contains: Database models and relations
- **`migration`**: Database migrations
- **`lifebook`**: Main Tauri application
  - Integrates all contexts and provides the Tauri runtime

See [ARCHITECTURE.md](./doc/architecture/ARCHITECTURE.md), [GRAPHQL_GUIDE.md](./doc/development/GRAPHQL_GUIDE.md), and [CODING_GUIDELINES.md](./doc/development/CODING_GUIDELINES.md) for detailed documentation.

## 🛠️ Development

### Prerequisites

- Node.js (v18+)
- pnpm
- Rust (latest stable)

### Install Dependencies

```bash
pnpm install
```

### Run Development Server

```bash
pnpm tauri dev
```

### Build for Production

```bash
pnpm tauri build
```

## 📖 Project Structure

```
LifeBook/
├── src/                          # Frontend source
│   ├── lib/
│   │   └── graphql.ts           # GraphQL client utilities
│   ├── routes/
│   │   ├── home.tsx             # Home page
│   │   └── books.tsx            # Books management (GraphQL demo)
│   └── root.tsx                 # Root component
├── src-tauri/                   # Rust backend (DDD with Bounded Contexts)
│   ├── contexts/                # Bounded Contexts (independent Crates)
│   │   ├── library/             # Library context (Domain, Application, Infrastructure, Presentation)
│   │   └── shared/              # Shared utilities across contexts
│   ├── entity/                  # SeaORM entities (shared DB layer)
│   ├── migration/               # Database migrations
│   ├── lifebook/                # Main Tauri application
│   │   └── src/
│   │       ├── graphql_schema.rs # GraphQL schema integration
│   │       ├── app_state.rs     # Dependency injection container
│   │       ├── database.rs      # Database connection
│   │       ├── lib.rs           # Entry point
│   │       └── main.rs          # Binary entry
│   └── Cargo.toml               # Workspace definition
└── doc/                         # Documentation
    ├── architecture/            # Architecture documentation
    ├── development/             # Development guides and coding standards
    └── plan/                    # Plan documents (Cursor Plan mode)
```

## 🎓 Learning Resources

For a comprehensive guide on the GraphQL integration, refer to:

- [GRAPHQL_GUIDE.md](./doc/development/GRAPHQL_GUIDE.md) - Complete guide with examples
- [CODING_GUIDELINES.md](./doc/development/CODING_GUIDELINES.md) - Project coding guidelines and best practices
- [QUICKSTART.md](./doc/development/QUICKSTART.md) - Quick start guide

## 📝 Example GraphQL Operations

### Query: Get all books

```graphql
query {
  books {
    id
    title
    author
    description
    publishedYear
  }
}
```

### Mutation: Create a book

```graphql
mutation {
  createBook(
    title: "The Rust Programming Language"
    author: "Steve Klabnik"
    publishedYear: 2018
  ) {
    id
    title
  }
}
```

## 🔧 Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 📄 License

This project is open source and available for educational purposes.
