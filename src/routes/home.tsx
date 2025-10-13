import { Link } from "react-router"
import { Button } from "~/components/ui/button"
import { Welcome } from "../welcome/welcome"
import type { Route } from "./+types/home"

export function meta(_: Route.MetaArgs) {
  return [
    { title: "LifeBook - SeaORM + GraphQL Demo" },
    { name: "description", content: "Tauri with SeaORM and GraphQL" },
  ]
}

export default function Home() {
  return (
    <div>
      <Welcome />
      <div className="flex justify-center mt-8">
        <Button asChild size="lg">
          <Link to="/books" prefetch="intent">
            📚 Book Manager (GraphQL Demo) へ
          </Link>
        </Button>
      </div>
    </div>
  )
}
