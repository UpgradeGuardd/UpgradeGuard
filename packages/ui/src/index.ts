// Shared UI components for UpdrageGuard frontend apps.
//
// Components defined here can be consumed by `apps/web` and any future
// frontend packages via the pnpm workspace.

import { ReactNode } from "react";

export function Badge({ children }: { children: ReactNode }) {
  return (
    <span className="inline-flex items-center rounded-full border border-slate-700 bg-slate-800 px-3 py-1 text-xs font-medium text-slate-200">
      {children}
    </span>
  );
}
