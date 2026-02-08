import { Terminal } from "lucide-react"
import { Link } from "react-router-dom"

export function Header() {
    return (
	<header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
	    <div className="container flex h-14 items-center justify-between px-4 md:px-6 mx-auto">
		<Link to="/" className="flex items-center gap-2 font-bold">
		    <Terminal className="h-6 w-6" />
		    <span>jump</span>
		</Link>
		<nav className="flex items-center gap-8">
		    <a
			href="https://github.com/JoseMaurette1/jump"
			target="_blank"
			rel="noopener noreferrer"
			className="text-sm font-medium hover:underline decoration-gray-400 underline-offset-4"
		    >
			GitHub
		    </a>
		    <Link
			to="/docs/introduction"
			className="text-sm font-medium hover:underline decoration-gray-400 underline-offset-4"
		    >
			Docs
		    </Link>
		</nav>
	    </div>
	</header>
    )
}
