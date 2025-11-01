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
import { getGeneralSettings } from "./api/getGeneralSettings"
import { updateGeneralSettings } from "./api/updateGeneralSettings"

export function meta(_: Route.MetaArgs) {
  return [{ title: "Settings - General" }]
}

export async function clientLoader() {
  const generalSettings = await getGeneralSettings()
  return { generalSettings }
}

export async function clientAction({ request }: Route.ClientActionArgs) {
  const formData = await request.formData()
  const language = formData.get("language") as string | null
  const updateGeneralSettingsInput = { language: language ?? "ja" }
  const updateGeneralSettingsResult = await updateGeneralSettings(updateGeneralSettingsInput)
  return { updateGeneralSettings: updateGeneralSettingsResult }
}

export default function GeneralSettingsPage({ loaderData }: Route.ComponentProps) {
  const fetcher = useFetcher<typeof clientAction>()
  const isSubmitting = fetcher.state !== "idle"
  const currentLanguage =
    fetcher.data?.updateGeneralSettings?.language ?? loaderData.generalSettings.language

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-semibold mb-1">一般設定</h2>
        <p className="text-sm text-muted-foreground">アプリケーションの基本設定を管理します。</p>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="language">言語</Label>
          <fetcher.Form method="post">
            <Select
              value={currentLanguage}
              onValueChange={(value: string) => {
                const fd = new FormData()
                fd.set("language", value)
                fetcher.submit(fd, { method: "post" })
              }}
              disabled={isSubmitting}
            >
              <SelectTrigger id="language" className="w-full">
                <SelectValue placeholder="言語を選択" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="ja">日本語</SelectItem>
                <SelectItem value="en">English</SelectItem>
              </SelectContent>
            </Select>
          </fetcher.Form>
          <p className="text-xs text-muted-foreground">アプリケーションの表示言語を設定します。</p>
        </div>
      </div>
    </div>
  )
}
