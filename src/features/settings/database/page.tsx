import { useEffect, useState } from "react"
import { useFetcher } from "react-router"
import { Button } from "~/components/ui/button"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import type { Route } from "./+types/page"
import { getDatabaseSettings } from "./api/getDatabaseSettings"
import { updateDatabaseSettings } from "./api/updateDatabaseSettings"

export function meta(_: Route.MetaArgs) {
  return [{ title: "Settings - Database" }]
}

export async function clientLoader() {
  const databaseSettings = await getDatabaseSettings()
  return { databaseSettings }
}

export async function clientAction({ request }: Route.ClientActionArgs) {
  const formData = await request.formData()
  const databaseDirectory = formData.get("databaseDirectory") as string | null
  const updateDatabaseSettingsInput = { databaseDirectory: databaseDirectory ?? "" }
  const updateDatabaseSettingsResult = await updateDatabaseSettings(updateDatabaseSettingsInput)
  return { updateDatabaseSettings: updateDatabaseSettingsResult }
}

export default function DatabaseSettingsPage({ loaderData }: Route.ComponentProps) {
  const fetcher = useFetcher<typeof clientAction>()
  const isSubmitting = fetcher.state !== "idle"
  const updatedDirectory = fetcher.data?.updateDatabaseSettings?.databaseDirectory
  const [directory, setDirectory] = useState(() => loaderData.databaseSettings.databaseDirectory)

  useEffect(() => {
    if (updatedDirectory) {
      setDirectory(updatedDirectory)
    }
  }, [updatedDirectory])

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-semibold mb-1">データベース</h2>
        <p className="text-sm text-muted-foreground">
          データベースファイルの保存場所を管理します。
        </p>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="database-directory">データベース保存先</Label>
          <div className="flex gap-2">
            <Input
              id="database-directory"
              value={directory}
              onChange={(e) => setDirectory(e.target.value)}
              placeholder="/path/to/database"
              className="flex-1"
              disabled={isSubmitting}
            />
            <Button type="button" variant="outline" disabled={isSubmitting}>
              参照
            </Button>
          </div>
          <p className="text-xs text-muted-foreground">
            LifeBookのデータベースファイルが保存されるディレクトリです。
          </p>
        </div>

        <fetcher.Form method="post">
          <input type="hidden" name="databaseDirectory" value={directory} />
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? "保存中..." : "保存"}
          </Button>
        </fetcher.Form>
      </div>
    </div>
  )
}
