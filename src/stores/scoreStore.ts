import { create } from "zustand";
import type {
  CompileStatus,
  CompileError,
  ReviewViolation,
} from "../types/lilypond";

interface ScoreState {
  lilypondCode: string;
  aiInput: string;
  compileStatus: CompileStatus;
  previewImage: string;
  previewFormat: string;
  compileErrors: CompileError[];
  lilypondVersion: string;
  lastCompileTime: number;
  isOnline: boolean;
  isAIGenerating: boolean;
  selectedTemplate: string;
  autoCompile: number;
  reviewWarnings: ReviewViolation[];

  setLilypondCode: (code: string) => void;
  setAiInput: (input: string) => void;
  setCompileStatus: (status: CompileStatus) => void;
  setPreviewImage: (base64: string, format: string) => void;
  setCompileErrors: (errors: CompileError[]) => void;
  setLilypondVersion: (version: string) => void;
  setLastCompileTime: (ms: number) => void;
  setIsOnline: (online: boolean) => void;
  setIsAIGenerating: (generating: boolean) => void;
  triggerAutoCompile: () => void;
  clearAutoCompile: () => void;
  setSelectedTemplate: (templateId: string) => void;
  setReviewWarnings: (warnings: ReviewViolation[]) => void;
  clearReviewWarnings: () => void;
  clear: () => void;
}

const initialState = {
  lilypondCode: "",
  aiInput: "",
  compileStatus: "IDLE" as CompileStatus,
  previewImage: "",
  previewFormat: "png",
  compileErrors: [] as CompileError[],
  lilypondVersion: "",
  lastCompileTime: 0,
  isOnline: navigator.onLine,
  isAIGenerating: false,
  selectedTemplate: "",
  autoCompile: 0,
  reviewWarnings: [] as ReviewViolation[],
};

export const useScoreStore = create<ScoreState>((set) => ({
  ...initialState,
  setLilypondCode: (code) => set({ lilypondCode: code }),
  setAiInput: (input) => set({ aiInput: input }),
  setCompileStatus: (status) => set({ compileStatus: status }),
  setPreviewImage: (base64, format) =>
    set({ previewImage: base64, previewFormat: format }),
  setCompileErrors: (errors) => set({ compileErrors: errors }),
  setLilypondVersion: (version) => set({ lilypondVersion: version }),
  setLastCompileTime: (ms) => set({ lastCompileTime: ms }),
  setIsOnline: (online) => set({ isOnline: online }),
  setIsAIGenerating: (generating) => set({ isAIGenerating: generating }),
  setSelectedTemplate: (templateId) => set({ selectedTemplate: templateId }),
  setReviewWarnings: (warnings) => set({ reviewWarnings: warnings }),
  clearReviewWarnings: () => set({ reviewWarnings: [] }),
  triggerAutoCompile: () => set((s) => ({ autoCompile: s.autoCompile + 1 })),
  clearAutoCompile: () => set({ autoCompile: 0 }),
  clear: () => set(initialState),
}));
