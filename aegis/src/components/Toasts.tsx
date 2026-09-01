import { useEffect } from "react";
import { useAppStore } from "@/store/useAppStore";

export function Toasts() {
  const { toasts, dismissToast } = useAppStore();

  useEffect(() => {
    const timers = toasts.map((t) => setTimeout(() => dismissToast(t.id), 4000));
    return () => timers.forEach(clearTimeout);
  }, [toasts, dismissToast]);

  return (
    <div className="toast-stack">
      {toasts.map((t) => (
        <div key={t.id} className="toast" onClick={() => dismissToast(t.id)}>
          {t.message}
        </div>
      ))}
    </div>
  );
}
