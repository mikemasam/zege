import { useAuth } from "@/auth/use.auth";
import { useNavigate } from "react-router";

export default function Home() {
  let navigate = useNavigate();
  const auth = useAuth();
  console.log(auth);
  if (auth.loading) {
    return null;
  }
  if (auth.config?.features?.landing === false) {
    navigate("/login");
  }
  return (
    <div className="min-h-screen bg-black text-white">
      <Header />
      <Hero />
      <Features />
      <CTA />
      <Footer />
    </div>
  );
}

function Header() {
  return (
    <header className="sticky top-0 z-50 bg-black/70 backdrop-blur border-b border-white/10">
      <div className="container mx-auto px-6 py-4 flex items-center justify-between">
        <h1 className="text-lg font-semibold tracking-tight">Zege</h1>
        <nav className="flex gap-6 text-sm text-gray-400 items-center">
          <a href="#features" className="hover:text-white ">
            Features
          </a>
          <a href="#install" className="hover:text-white ">
            Install
          </a>
          <a
            href="https://github.com/mikemasam/zege"
            target="_blank"
            className="hover:text-white "
          >
            GitHub
          </a>
          <a
            href="/login"
            className="px-8 py-3 rounded-lg border-white/40 bg-white/15  font-bold"
          >
            Login
          </a>
        </nav>
      </div>
    </header>
  );
}

function Hero() {
  return (
    <section className="relative overflow-hidden">
      {/* ambient glow */}
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_50%_20%,_rgba(56,189,248,0.25),transparent_55%)]" />
      <div className="absolute inset-0 bg-gradient-to-b from-black via-black/80 to-black" />

      <div className="relative container mx-auto px-6 py-32 max-w-5xl">
        <div className="text-center">
          <span className="inline-flex items-center gap-2 mb-6 px-4 py-1.5 rounded-full text-sm bg-white/10 backdrop-blur border border-white/10">
            <span className="h-2 w-2 rounded-full bg-sky-400" />
            Rust-native event logging
          </span>

          <h2 className="text-5xl md:text-6xl lg:text-7xl font-extrabold tracking-tight leading-[1.05] mb-8">
            <span className="block text-white">Event logging</span>
            <span className="block bg-gradient-to-r from-sky-400 via-cyan-300 to-emerald-300 bg-clip-text text-transparent">
              that stays out of your way
            </span>
          </h2>

          <p className="text-lg md:text-xl text-gray-300 leading-relaxed max-w-3xl mx-auto mb-12">
            Zege is a fast, minimal event logging service built in Rust. Drop it
            in front of nginx, web apps, backends, Laravel, Next.js, or anything
            that can send JSON — and get clean, structured events.
          </p>

          <div className="flex justify-center gap-4">
            <a
              href="#install"
              className="px-8 py-3 rounded-lg bg-gradient-to-r from-sky-400 via-cyan-300 to-emerald-300 text-transparent bg-clip-text font-semibold hover:bg-sky-400 "
            >
              Install Zege
            </a>
            <a
              href="https://github.com/mikemasam/zege"
              target="_blank"
              className="px-8 py-3 rounded-lg border-white/20 bg-white/5  font-bold"
            >
              View source
            </a>
            <a
              href="https://github.com/mikemasam/zege"
              target="_blank"
              className="px-8 py-3 rounded-lg border-white/20 bg-white/5  font-bold"
            >
              ⭐ Star on GitHub
            </a>
          </div>
        </div>
      </div>
    </section>
  );
}

const features = [
  {
    title: "Drop-in anywhere",
    desc: "Run Zege behind nginx, alongside web apps, or as a standalone service.",
    icon: "🧩",
  },
  {
    title: "Backend-friendly",
    desc: "Works naturally with APIs, microservices, Laravel, Next.js, and more.",
    icon: "⚙️",
  },
  {
    title: "Fast & predictable",
    desc: "Rust core built for throughput, low overhead, and reliability.",
    icon: "⚡",
  },
];

function Features() {
  return (
    <section id="features" className="py-28 bg-slate-950">
      <div className="container mx-auto px-6 max-w-6xl">
        <h3 className="text-4xl font-bold text-center mb-20">
          Built for real systems
        </h3>

        <div className="grid gap-8 md:grid-cols-3">
          {features.map((f, i) => (
            <div
              key={i}
              className="relative rounded-2xl p-8 bg-white/5 backdrop-blur border border-white/10 hover:border-sky-400/40 "
            >
              <div className="text-3xl mb-4">{f.icon}</div>
              <h4 className="text-xl font-semibold mb-3">{f.title}</h4>
              <p className="text-gray-400 leading-relaxed">{f.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function CTA() {
  return (
    <section id="install" className="relative overflow-hidden py-28">
      <div className="absolute inset-0 bg-gradient-to-br from-sky-500 via-cyan-400 to-emerald-400" />
      <div className="absolute inset-0 bg-black/20" />

      <div className="relative container mx-auto px-6 max-w-4xl text-center text-black">
        <h4 className="text-4xl md:text-5xl font-extrabold mb-8">
          Install Zege. Start shipping events.
        </h4>

        <p className="text-lg mb-12">
          Deploy it behind nginx, plug it into your backend, or run it alongside
          Laravel, Next.js, and other tools. Simple setup, zero lock-in.
        </p>

        <div className="flex flex-col sm:flex-row justify-center gap-4">
          <a
            href="https://github.com/mikemasam/zege#installation"
            target="_blank"
            className="px-10 py-4 rounded-xl bg-black text-white font-semibold hover:bg-slate-900 "
          >
            Installation guide
          </a>
          <a
            href="https://github.com/mikemasam/zege"
            target="_blank"
            className="px-10 py-4 rounded-xl bg-white/90 text-black font-semibold hover:bg-white "
          >
            ⭐ Star on GitHub
          </a>
        </div>
      </div>
    </section>
  );
}

function Footer() {
  return (
    <footer className="py-12 text-center text-sm text-gray-500 bg-black">
      <p>Zege - Apache-2.0 license</p>
    </footer>
  );
}
