import { Settings } from "lucide-react"
import type * as React from "react"

import { NavLink, useLocation } from "react-router"
import { SearchForm } from "~/components/common/search-form"
import { VersionSwitcher } from "~/components/common/version-switcher"
import { Button } from "~/components/ui/button"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "~/components/ui/sidebar"

// LifeBook navigation data
const data = {
  versions: ["0.1.0"],
  navMain: [
    {
      title: "ダッシュボード",
      url: "/",
      items: [
        {
          title: "概要",
          url: "/",
          isActive: true,
        },
      ],
    },
    {
      title: "ライブラリ",
      url: "#",
      items: [
        {
          title: "書籍管理",
          url: "/books",
        },
        {
          title: "著者",
          url: "#",
        },
        {
          title: "出版社",
          url: "#",
        },
        {
          title: "タグ",
          url: "#",
        },
      ],
    },
  ],
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const location = useLocation()

  return (
    <Sidebar {...props}>
      <SidebarHeader>
        <VersionSwitcher versions={data.versions} defaultVersion={data.versions[0]} />
        <SearchForm />
      </SidebarHeader>
      <SidebarContent>
        {/* We create a SidebarGroup for each parent. */}
        {data.navMain.map((item) => (
          <SidebarGroup key={item.title}>
            <SidebarGroupLabel>{item.title}</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {item.items.map((subItem) => {
                  const isActive =
                    "isActive" in subItem
                      ? subItem.isActive
                      : subItem.url !== "#" && location.pathname.startsWith(subItem.url)

                  const navState = subItem.url.startsWith("/settings")
                    ? { backgroundLocation: location }
                    : undefined

                  return (
                    <SidebarMenuItem key={subItem.title}>
                      <SidebarMenuButton asChild isActive={isActive}>
                        <NavLink to={subItem.url} state={navState}>
                          {subItem.title}
                        </NavLink>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  )
                })}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        ))}
      </SidebarContent>
      <SidebarFooter>
        <Button asChild variant="ghost" className="w-full justify-start">
          <NavLink to="/settings" state={{ backgroundLocation: location }}>
            <Settings className="mr-2 h-4 w-4" />
            設定
          </NavLink>
        </Button>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  )
}
