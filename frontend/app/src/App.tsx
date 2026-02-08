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
            <div className="min-h-screen bg-background text-foreground flex flex-col items-center">
              <div className="w-full max-w-5xl border-x border-border/40 min-h-screen relative shadow-[0_0_50px_-12px_rgba(0,0,0,0.5)] bg-background/50">
                  <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px] pointer-events-none -z-10" />
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
