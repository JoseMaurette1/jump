import { useEffect, useState } from "react"

const TYPING_DELAY = 100
const STEP_DELAY = 1000

export function Hero() {
  const [terminalState, setTerminalState] = useState(0)
  const [typedText, setTypedText] = useState("")

  useEffect(() => {
    let timeout: ReturnType<typeof setTimeout> | undefined

    const runAnimation = async () => {
      setTerminalState(0)
      setTypedText("")
      await new Promise(r => setTimeout(r, STEP_DELAY))

      const command = "jump"
      for (let i = 0; i <= command.length; i++) {
        setTypedText(command.slice(0, i))
        await new Promise(r => setTimeout(r, TYPING_DELAY))
      }
      
      await new Promise(r => setTimeout(r, 500))
      
      setTerminalState(1)
      setTypedText("")
      
      await new Promise(r => setTimeout(r, STEP_DELAY * 1.5))
      
      const selection = "as"
      for (let i = 0; i <= selection.length; i++) {
        setTypedText(selection.slice(0, i))
        await new Promise(r => setTimeout(r, TYPING_DELAY * 2))
      }
      
      await new Promise(r => setTimeout(r, 300))
      
      setTerminalState(2)
      setTypedText("")
      
      await new Promise(r => setTimeout(r, 3000))
      runAnimation()
    }

    runAnimation()

    return () => clearTimeout(timeout)
  }, [])

  return (
    <section className="py-24 text-center space-y-8 max-w-3xl mx-auto px-4 relative z-10">
      <div className="space-y-4">
        <h1 className="text-4xl sm:text-6xl font-extrabold tracking-tight">
          Navigate your terminal at the <span className="text-primary">speed of thought</span>
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          A minimal, Vim-inspired directory navigation tool built in Rust. 
          Stop typing <code className="bg-muted px-1 py-0.5 rounded font-mono text-sm">cd</code> and start jumping.
        </p>
      </div>
      
      
      <div className="pt-8 relative">
        <div className="bg-zinc-950 text-left p-4 rounded-lg shadow-2xl border border-zinc-800 font-mono text-sm max-w-lg mx-auto overflow-hidden min-h-[220px]">
          <div className="flex gap-2 mb-4 bg-zinc-950 pb-2 z-10 border-b border-zinc-900/50">
            <div className="w-3 h-3 rounded-full bg-red-500/80"/>
            <div className="w-3 h-3 rounded-full bg-yellow-500/80"/>
            <div className="w-3 h-3 rounded-full bg-green-500/80"/>
          </div>
          
          <div className="space-y-1 text-zinc-400">
            <p>
              <span className="text-green-400">➜</span> <span className="text-blue-400">~</span> 
              {terminalState === 0 && <>{typedText}<span className="animate-pulse">_</span></>}
              {terminalState > 0 && " jump"}
            </p>

            {terminalState >= 1 && (
              <div className="animate-in fade-in slide-in-from-top-1 duration-300">
                <p className="text-zinc-500 mb-2">Scanning directories...</p>
                <div className={`grid grid-cols-2 gap-x-8 gap-y-1 text-zinc-300 py-2 pl-2 border-l-2 border-zinc-800 ml-1 transition-opacity duration-500 ${terminalState === 2 ? 'opacity-50 blur-[0.5px]' : ''}`}>
                  <p><span className="text-blue-400 font-bold">as</span> src/</p>
                  <p><span className="text-blue-400 font-bold">df</span> target/</p>
                  <p><span className="text-blue-400 font-bold">gh</span> .git/</p>
                </div>
                
                {terminalState === 1 && (
                   <div className="mt-2 text-zinc-500">
                      <span className="text-zinc-600 mr-2">Jump to:</span>
                      <span className="text-blue-400 font-bold">{typedText}</span><span className="animate-pulse">_</span>
                   </div>
                )}
              </div>
            )}

            {terminalState === 2 && (
              <div className="animate-in fade-in duration-300">
                 <p className="mt-4"><span className="text-green-400">➜</span> <span className="text-blue-400">~/src</span> <span className="animate-pulse">_</span></p>
              </div>
            )}
          </div>
        </div>
        
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[120%] h-[120%] bg-primary/5 blur-[100px] -z-10 pointer-events-none rounded-full" />
      </div>
    </section>
  )
}
