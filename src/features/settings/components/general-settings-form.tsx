// Settings Feature - General Settings Form Component

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

export function GeneralSettingsForm() {
  const {
    generalSettings,
    loadingGeneral,
    errorGeneral,
    fetchGeneralSettings,
    updateGeneralSettings,
  } = useSettings()

  useEffect(() => {
    fetchGeneralSettings()
  }, [fetchGeneralSettings])

  const handleLanguageChange = async (value: string) => {
    await updateGeneralSettings(value)
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-semibold mb-1">一般設定</h2>
        <p className="text-sm text-muted-foreground">アプリケーションの基本設定を管理します。</p>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="language">言語</Label>
          <Select
            value={generalSettings?.language || "ja"}
            onValueChange={handleLanguageChange}
            disabled={loadingGeneral}
          >
            <SelectTrigger id="language" className="w-full max-w-xs">
              <SelectValue placeholder="言語を選択" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="ja">日本語</SelectItem>
              <SelectItem value="en">English</SelectItem>
            </SelectContent>
          </Select>
          <p className="text-xs text-muted-foreground">アプリケーションの表示言語を設定します。</p>
        </div>

        {errorGeneral && (
          <div className="text-sm text-destructive bg-destructive/10 p-3 rounded-md">
            {errorGeneral}
          </div>
        )}
      </div>
    </div>
  )
}
