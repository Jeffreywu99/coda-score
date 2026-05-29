interface TauriGlobal {
  core: {
    invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>;
  };
}

function getTauri(): TauriGlobal | undefined {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return (window as any).__TAURI__;
}

export async function writeFile(
  path: string,
  contents: number[]
): Promise<void> {
  const tauri = getTauri();
  if (!tauri) throw new Error("Tauri not available");
  return tauri.core.invoke("write_file", { path, contents }) as Promise<void>;
}
