import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom"
import { LandingPage } from "@/components/LandingPage"
import { ThemeProvider } from "@/components/theme-provider"
import { Header } from "@/components/Header"
import DocsLayout from "@/layouts/DocsLayout"
import { Introduction } from "@/pages/docs/Introduction"
import { Installation } from "@/pages/docs/Installation"
import { HowItWorks } from "@/pages/docs/HowItWorks"
import { BasicCommands } from "@/pages/docs/BasicCommands"

export function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={
            <div className="min-h-screen bg-background text-foreground flex justify-center">
              <div className="w-full max-w-7xl border-x border-zinc-800 min-h-screen">
                <Header />
                <main>
                  <LandingPage />
                </main>
              </div>
            </div>
          } />
          
          <Route path="/docs" element={<DocsLayout />}>
            <Route index element={<Navigate to="/docs/introduction" replace />} />
            <Route path="introduction" element={<Introduction />} />
            <Route path="installation" element={<Installation />} />
            <Route path="how-it-works" element={<HowItWorks />} />
            <Route path="basic-commands" element={<BasicCommands />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </ThemeProvider>
  )
}

export default App;
