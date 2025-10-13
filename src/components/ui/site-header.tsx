import { BookOpen } from "lucide-react"
import { Link } from "react-router"
import { Button } from "~/components/ui/button"
import { Separator } from "~/components/ui/separator"
import { SidebarTrigger, useSidebar } from "~/components/ui/sidebar"

export function SiteHeader() {
  const { state } = useSidebar()
  
  return (
    <header 
      className="flex h-16 shrink-0 items-center gap-2 border-b px-4"
      style={{
        marginLeft: state === "expanded" ? "var(--sidebar-width)" : "var(--sidebar-width-icon)",
      }}
    >
      <SidebarTrigger />
      <Separator orientation="vertical" className="mr-2 h-4" />
      <div className="flex flex-1 items-center justify-between">
        <h1 className="text-lg font-semibold">LifeBook</h1>
        <div className="flex items-center gap-2">
          <Button variant="ghost" size="sm" asChild>
            <Link to="/">
              <BookOpen className="h-4 w-4" />
              <span className="sr-only">Books</span>
            </Link>
          </Button>
        </div>
      </div>
    </header>
  )
}
