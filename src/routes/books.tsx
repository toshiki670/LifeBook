import { useEffect, useState } from "react";
import type { Route } from "./+types/books";
import { getBooks, createBook, deleteBook, getDbStatus } from "../lib/graphql";

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
        <p className="text-gray-600">
          SeaORM + GraphQL + Tauri の統合デモ
        </p>
        <p className="text-sm text-gray-500 mt-2">
          DB Status: <span className="font-semibold">{dbStatus}</span>
        </p>
      </div>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      {/* 新しい本を追加するフォーム */}
      <div className="bg-white shadow-md rounded-lg p-6 mb-8">
        <h2 className="text-2xl font-semibold mb-4">新しい本を追加</h2>
        <form onSubmit={handleCreateBook} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">
              タイトル <span className="text-red-500">*</span>
            </label>
            <input
              type="text"
              required
              value={newBook.title}
              onChange={(e) => setNewBook({ ...newBook, title: e.target.value })}
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="本のタイトル"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">著者</label>
            <input
              type="text"
              value={newBook.author}
              onChange={(e) => setNewBook({ ...newBook, author: e.target.value })}
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="著者名"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">説明</label>
            <textarea
              value={newBook.description}
              onChange={(e) =>
                setNewBook({ ...newBook, description: e.target.value })
              }
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="本の説明"
              rows={3}
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">出版年</label>
            <input
              type="number"
              value={newBook.publishedYear}
              onChange={(e) =>
                setNewBook({ ...newBook, publishedYear: e.target.value })
              }
              className="w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="2024"
            />
          </div>
          <button
            type="submit"
            className="w-full bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded-lg transition"
          >
            追加
          </button>
        </form>
      </div>

      {/* 本のリスト */}
      <div className="bg-white shadow-md rounded-lg p-6">
        <h2 className="text-2xl font-semibold mb-4">本のリスト</h2>
        {loading ? (
          <p className="text-gray-500">読み込み中...</p>
        ) : books.length === 0 ? (
          <p className="text-gray-500">本がまだありません。上のフォームから追加してください。</p>
        ) : (
          <div className="space-y-4">
            {books.map((book) => (
              <div
                key={book.id}
                className="border rounded-lg p-4 hover:shadow-md transition"
              >
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <h3 className="text-xl font-semibold">{book.title}</h3>
                    {book.author && (
                      <p className="text-gray-600 mt-1">著者: {book.author}</p>
                    )}
                    {book.description && (
                      <p className="text-gray-700 mt-2">{book.description}</p>
                    )}
                    {book.publishedYear && (
                      <p className="text-gray-500 text-sm mt-1">
                        出版年: {book.publishedYear}
                      </p>
                    )}
                  </div>
                  <button
                    onClick={() => handleDeleteBook(book.id)}
                    className="ml-4 bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded transition"
                  >
                    削除
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

