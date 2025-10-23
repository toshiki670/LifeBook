// Settings Feature - Settings Modal Component

import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "~/components/ui/dialog"
import { AppearanceSettingsForm } from "./components/appearance-settings-form"
import { DatabaseSettingsForm } from "./components/database-settings-form"
import { GeneralSettingsForm } from "./components/general-settings-form"
import { SettingsSidebar } from "./components/settings-sidebar"
import type { SettingsSection } from "./types"

interface SettingsModalProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function SettingsModal({ open, onOpenChange }: SettingsModalProps) {
  const [selectedSection, setSelectedSection] = useState<SettingsSection>("general")

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-4xl h-[80vh] p-0">
        <DialogHeader className="px-6 pt-6 pb-4 border-b">
          <DialogTitle>設定</DialogTitle>
          <DialogDescription>
            アプリケーションの設定を管理します。変更は自動的に保存されます。
          </DialogDescription>
        </DialogHeader>

        <div className="flex h-[calc(80vh-6rem)] overflow-hidden">
          {/* Left Sidebar - 25% */}
          <div className="w-1/4 min-w-[200px] p-6">
            <SettingsSidebar
              selectedSection={selectedSection}
              onSelectSection={setSelectedSection}
            />
          </div>

          {/* Right Content - 75% - Design Research */}
          <div className="flex-1 overflow-y-auto p-6">
            {selectedSection === "general" && <GeneralSettingsForm />}
            {selectedSection === "appearance" && <AppearanceSettingsForm />}
            {selectedSection === "database" && <DatabaseSettingsForm />}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
