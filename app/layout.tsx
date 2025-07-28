import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "RSS Reader - You Know",
  description: "A modern RSS reader application built with Next.js and Tauri",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  );
}
