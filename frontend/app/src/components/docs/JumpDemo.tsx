import { useState, useEffect, useRef } from "react"
import { cn } from "@/lib/utils"

const DIRECTORIES = [
  { name: "backend", label: "AS" },
  { name: "frontend", label: "AD" },
  { name: "docs", label: "AF" },
  { name: "scripts", label: "AG" },
  { name: "tests", label: "AH" },
  { name: "config", label: "AJ" },
]

export function JumpDemo() {
  const [mode, setMode] = useState<"shell" | "jump">("shell")
  const [cwd, setCwd] = useState("~/projects")
  const [command, setCommand] = useState("")
  const [jumpInput, setJumpInput] = useState("")
  const containerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    let timeout: NodeJS.Timeout

    const runSimulation = async () => {
        setMode("shell")
        setCwd("~/projects")
        setCommand("")
        setJumpInput("")

        await new Promise(r => setTimeout(r, 1000))

        const cmd = "j"
        for (let i = 0; i <= cmd.length; i++) {
            setCommand(cmd.slice(0, i))
            await new Promise(r => setTimeout(r, 150))
        }

        await new Promise(r => setTimeout(r, 500))

        setMode("jump")
        setCommand("")

        await new Promise(r => setTimeout(r, 1000))

        setJumpInput("A")
        await new Promise(r => setTimeout(r, 400))
        setJumpInput("AD")
        
        await new Promise(r => setTimeout(r, 400))

        setMode("shell")
        setCwd("~/projects/frontend")
        setJumpInput("")

        await new Promise(r => setTimeout(r, 2000))
        runSimulation()
    }

    runSimulation()

    return () => {}
  }, [])

  return (
    <div ref={containerRef} className="rounded-lg border border-zinc-800 bg-zinc-950 font-mono text-sm p-4 shadow-2xl h-[300px] overflow-hidden flex flex-col relative">
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
                <div className="border-b border-zinc-800 pb-2 mb-2 text-zinc-500 text-xs uppercase tracking-wider">
                    Jump Mode
                </div>
                <div className="grid grid-cols-1 gap-1">
                    {DIRECTORIES.map((dir) => {
                        const isMatch = jumpInput.length > 0 && dir.label.startsWith(jumpInput)
                        const isSelected = jumpInput === dir.label

                        return (
                            <div key={dir.label} className={cn(
                                "flex items-center gap-4 px-2 py-1 rounded transition-colors",
                                isSelected ? "bg-zinc-800 text-white" : "text-zinc-400"
                            )}>
                                <span className={cn(
                                    "font-bold px-1.5 py-0.5 rounded bg-zinc-900 border border-zinc-700 text-xs",
                                    isMatch || isSelected ? "text-yellow-400 border-yellow-400/50" : "text-zinc-500"
                                )}>
                                    {dir.label}
                                </span>
                                <span>{dir.name}/</span>
                            </div>
                        )
                    })}
                </div>
                <div className="mt-auto border-t border-zinc-800 pt-2 text-zinc-500 text-xs flex justify-between">
                   <span>Input: <span className="text-white font-bold">{jumpInput}</span><span className="animate-pulse">_</span></span>
                   <span>[ESC] Cancel</span>
                </div>
            </div>
        )}
    </div>
  )
}
