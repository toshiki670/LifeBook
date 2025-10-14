import { index, layout, type RouteConfig, route } from "@react-router/dev/routes"

export default [
  layout("layouts/app-layout.tsx", [
    index("features/dashboard/page.tsx"),
    route("books", "features/books/page.tsx"),
  ]),
] satisfies RouteConfig
