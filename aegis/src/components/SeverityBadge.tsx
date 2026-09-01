import { useTranslation } from "react-i18next";
import type { Severity } from "@/types";

export function SeverityBadge({ severity }: { severity: Severity }) {
  const { t } = useTranslation();
  return (
    <span className={`badge badge-${severity}`}>
      <span className="badge-dot" />
      {t(`severity.${severity}`)}
    </span>
  );
}
