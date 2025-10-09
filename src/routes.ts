import { index, type RouteConfig, route } from "@react-router/dev/routes"

export default [index("routes/home.tsx"), route("books", "routes/books.tsx")] satisfies RouteConfig
