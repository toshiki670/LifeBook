import type { Location } from "react-router"
import { NavLink, Outlet, useLocation, useNavigate } from "react-router"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog"
import type { Route } from "./+types/page"

export function meta(_: Route.MetaArgs) {
  return [{ title: "Settings - LifeBook" }]
}

export default function SettingsLayout() {
  const navigate = useNavigate()
  const location = useLocation()
  const backgroundLocation = (location.state as { backgroundLocation?: Location } | undefined)
    ?.backgroundLocation

  const handleOpenChange = (open: boolean) => {
    if (!open) {
      if (backgroundLocation) {
        navigate(-1)
      } else {
        navigate("/")
      }
    }
  }

  return (
    <Dialog open onOpenChange={handleOpenChange} modal>
      <DialogContent className="sm:max-w-4xl h-[80vh] p-0" showCloseButton>
        <DialogHeader className="px-6 pt-6 pb-4 border-b">
          <DialogTitle>設定</DialogTitle>
          <DialogDescription>
            アプリケーションの設定を管理します。変更は自動的に保存されます。
          </DialogDescription>
        </DialogHeader>
        <div className="flex h-[calc(80vh-6rem)] overflow-hidden">
          <div className="w-1/4 min-w-[200px] border-r">
            <nav className="flex h-full flex-col gap-1 p-6">
              <NavLink
                to="/settings"
                className={({ isActive }) =>
                  `rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring ${isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground"}`
                }
                end
              >
                一般設定
              </NavLink>
              <NavLink
                to="/settings/appearance"
                className={({ isActive }) =>
                  `rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring ${isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground"}`
                }
              >
                外観
              </NavLink>
              <NavLink
                to="/settings/database"
                className={({ isActive }) =>
                  `rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring ${isActive ? "bg-accent text-accent-foreground" : "text-muted-foreground"}`
                }
              >
                データベース
              </NavLink>
            </nav>
          </div>
          <div className="flex-1 overflow-y-auto p-6">
            <Outlet />
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
