import { useEffect } from "react";
import { HashRouter, Route, Routes } from "react-router-dom";
import { Sidebar } from "@/components/Sidebar";
import { Topbar } from "@/components/Topbar";
import { CommandPalette } from "@/components/CommandPalette";
import { Toasts } from "@/components/Toasts";
import { useAppStore } from "@/store/useAppStore";
import { dirForLanguage } from "@/i18n";

import Dashboard from "@/pages/Dashboard";
import NetworkMap from "@/pages/NetworkMap";
import Devices from "@/pages/Devices";
import Timeline from "@/pages/Timeline";
import Flows from "@/pages/Flows";
import Logs from "@/pages/Logs";
import Incidents from "@/pages/Incidents";
import RulesStudio from "@/pages/RulesStudio";
import FileIntegrity from "@/pages/FileIntegrity";
import AiAnalyst from "@/pages/AiAnalyst";
import Vault from "@/pages/Vault";
import Settings from "@/pages/Settings";

export default function App() {
  const theme = useAppStore((s) => s.theme);
  const language = useAppStore((s) => s.language);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);

  useEffect(() => {
    document.documentElement.dir = dirForLanguage(language);
    document.documentElement.lang = language;
  }, [language]);

  return (
    <HashRouter>
      <div className="app-shell">
        <Sidebar />
        <div className="main-col">
          <Topbar />
          <div className="content-area">
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/network-map" element={<NetworkMap />} />
              <Route path="/devices" element={<Devices />} />
              <Route path="/timeline" element={<Timeline />} />
              <Route path="/flows" element={<Flows />} />
              <Route path="/logs" element={<Logs />} />
              <Route path="/incidents" element={<Incidents />} />
              <Route path="/rules" element={<RulesStudio />} />
              <Route path="/fim" element={<FileIntegrity />} />
              <Route path="/ai-analyst" element={<AiAnalyst />} />
              <Route path="/vault" element={<Vault />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </div>
        </div>
      </div>
      <CommandPalette />
      <Toasts />
    </HashRouter>
  );
}
