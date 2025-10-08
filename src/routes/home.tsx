import type { Route } from "./+types/home";
import { Welcome } from "../welcome/welcome";
import { Link } from "react-router";

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
        <Link
          to="/books"
          className="bg-blue-500 hover:bg-blue-600 text-white font-semibold py-3 px-6 rounded-lg transition shadow-md hover:shadow-lg"
        >
          📚 Book Manager (GraphQL Demo) へ
        </Link>
      </div>
    </div>
  );
}
