// Settings Feature - Settings Sidebar Component

import { cn } from "~/lib/utils"
import type { SettingsSection } from "../types"

interface SettingsSidebarProps {
  selectedSection: SettingsSection
  onSelectSection: (section: SettingsSection) => void
  className?: string
}

const sections: Array<{ id: SettingsSection; label: string }> = [
  { id: "general", label: "一般設定" },
  { id: "appearance", label: "外観" },
  { id: "database", label: "データベース" },
]

export function SettingsSidebar({
  selectedSection,
  onSelectSection,
  className,
}: SettingsSidebarProps) {
  return (
    <nav className={cn("flex flex-col space-y-1 border-r pr-4", className)}>
      {sections.map((section) => (
        <button
          key={section.id}
          type="button"
          onClick={() => onSelectSection(section.id)}
          className={cn(
            "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
            "hover:bg-accent hover:text-accent-foreground",
            "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
            selectedSection === section.id
              ? "bg-accent text-accent-foreground"
              : "text-muted-foreground",
          )}
        >
          {section.label}
        </button>
      ))}
    </nav>
  )
}
