import { Terminal } from "lucide-react"
import { Link } from "react-router-dom"

export function Header() {
    return (
	<header className="sticky top-0 z-50 w-full border-b border-zinc-800 bg-zinc-950/95 backdrop-blur-lg">
	    <div className="container flex h-14 items-center justify-between px-6 mx-auto max-w-7xl">
		<Link to="/" className="flex items-center gap-2 font-bold text-zinc-100 text-base">
		    <Terminal className="h-5 w-5" />
		    <span>jump</span>
		</Link>
		<nav className="flex items-center gap-6">
		    <a
			href="https://github.com/JoseMaurette1/jump"
			target="_blank"
			rel="noopener noreferrer"
			className="text-sm text-zinc-400 hover:text-zinc-100 transition-colors"
		    >
			GitHub
		    </a>
		    <Link
			to="/docs/introduction"
			className="text-sm text-zinc-400 hover:text-zinc-100 transition-colors"
		    >
			Docs
		    </Link>
		</nav>
	    </div>
	</header>
    )
}
