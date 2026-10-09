import { useEffect, useState } from "react";

export default function HomePage() {
  const [version, setVersion] = useState<string | null>(null);

  useEffect(() => {
    async function checkVersion() {
      try {
        const response = await fetch("http://localhost:8080/api/version");
        const data = await response.json();
        setVersion(data.version);
      } catch (error) {
        console.error("Error fetching version:", error);
      }
    }

    checkVersion();
  }, []);

  return (
    <main className="bg-zinc-950 text-white min-h-screen flex items-center justify-center">
      <h1 className="text-2xl">
        Current version: {version}
      </h1>
    </main>
  );
}