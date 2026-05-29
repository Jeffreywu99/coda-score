import { useState, useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import {
  X,
  Eye,
  EyeOff,
  Loader2,
  CheckCircle2,
  AlertCircle,
  Key,
  Globe,
  Cpu,
} from "lucide-react";
import { getAIConfig, saveAIConfig, testAIConnection } from "../../lib/ai";
import type { AiConfigDisplay } from "../../lib/ai";

interface SettingsDialogProps {
  open: boolean;
  onClose: () => void;
}

export function SettingsDialog({ open, onClose }: SettingsDialogProps) {
  const { t } = useTranslation();
  const [apiKey, setApiKey] = useState("");
  const [baseUrl, setBaseUrl] = useState("https://api.deepseek.com");
  const [model, setModel] = useState("deepseek-chat");
  const [showKey, setShowKey] = useState(false);
  const [source, setSource] = useState<string>("none");
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{
    success: boolean;
    message: string;
  } | null>(null);
  const [saveResult, setSaveResult] = useState<{
    success: boolean;
    message: string;
  } | null>(null);
  const overlayRef = useRef<HTMLDivElement>(null);

  // Load current config when dialog opens
  useEffect(() => {
    if (!open) return;
    setTestResult(null);
    setSaveResult(null);
    setLoading(true);
    getAIConfig()
      .then((config: AiConfigDisplay) => {
        setApiKey(config.apiKey);
        setBaseUrl(config.baseUrl);
        setModel(config.model);
        setSource(config.source);
      })
      .catch(() => {
        // Not in Tauri environment, use defaults
        setSource("none");
      })
      .finally(() => setLoading(false));
  }, [open]);

  function handleOverlayClick(e: React.MouseEvent) {
    if (e.target === overlayRef.current) {
      onClose();
    }
  }

  async function handleSave() {
    setSaving(true);
    setSaveResult(null);
    try {
      await saveAIConfig(apiKey, baseUrl, model);
      setSource("file");
      setSaveResult({
        success: true,
        message: t("settings.saved") as string,
      });
      setTimeout(() => setSaveResult(null), 2500);
    } catch (err) {
      setSaveResult({
        success: false,
        message: err instanceof Error ? err.message : String(err),
      });
    } finally {
      setSaving(false);
    }
  }

  async function handleTest() {
    setTesting(true);
    setTestResult(null);
    try {
      // Save first, then test
      if (apiKey.trim()) {
        await saveAIConfig(apiKey, baseUrl, model);
      }
      const msg = await testAIConnection();
      setTestResult({ success: true, message: msg });
    } catch (err) {
      setTestResult({
        success: false,
        message: err instanceof Error ? err.message : String(err),
      });
    } finally {
      setTesting(false);
    }
  }

  if (!open) return null;

  const isFromEnv = source === "env";

  return (
    <div
      ref={overlayRef}
      onClick={handleOverlayClick}
      style={{
        position: "fixed",
        inset: 0,
        zIndex: 1000,
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        backgroundColor: "rgba(0,0,0,0.3)",
        backdropFilter: "blur(2px)",
      }}
    >
      <div
        style={{
          width: 440,
          maxHeight: "80vh",
          backgroundColor: "var(--color-surface)",
          borderRadius: 12,
          boxShadow: "0 8px 32px rgba(0,0,0,0.12)",
          overflow: "hidden",
          display: "flex",
          flexDirection: "column",
        }}
      >
        {/* Header */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            padding: "16px 20px",
            borderBottom: "1px solid var(--color-divider)",
          }}
        >
          <span
            style={{
              fontSize: 15,
              fontWeight: 600,
              color: "var(--color-ink)",
            }}
          >
            {t("settings.title")}
          </span>
          <button
            onClick={onClose}
            style={{
              display: "flex",
              padding: 4,
              borderRadius: 6,
              border: "none",
              backgroundColor: "transparent",
              color: "var(--color-ink-muted)",
              cursor: "pointer",
            }}
          >
            <X size={16} />
          </button>
        </div>

        {/* Body */}
        <div style={{ padding: 20, display: "flex", flexDirection: "column", gap: 16, overflowY: "auto" }}>
          {loading ? (
            <div
              style={{
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                padding: 40,
                color: "var(--color-ink-muted)",
              }}
            >
              <Loader2 size={20} className="animate-spin" />
            </div>
          ) : (
            <>
              {/* Source badge */}
              {isFromEnv && (
                <div
                  style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 6,
                    padding: "8px 12px",
                    fontSize: 12,
                    borderRadius: 6,
                    backgroundColor: "rgba(196,149,106,0.08)",
                    color: "var(--color-accent-deep)",
                    border: "1px solid rgba(196,149,106,0.2)",
                  }}
                >
                  <AlertCircle size={13} />
                  {t("settings.envNotice")}
                </div>
              )}

              {/* API Key */}
              <FieldGroup
                icon={<Key size={14} />}
                label={t("settings.apiKey")}
                hint={isFromEnv ? t("settings.fromEnv") : undefined}
              >
                <div style={{ position: "relative" }}>
                  <input
                    type={showKey ? "text" : "password"}
                    value={apiKey}
                    onChange={(e) => setApiKey(e.target.value)}
                    placeholder="sk-..."
                    disabled={isFromEnv}
                    style={{
                      width: "100%",
                      padding: "8px 36px 8px 12px",
                      fontSize: 13,
                      borderRadius: 8,
                      border: "1px solid var(--color-divider)",
                      backgroundColor: isFromEnv
                        ? "var(--color-parchment)"
                        : "var(--color-surface)",
                      color: "var(--color-ink)",
                      outline: "none",
                      fontFamily: "'SF Mono', 'Cascadia Code', 'Fira Code', 'Consolas', 'Courier New', 'JetBrains Mono', monospace",
                      boxSizing: "border-box",
                    }}
                  />
                  <button
                    onClick={() => setShowKey(!showKey)}
                    disabled={isFromEnv}
                    style={{
                      position: "absolute",
                      right: 8,
                      top: "50%",
                      transform: "translateY(-50%)",
                      display: "flex",
                      padding: 2,
                      border: "none",
                      backgroundColor: "transparent",
                      color: "var(--color-ink-muted)",
                      cursor: isFromEnv ? "default" : "pointer",
                      opacity: isFromEnv ? 0.4 : 0.7,
                    }}
                  >
                    {showKey ? <EyeOff size={14} /> : <Eye size={14} />}
                  </button>
                </div>
              </FieldGroup>

              {/* Base URL */}
              <FieldGroup
                icon={<Globe size={14} />}
                label={t("settings.baseUrl")}
              >
                <input
                  type="text"
                  value={baseUrl}
                  onChange={(e) => setBaseUrl(e.target.value)}
                  placeholder="https://api.deepseek.com"
                  style={{
                    width: "100%",
                    padding: "8px 12px",
                    fontSize: 13,
                    borderRadius: 8,
                    border: "1px solid var(--color-divider)",
                    backgroundColor: "var(--color-surface)",
                    color: "var(--color-ink)",
                    outline: "none",
                    fontFamily: "'SF Mono', 'Cascadia Code', 'Fira Code', 'Consolas', 'Courier New', 'JetBrains Mono', monospace",
                    boxSizing: "border-box",
                  }}
                />
              </FieldGroup>

              {/* Model */}
              <FieldGroup
                icon={<Cpu size={14} />}
                label={t("settings.model")}
              >
                <input
                  type="text"
                  value={model}
                  onChange={(e) => setModel(e.target.value)}
                  placeholder="deepseek-chat"
                  style={{
                    width: "100%",
                    padding: "8px 12px",
                    fontSize: 13,
                    borderRadius: 8,
                    border: "1px solid var(--color-divider)",
                    backgroundColor: "var(--color-surface)",
                    color: "var(--color-ink)",
                    outline: "none",
                    fontFamily: "'SF Mono', 'Cascadia Code', 'Fira Code', 'Consolas', 'Courier New', 'JetBrains Mono', monospace",
                    boxSizing: "border-box",
                  }}
                />
              </FieldGroup>

              {/* Presets */}
              <div>
                <div
                  style={{
                    fontSize: 11,
                    color: "var(--color-ink-muted)",
                    marginBottom: 6,
                  }}
                >
                  {t("settings.presets")}
                </div>
                <div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
                  <PresetButton
                    label="DeepSeek"
                    onClick={() => {
                      setBaseUrl("https://api.deepseek.com");
                      setModel("deepseek-chat");
                    }}
                  />
                  <PresetButton
                    label="OpenAI"
                    onClick={() => {
                      setBaseUrl("https://api.openai.com/v1");
                      setModel("gpt-4o");
                    }}
                  />
                  <PresetButton
                    label="Ollama"
                    onClick={() => {
                      setBaseUrl("http://localhost:11434/v1");
                      setModel("llama3");
                    }}
                  />
                  <PresetButton
                    label="SiliconFlow"
                    onClick={() => {
                      setBaseUrl("https://api.siliconflow.cn/v1");
                      setModel("deepseek-ai/DeepSeek-V3");
                    }}
                  />
                </div>
              </div>

              {/* Test result */}
              {testResult && (
                <div
                  style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 6,
                    padding: "8px 12px",
                    fontSize: 12,
                    borderRadius: 6,
                    backgroundColor: testResult.success
                      ? "rgba(122,139,94,0.08)"
                      : "rgba(196,106,90,0.08)",
                    color: testResult.success
                      ? "var(--color-success)"
                      : "var(--color-danger)",
                    border: `1px solid ${testResult.success ? "rgba(122,139,94,0.2)" : "rgba(196,106,90,0.2)"}`,
                  }}
                >
                  {testResult.success ? (
                    <CheckCircle2 size={13} />
                  ) : (
                    <AlertCircle size={13} />
                  )}
                  <span style={{ wordBreak: "break-all" }}>
                    {testResult.message}
                  </span>
                </div>
              )}

              {/* Save result */}
              {saveResult && (
                <div
                  style={{
                    display: "flex",
                    alignItems: "center",
                    gap: 6,
                    padding: "8px 12px",
                    fontSize: 12,
                    borderRadius: 6,
                    backgroundColor: saveResult.success
                      ? "rgba(122,139,94,0.08)"
                      : "rgba(196,106,90,0.08)",
                    color: saveResult.success
                      ? "var(--color-success)"
                      : "var(--color-danger)",
                    border: `1px solid ${saveResult.success ? "rgba(122,139,94,0.2)" : "rgba(196,106,90,0.2)"}`,
                  }}
                >
                  {saveResult.success ? (
                    <CheckCircle2 size={13} />
                  ) : (
                    <AlertCircle size={13} />
                  )}
                  {saveResult.message}
                </div>
              )}
            </>
          )}
        </div>

        {/* Footer */}
        {!loading && (
          <div
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "flex-end",
              gap: 8,
              padding: "12px 20px",
              borderTop: "1px solid var(--color-divider)",
            }}
          >
            <button
              onClick={handleTest}
              disabled={testing || !apiKey.trim()}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 6,
                padding: "7px 14px",
                fontSize: 13,
                fontWeight: 500,
                borderRadius: 8,
                border: "1px solid var(--color-divider)",
                backgroundColor: "var(--color-surface)",
                color:
                  testing || !apiKey.trim()
                    ? "var(--color-ink-muted)"
                    : "var(--color-ink)",
                cursor:
                  testing || !apiKey.trim() ? "default" : "pointer",
                opacity: testing || !apiKey.trim() ? 0.5 : 1,
                transition: "all 0.15s ease",
              }}
            >
              {testing ? (
                <Loader2 size={13} className="animate-spin" />
              ) : (
                <Globe size={13} />
              )}
              {t("settings.testConnection")}
            </button>
            <button
              onClick={handleSave}
              disabled={saving}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 6,
                padding: "7px 18px",
                fontSize: 13,
                fontWeight: 600,
                borderRadius: 8,
                border: "none",
                backgroundColor: saving
                  ? "var(--color-divider)"
                  : "var(--color-accent)",
                color: saving ? "var(--color-ink-muted)" : "#fff",
                cursor: saving ? "default" : "pointer",
                transition: "all 0.15s ease",
              }}
            >
              {saving ? (
                <Loader2 size={13} className="animate-spin" />
              ) : (
                <CheckCircle2 size={13} />
              )}
              {t("settings.save")}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

