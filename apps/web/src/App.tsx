export default function App() {
  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      <main className="mx-auto max-w-4xl px-6 py-16">
        <h1 className="text-4xl font-bold tracking-tight">
          UpdrageGuard
        </h1>
        <p className="mt-2 text-lg text-slate-400">
          Soroban Smart Contract Upgrade &amp; Migration Safety Platform
        </p>
        <p className="mt-1 text-slate-500">
          Test before you upgrade. Migrate with confidence.
        </p>

        <section className="mt-12 grid gap-6 sm:grid-cols-2">
          {FEATURES.map((feature) => (
            <div
              key={feature.title}
              className="rounded-lg border border-slate-800 bg-slate-900 p-6"
            >
              <h2 className="text-lg font-semibold">{feature.title}</h2>
              <p className="mt-2 text-sm text-slate-400">
                {feature.description}
              </p>
            </div>
          ))}
        </section>
      </main>
    </div>
  );
}

const FEATURES = [
  {
    title: "Contract Version Comparison",
    description:
      "Identify changes between contract versions, including functions, parameters, return types and contract specs.",
  },
  {
    title: "Storage Compatibility Analysis",
    description:
      "Detect incompatible storage structures, changed data types and serialization issues.",
  },
  {
    title: "Migration Simulation",
    description:
      "Test migration procedures against representative existing contract data in an isolated environment.",
  },
  {
    title: "Regression Test Generation",
    description:
      "Generate tests to validate existing functionality, authorization, storage preservation and edge cases.",
  },
  {
    title: "Upgrade Risk Assessment",
    description:
      "Produce reports highlighting compatibility issues, migration failures and potential upgrade risks.",
  },
  {
    title: "CI/CD Integration",
    description:
      "Enable automated upgrade validation through GitHub Actions and other development workflows.",
  },
];
