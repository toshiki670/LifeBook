import { index, layout, type RouteConfig, route } from "@react-router/dev/routes"

export default [
  layout("routes/_app.tsx", [index("routes/dashboard.tsx")]),
  route("home", "routes/home.tsx"),
  route("books", "routes/books.tsx"),
] satisfies RouteConfig
