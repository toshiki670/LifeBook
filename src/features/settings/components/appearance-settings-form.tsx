// Settings Feature - Appearance Settings Form Component

import { useEffect } from "react"
import { Label } from "~/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select"
import { useSettings } from "../hooks/use-settings"

export function AppearanceSettingsForm() {
  const {
    appearanceSettings,
    loadingAppearance,
    errorAppearance,
    fetchAppearanceSettings,
    updateAppearanceSettings,
  } = useSettings()

  useEffect(() => {
    fetchAppearanceSettings()
  }, [fetchAppearanceSettings])

  const handleThemeChange = async (value: string) => {
    await updateAppearanceSettings(value)
  }

  const getThemeLabel = (theme: string) => {
    switch (theme) {
      case "light":
        return "ライト"
      case "dark":
        return "ダーク"
      case "system":
        return "システム設定"
      default:
        return theme
    }
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-semibold mb-1">外観</h2>
        <p className="text-sm text-muted-foreground">
          アプリケーションの見た目をカスタマイズします。
        </p>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="theme">テーマ</Label>
          <Select
            value={appearanceSettings?.theme || "system"}
            onValueChange={handleThemeChange}
            disabled={loadingAppearance}
          >
            <SelectTrigger id="theme" className="w-full max-w-xs">
              <SelectValue placeholder="テーマを選択" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="light">ライト</SelectItem>
              <SelectItem value="dark">ダーク</SelectItem>
              <SelectItem value="system">システム設定</SelectItem>
            </SelectContent>
          </Select>
          <p className="text-xs text-muted-foreground">
            {appearanceSettings?.theme === "system"
              ? "システムの設定に従ってテーマが自動的に切り替わります。"
              : `現在のテーマ: ${getThemeLabel(appearanceSettings?.theme || "system")}`}
          </p>
        </div>

        {errorAppearance && (
          <div className="text-sm text-destructive bg-destructive/10 p-3 rounded-md">
            {errorAppearance}
          </div>
        )}
      </div>
    </div>
  )
}
