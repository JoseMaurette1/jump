import { Outlet } from "react-router-dom"
import { Header } from "@/components/Header"
import { DocsSidebar } from "@/components/DocsSidebar"

export default function DocsLayout() {
  return (
    <div className="relative flex min-h-screen flex-col bg-background">
      <Header />
      <div className="container flex-1 items-start md:grid md:grid-cols-[220px_minmax(0,1fr)] md:gap-6 lg:grid-cols-[240px_minmax(0,1fr)] lg:gap-10 mx-auto px-4 md:px-8">
        <DocsSidebar />
        <main className="relative py-6 lg:gap-10 lg:py-8">
          <div className="mx-auto w-full min-w-0">
             <Outlet />
          </div>
        </main>
      </div>
    </div>
  )
}
