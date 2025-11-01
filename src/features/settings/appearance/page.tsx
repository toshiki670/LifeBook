import { useEffect } from "react"
import { useFetcher } from "react-router"
import { Label } from "~/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select"
import type { Route } from "./+types/page"
import { getAppearanceSettings } from "./api/getAppearanceSettings"
import { updateAppearanceSettings } from "./api/updateAppearanceSettings"
import { applyTheme, type ThemeMode } from "./utils/theme"

export function meta(_: Route.MetaArgs) {
  return [{ title: "Settings - Appearance" }]
}

export async function clientLoader() {
  const appearanceSettings = await getAppearanceSettings()
  return { appearanceSettings }
}

export async function clientAction({ request }: Route.ClientActionArgs) {
  const formData = await request.formData()
  const theme = formData.get("theme") as string | null
  const updateAppearanceSettingsInput = { theme: theme ?? "system" }
  const updateAppearanceSettingsResult = await updateAppearanceSettings(
    updateAppearanceSettingsInput,
  )
  return { updateAppearanceSettings: updateAppearanceSettingsResult }
}

export default function AppearanceSettingsPage({ loaderData }: Route.ComponentProps) {
  const fetcher = useFetcher<typeof clientAction>()
  const isSubmitting = fetcher.state !== "idle"
  const currentTheme = (fetcher.data?.updateAppearanceSettings?.theme ??
    loaderData.appearanceSettings.theme) as ThemeMode

  useEffect(() => {
    if (!currentTheme) {
      return
    }
    applyTheme(currentTheme)
  }, [currentTheme])

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
          <fetcher.Form method="post">
            <Select
              value={currentTheme}
              onValueChange={(value: string) => {
                const fd = new FormData()
                fd.set("theme", value)
                fetcher.submit(fd, { method: "post" })
              }}
              disabled={isSubmitting}
            >
              <SelectTrigger id="theme" className="w-full">
                <SelectValue placeholder="テーマを選択" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="light">ライト</SelectItem>
                <SelectItem value="dark">ダーク</SelectItem>
                <SelectItem value="system">システム設定</SelectItem>
              </SelectContent>
            </Select>
          </fetcher.Form>
          <p className="text-xs text-muted-foreground">
            {currentTheme === "system"
              ? "システムの設定に従ってテーマが自動的に切り替わります。"
              : `現在のテーマ: ${currentTheme}`}
          </p>
        </div>
      </div>
    </div>
  )
}
