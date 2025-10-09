import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";
import { Link } from "react-router";
import { Button } from "~/components/ui/button";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "LifeBook - SeaORM + GraphQL Demo" },
    { name: "description", content: "Tauri with SeaORM and GraphQL" },
  ];
}

export default function Home() {
  return (
    <div>
      <Welcome />
      <div className="flex justify-center mt-8">
        <Button asChild size="lg">
          <Link to="/books">
            📚 Book Manager (GraphQL Demo) へ
          </Link>
        </Button>
      </div>
    </div>
  );
}
