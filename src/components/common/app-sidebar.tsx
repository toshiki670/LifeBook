import { Settings } from "lucide-react"
import type * as React from "react"
import { useState } from "react"

import { Link } from "react-router"
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
import { SettingsModal } from "~/features/settings/settings-modal"

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
    {
      title: "設定",
      url: "#",
      items: [
        {
          title: "アプリケーション設定",
          url: "#",
        },
        {
          title: "データベース",
          url: "#",
        },
        {
          title: "バックアップ",
          url: "#",
        },
      ],
    },
  ],
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const [settingsOpen, setSettingsOpen] = useState(false)

  return (
    <>
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
                  {item.items.map((subItem) => (
                    <SidebarMenuItem key={subItem.title}>
                      <SidebarMenuButton
                        asChild
                        isActive={"isActive" in subItem ? subItem.isActive : false}
                      >
                        <Link to={subItem.url}>{subItem.title}</Link>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  ))}
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          ))}
        </SidebarContent>
        <SidebarFooter>
          <Button
            variant="ghost"
            className="w-full justify-start"
            onClick={() => setSettingsOpen(true)}
          >
            <Settings className="mr-2 h-4 w-4" />
            設定
          </Button>
        </SidebarFooter>
        <SidebarRail />
      </Sidebar>

      <SettingsModal open={settingsOpen} onOpenChange={setSettingsOpen} />
    </>
  )
}
