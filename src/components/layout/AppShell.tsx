interface AppShellProps {
  children: React.ReactNode;
}

export function AppShell({ children }: AppShellProps) {
  return (
    <div style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
      <main style={{ flex: 1, overflow: "hidden", padding: "24px 32px" }}>
        {children}
      </main>
    </div>
  );
}
