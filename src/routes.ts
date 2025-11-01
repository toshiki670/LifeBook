import { index, layout, type RouteConfig, route } from "@react-router/dev/routes"

export default [
  layout("layouts/app-layout.tsx", [
    index("features/dashboard/page.tsx"),
    route("books", "features/books/list/page.tsx", [
      route("create", "features/books/create/page.tsx"),
    ]),
    route("books/:id", "features/books/detail/page.tsx", [
      route("edit", "features/books/update/page.tsx"),
    ]),
    route("settings", "features/settings/page.tsx", [
      index("features/settings/general/page.tsx"),
      route("appearance", "features/settings/appearance/page.tsx"),
      route("database", "features/settings/database/page.tsx"),
    ]),
  ]),
] satisfies RouteConfig