function FieldGroup({
  icon,
  label,
  hint,
  children,
}: {
  icon: React.ReactNode;
  label: string;
  hint?: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 5,
          fontSize: 12,
          fontWeight: 500,
          color: "var(--color-ink)",
          marginBottom: 6,
        }}
      >
        <span style={{ color: "var(--color-ink-muted)" }}>{icon}</span>
        {label}
        {hint && (
          <span
            style={{
              fontSize: 10,
              color: "var(--color-accent)",
              fontWeight: 400,
              marginLeft: 4,
            }}
          >
            ({hint})
          </span>
        )}
      </div>
      {children}
    </div>
  );
}

function PresetButton({
  label,
  onClick,
}: {
  label: string;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      style={{
        padding: "4px 10px",
        fontSize: 11,
        fontWeight: 500,
        borderRadius: 5,
        border: "1px solid var(--color-divider)",
        backgroundColor: "var(--color-surface)",
        color: "var(--color-ink-muted)",
        cursor: "pointer",
        transition: "all 0.15s ease",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.borderColor = "var(--color-accent)";
        e.currentTarget.style.color = "var(--color-accent-deep)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.borderColor = "var(--color-divider)";
        e.currentTarget.style.color = "var(--color-ink-muted)";
      }}
    >
      {label}
    </button>
  );
}
