import type * as React from "react"

import { SearchForm } from "~/components/common/search-form"
import { VersionSwitcher } from "~/components/common/version-switcher"
import {
  Sidebar,
  SidebarContent,
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
                {item.items.map((subItem) => (
                  <SidebarMenuItem key={subItem.title}>
                    <SidebarMenuButton
                      asChild
                      isActive={"isActive" in subItem ? subItem.isActive : false}
                    >
                      <a href={subItem.url}>{subItem.title}</a>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        ))}
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  )
}
