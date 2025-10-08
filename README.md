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
2. **GraphQL API**: Uses GraphQL as the API layer between frontend and backend
3. **SeaORM**: Leverages SeaORM for database operations with repository pattern
4. **Type Safety**: Maintains type safety across the entire stack
5. **Clean Architecture**: Clear separation of concerns with dependency inversion

See [GRAPHQL_GUIDE.md](./GRAPHQL_GUIDE.md) and [CODING_GUIDELINES.md](./CODING_GUIDELINES.md) for detailed documentation.

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
├── src/                    # Frontend source
│   ├── lib/
│   │   └── graphql.ts     # GraphQL client utilities
│   ├── routes/
│   │   ├── home.tsx       # Home page
│   │   └── books.tsx      # Books management (GraphQL demo)
│   └── root.tsx           # Root component
├── src-tauri/             # Rust backend (DDD Architecture)
│   ├── src/
│   │   ├── domain/        # Domain Layer
│   │   │   ├── entities.rs
│   │   │   ├── repositories.rs
│   │   │   └── errors.rs
│   │   ├── application/   # Application Layer
│   │   │   ├── services.rs
│   │   │   ├── dto.rs
│   │   │   └── errors.rs
│   │   ├── infrastructure/ # Infrastructure Layer
│   │   │   ├── models.rs
│   │   │   └── repositories.rs
│   │   ├── presentation/  # Presentation Layer
│   │   │   ├── query.rs
│   │   │   ├── mutation.rs
│   │   │   └── schema.rs
│   │   ├── lib.rs         # Entry point & DI
│   │   ├── database.rs    # Database connection
│   │   └── migration.rs   # DB migrations
│   └── Cargo.toml         # Rust dependencies
├── GRAPHQL_GUIDE.md       # GraphQL integration guide
└── CODING_GUIDELINES.md   # DDD architecture & coding standards
```

## 🎓 Learning Resources

For a comprehensive guide on the GraphQL integration, refer to:

- [GRAPHQL_GUIDE.md](./GRAPHQL_GUIDE.md) - Complete guide with examples
- [CODING_GUIDELINES.md](./CODING_GUIDELINES.md) - Project coding guidelines and best practices
- [QUICKSTART.md](./QUICKSTART.md) - Quick start guide

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
