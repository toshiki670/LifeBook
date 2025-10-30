import type { BookFormValues } from "../types"

export function parseBookFormData(formData: FormData): BookFormValues {
  const title = (formData.get("title") as string) ?? ""
  const authorRaw = formData.get("author") as string | null
  const descriptionRaw = formData.get("description") as string | null
  const publishedYearRaw = formData.get("publishedYear") as string | null

  const author = authorRaw ? authorRaw : undefined
  const description = descriptionRaw ? descriptionRaw : undefined
  const publishedYear = publishedYearRaw ? parseInt(publishedYearRaw, 10) : undefined

  return {
    title,
    author,
    description,
    publishedYear,
  }
}
