import { useEffect, useState } from "react";
import type { Route } from "./+types/books";
import { getBooks, createBook, deleteBook, getDbStatus } from "../lib/graphql";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Textarea } from "~/components/ui/textarea";
import { Card, CardContent, CardHeader, CardTitle } from "~/components/ui/card";
import { Label } from "~/components/ui/label";
import { Alert, AlertDescription } from "~/components/ui/alert";

interface Book {
  id: number;
  title: string;
  author?: string;
  description?: string;
  publishedYear?: number;
}

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Book Manager - LifeBook" },
    { name: "description", content: "Manage books with SeaORM + GraphQL + Tauri" },
  ];
}

export default function Books() {
  const [books, setBooks] = useState<Book[]>([]);
  const [dbStatus, setDbStatus] = useState<string>("Unknown");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newBook, setNewBook] = useState({
    title: "",
    author: "",
    description: "",
    publishedYear: "",
  });

  useEffect(() => {
    loadBooks();
    checkDbStatus();
  }, []);

  const checkDbStatus = async () => {
    try {
      const status = await getDbStatus();
      setDbStatus(status);
    } catch (err) {
      console.error("Failed to check DB status:", err);
    }
  };

  const loadBooks = async () => {
    try {
      setLoading(true);
      const response = await getBooks();
      if (response.errors) {
        setError(response.errors[0]?.message || "Unknown error");
      } else if (response.data) {
        setBooks(response.data.books || []);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load books");
    } finally {
      setLoading(false);
    }
  };

  const handleCreateBook = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const bookData = {
        title: newBook.title,
        author: newBook.author || undefined,
        description: newBook.description || undefined,
        publishedYear: newBook.publishedYear
          ? parseInt(newBook.publishedYear)
          : undefined,
      };

      const response = await createBook(bookData);
      if (response.errors) {
        setError(response.errors[0]?.message || "Failed to create book");
      } else {
        setNewBook({ title: "", author: "", description: "", publishedYear: "" });
        loadBooks();
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create book");
    }
  };

  const handleDeleteBook = async (id: number) => {
    try {
      const response = await deleteBook(id);
      if (response.errors) {
        setError(response.errors[0]?.message || "Failed to delete book");
      } else {
        loadBooks();
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to delete book");
    }
  };

  return (
    <div className="container mx-auto p-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">LifeBook - GraphQL Demo</h1>
        <p className="text-muted-foreground">
          SeaORM + GraphQL + Tauri の統合デモ
        </p>
        <p className="text-sm text-muted-foreground mt-2">
          DB Status: <span className="font-semibold">{dbStatus}</span>
        </p>
      </div>

      {error && (
        <Alert variant="destructive" className="mb-4">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {/* 新しい本を追加するフォーム */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle>新しい本を追加</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleCreateBook} className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="title">
                タイトル <span className="text-destructive">*</span>
              </Label>
              <Input
                id="title"
                type="text"
                required
                value={newBook.title}
                onChange={(e) => setNewBook({ ...newBook, title: e.target.value })}
                placeholder="本のタイトル"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="author">著者</Label>
              <Input
                id="author"
                type="text"
                value={newBook.author}
                onChange={(e) => setNewBook({ ...newBook, author: e.target.value })}
                placeholder="著者名"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="description">説明</Label>
              <Textarea
                id="description"
                value={newBook.description}
                onChange={(e) =>
                  setNewBook({ ...newBook, description: e.target.value })
                }
                placeholder="本の説明"
                rows={3}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="publishedYear">出版年</Label>
              <Input
                id="publishedYear"
                type="number"
                value={newBook.publishedYear}
                onChange={(e) =>
                  setNewBook({ ...newBook, publishedYear: e.target.value })
                }
                placeholder="2024"
              />
            </div>
            <Button type="submit" className="w-full">
              追加
            </Button>
          </form>
        </CardContent>
      </Card>

      {/* 本のリスト */}
      <Card>
        <CardHeader>
          <CardTitle>本のリスト</CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <p className="text-muted-foreground">読み込み中...</p>
          ) : books.length === 0 ? (
            <p className="text-muted-foreground">本がまだありません。上のフォームから追加してください。</p>
          ) : (
            <div className="space-y-4">
              {books.map((book) => (
                <Card key={book.id}>
                  <CardContent className="pt-6">
                    <div className="flex justify-between items-start">
                      <div className="flex-1">
                        <h3 className="text-xl font-semibold">{book.title}</h3>
                        {book.author && (
                          <p className="text-muted-foreground mt-1">著者: {book.author}</p>
                        )}
                        {book.description && (
                          <p className="mt-2">{book.description}</p>
                        )}
                        {book.publishedYear && (
                          <p className="text-muted-foreground text-sm mt-1">
                            出版年: {book.publishedYear}
                          </p>
                        )}
                      </div>
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={() => handleDeleteBook(book.id)}
                        className="ml-4"
                      >
                        削除
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

