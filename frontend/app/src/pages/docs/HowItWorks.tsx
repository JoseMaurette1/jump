import { JumpDemo } from "@/components/docs/JumpDemo"

export function HowItWorks() {
  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          How it Works
        </h1>
        <p className="text-lg text-muted-foreground">
          See jump in action.
        </p>
      </div>
      
      <div className="mt-8 mb-8">
        <JumpDemo />
      </div>

      <div className="space-y-4">
        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          The Concept
        </h2>
        <p className="leading-7">
          Terminal navigation typically involves typing directory names (fuzzy finders) or long paths. <strong>jump</strong> takes a different approach inspired by browser plugins like Vimium.
        </p>
        <p className="leading-7">
            When you trigger `jump`:
        </p>
        <ol className="list-decimal pl-6 space-y-2">
            <li>It scans the current directory.</li>
            <li>It assigns a <strong>2-letter label</strong> (like AS, AD, AF) to each directory.</li>
            <li>You type the label to jump there instantly.</li>
        </ol>
      </div>
    </div>
  )
}
