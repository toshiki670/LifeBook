"use client"

import { useEffect } from "react"
import { Outlet } from "react-router"
import { AppSidebar } from "~/components/common/app-sidebar"
import { SidebarInset, SidebarProvider } from "~/components/ui/sidebar"
import { getAppearanceSettings } from "~/features/settings/appearance/api/getAppearanceSettings"
import {
  applyTheme,
  disposeThemeListener,
  type ThemeMode,
} from "~/features/settings/appearance/utils/theme"
import type { Route } from "./+types/app-layout"

export async function clientLoader(_: Route.ClientLoaderArgs) {
  const appearanceSettings = await getAppearanceSettings()
  return { appearanceSettings }
}

export default function AppLayout({ loaderData }: Route.ComponentProps) {
  const { appearanceSettings } = loaderData

  useEffect(() => {
    const theme = appearanceSettings?.theme as ThemeMode | undefined
    if (theme) {
      applyTheme(theme)
    }
    return () => {
      disposeThemeListener()
    }
  }, [appearanceSettings?.theme])

  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <Outlet />
      </SidebarInset>
    </SidebarProvider>
  )
}
