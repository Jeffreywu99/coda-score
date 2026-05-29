import { useEffect } from "react";

interface Options {
  onClassify?: () => void;
  onClear?: () => void;
  onCompile?: () => void;
}

export function useKeyboardShortcuts({ onClear, onCompile }: Options) {
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      // Ignore if user is typing in an input
      const tag = (e.target as HTMLElement).tagName;
      if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return;

      // Space = clear
      if (e.key === " ") {
        e.preventDefault();
        onClear?.();
        return;
      }

      // Enter = compile
      if (e.key === "Enter") {
        e.preventDefault();
        onCompile?.();
        return;
      }
    }

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [onClear, onCompile]);
}
