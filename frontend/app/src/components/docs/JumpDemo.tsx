import { useState, useEffect, useRef } from "react"
import { cn } from "@/lib/utils"

const DIRECTORIES = [
  { name: "backend", isBookmark: false },
  { name: "frontend", isBookmark: false },
  { name: "docs", isBookmark: false },
  { name: "projects", isBookmark: true, bookmarkKey: "p" },
  { name: "scripts", isBookmark: false },
  { name: "work", isBookmark: true, bookmarkKey: "w" },
]

type Mode = "normal" | "search"

export function JumpDemo() {
  const [mode, setMode] = useState<"shell" | "tui">("shell")
  const [tuiMode, setTuiMode] = useState<Mode>("normal")
  const [cwd, setCwd] = useState("~/projects")
  const [command, setCommand] = useState("")
  const [searchQuery, setSearchQuery] = useState("")
  const [selectedIndex, setSelectedIndex] = useState(0)
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const runSimulation = async () => {
      // Reset
      setMode("shell")
      setTuiMode("normal")
      setCwd("~/projects")
      setCommand("")
      setSearchQuery("")
      setSelectedIndex(0)

      await new Promise(r => setTimeout(r, 1000))

      // Type "jump" command
      const cmd = "jump"
      for (let i = 0; i <= cmd.length; i++) {
        setCommand(cmd.slice(0, i))
        await new Promise(r => setTimeout(r, 150))
      }

      await new Promise(r => setTimeout(r, 500))

      // Enter TUI
      setMode("tui")
      setCommand("")

      await new Promise(r => setTimeout(r, 1000))

      // Navigate down with j
      setSelectedIndex(1)
      await new Promise(r => setTimeout(r, 600))
      setSelectedIndex(2)
      await new Promise(r => setTimeout(r, 600))

      // Press / to search
      setTuiMode("search")
      await new Promise(r => setTimeout(r, 400))

      // Type "wor"
      const search = "wor"
      for (let i = 0; i <= search.length; i++) {
        setSearchQuery(search.slice(0, i))
        await new Promise(r => setTimeout(r, 200))
      }

      await new Promise(r => setTimeout(r, 800))

      // Press Enter
      setMode("shell")
      setCwd("~/projects/work")
      setTuiMode("normal")
      setSearchQuery("")

      await new Promise(r => setTimeout(r, 2000))
      runSimulation()
    }

    runSimulation()

    return () => {}
  }, [])

  const getFilteredDirs = () => {
    if (searchQuery === "") return DIRECTORIES
    return DIRECTORIES.filter(d =>
      d.name.toLowerCase().includes(searchQuery.toLowerCase())
    )
  }

  const filteredDirs = getFilteredDirs()

  return (
    <div ref={containerRef} className="rounded-2xl border border-zinc-800 bg-zinc-950 font-mono text-sm p-4 shadow-2xl h-[350px] overflow-hidden flex flex-col relative">
      <div className="flex gap-2 mb-4 border-b border-zinc-900/50 pb-2">
        <div className="w-3 h-3 rounded-full bg-red-500/80"/>
        <div className="w-3 h-3 rounded-full bg-yellow-500/80"/>
        <div className="w-3 h-3 rounded-full bg-green-500/80"/>
      </div>

      {mode === "shell" ? (
        <div className="text-zinc-300">
          <div className="flex gap-2">
            <span className="text-green-500">➜</span>
            <span className="text-blue-500">{cwd}</span>
            <span className="text-zinc-100">{command}</span>
            <span className="animate-pulse bg-zinc-500 w-2 h-5 block"></span>
          </div>
        </div>
      ) : (
        <div className="text-zinc-300 flex-1 flex flex-col">
          <div className="border-b border-zinc-800 pb-2 mb-2 flex items-center justify-between">
            <span className="text-zinc-500 text-xs uppercase tracking-wider">
              {tuiMode === "search" ? "Search Mode" : "Normal Mode"}
            </span>
            <span className="text-zinc-600 text-xs">
              {tuiMode === "normal" ? "/ to search" : "ESC to cancel"}
            </span>
          </div>

          <div className="grid grid-cols-1 gap-0.5 flex-1">
            {filteredDirs.map((dir, idx) => {
              const isSelected = idx === selectedIndex && tuiMode === "normal"

              return (
                <div key={dir.name} className={cn(
                  "flex items-center gap-3 px-2 py-1 rounded transition-colors",
                  isSelected ? "bg-blue-600/20 text-white border-l-2 border-blue-500" : "text-zinc-400"
                )}>
                  <span className="text-zinc-600 text-xs w-4 text-right">
                    {isSelected ? "▶" : idx - selectedIndex}
                  </span>
                  {dir.isBookmark && (
                    <span className="text-yellow-400 text-xs">★</span>
                  )}
                  <span className={isSelected ? "font-semibold" : ""}>
                    {dir.name}/
                  </span>
                  {dir.isBookmark && (
                    <span className="text-zinc-600 text-xs ml-auto">
                      [{dir.bookmarkKey}]
                    </span>
                  )}
                </div>
              )
            })}
          </div>

          {tuiMode === "search" && (
            <div className="mt-auto border-t border-zinc-800 pt-2 flex items-center gap-2">
              <span className="text-zinc-500 text-xs">/</span>
              <span className="text-white">{searchQuery}</span>
              <span className="animate-pulse bg-zinc-500 w-1.5 h-4"></span>
              <span className="ml-auto text-zinc-600 text-xs">
                {filteredDirs.length} matches
              </span>
            </div>
          )}

          {tuiMode === "normal" && (
            <div className="mt-auto border-t border-zinc-800 pt-2 text-zinc-600 text-xs flex justify-between">
              <span>j/k: move • b: bookmark • x: remove</span>
              <span>Enter: select</span>
            </div>
          )}
        </div>
      )}
    </div>
  )
}
