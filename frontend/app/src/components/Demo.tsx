import { JumpDemo } from "./docs/JumpDemo"

export function Demo() {
  return (
    <section className="px-6 py-24 max-w-7xl mx-auto">
      <div className="max-w-4xl mx-auto">
        <div className="relative">
          <JumpDemo />
          <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[120%] h-[120%] bg-primary/5 blur-[100px] -z-10 pointer-events-none rounded-full" />
        </div>
      </div>
    </section>
  )
}
