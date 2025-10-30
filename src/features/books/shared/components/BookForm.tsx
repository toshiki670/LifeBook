import { Form, Link } from "react-router"
import { Button } from "~/components/ui/button"
import { DialogHeader, DialogTitle } from "~/components/ui/dialog"
import { Input } from "~/components/ui/input"
import { Label } from "~/components/ui/label"
import { Textarea } from "~/components/ui/textarea"
import type { BookFormValues } from "../types"

interface BookFormProps {
  title: string
  initialValues?: Partial<BookFormValues>
  isSubmitting: boolean
  submitLabel: string
  submittingLabel: string
  cancelTo: string
}

export function BookForm({
  title,
  initialValues,
  isSubmitting,
  submitLabel,
  submittingLabel,
  cancelTo,
}: BookFormProps) {
  const values: Partial<BookFormValues> = initialValues ?? {}

  return (
    <>
      <DialogHeader>
        <DialogTitle>{title}</DialogTitle>
      </DialogHeader>

      <Form method="post" className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="title">
            タイトル <span className="text-destructive">*</span>
          </Label>
          <Input
            id="title"
            name="title"
            type="text"
            required
            placeholder="本のタイトル"
            disabled={isSubmitting}
            defaultValue={values.title || ""}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="author">著者</Label>
          <Input
            id="author"
            name="author"
            type="text"
            placeholder="著者名"
            disabled={isSubmitting}
            defaultValue={values.author || ""}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="description">説明</Label>
          <Textarea
            id="description"
            name="description"
            placeholder="本の説明"
            rows={3}
            disabled={isSubmitting}
            defaultValue={values.description || ""}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="publishedYear">出版年</Label>
          <Input
            id="publishedYear"
            name="publishedYear"
            type="number"
            placeholder="2024"
            disabled={isSubmitting}
            defaultValue={values.publishedYear ?? ""}
          />
        </div>

        <div className="flex gap-2 justify-end">
          <Button type="button" variant="outline" asChild disabled={isSubmitting}>
            <Link to={cancelTo}>キャンセル</Link>
          </Button>
          <Button type="submit" disabled={isSubmitting}>
            {isSubmitting ? submittingLabel : submitLabel}
          </Button>
        </div>
      </Form>
    </>
  )
}
