import React, { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Alert } from "../components/ui/Alert";
import { Button } from "../components/ui/Button";
import ProgressBar from "../components/shared/ProgressBar";

interface Progress {
  done: number;
  total: number;
}

/** IA local leve: baixa o llama-server + modelo uma vez e funciona sem internet nem API key. */
export const LocalAiPanel: React.FC = () => {
  const { t } = useTranslation();
  const [installed, setInstalled] = useState<boolean | null>(null);
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState<Progress | null>(null);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {
    invoke<boolean>("local_ai_installed")
      .then(setInstalled)
      .catch(() => setInstalled(false));
  }, []);

  useEffect(() => {
    refresh();
    const unlisten = listen<Progress>("local-ai-progress", (e) =>
      setProgress(e.payload),
    );
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [refresh]);

  const install = async () => {
    setBusy(true);
    setError(null);
    try {
      await invoke("install_local_ai");
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
      setProgress(null);
      refresh();
    }
  };

  const percent =
    progress && progress.total > 0
      ? Math.round((progress.done / progress.total) * 100)
      : 0;

  return (
    <div className="flex flex-col gap-2 p-4">
      <p className="text-sm text-mid-gray">
        {t("vozprompt.local.description")}
      </p>
      {installed ? (
        <Alert variant="success" contained>
          {t("vozprompt.local.ready")}
        </Alert>
      ) : (
        <Button onClick={install} disabled={busy || installed === null}>
          {busy
            ? t("vozprompt.local.downloading", { percent })
            : t("vozprompt.local.download")}
        </Button>
      )}
      {busy && progress && progress.total > 0 && (
        <ProgressBar progress={[{ id: "local-ai", percentage: percent }]} />
      )}
      {error && (
        <Alert variant="error" contained>
          {error}
        </Alert>
      )}
    </div>
  );
};
