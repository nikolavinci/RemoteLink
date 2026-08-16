import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  onConnect: (ip: string, port: number) => void;
}

export const ConnectionDialog: React.FC<Props> = ({ onConnect }) => {
  const [ip, setIp] = useState("127.0.0.1");
  const [port, setPort] = useState("5900");
  const [connecting, setConnecting] = useState(false);

  const handleConnect = async (e: React.FormEvent) => {
    e.preventDefault();
    setConnecting(true);
    try {
      // Stub to Tauri command
      // await invoke("connect_to_host", { ip, port: parseInt(port) });
      onConnect(ip, parseInt(port));
    } catch (err) {
      console.error(err);
    } finally {
      setConnecting(false);
    }
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center", justifyContent: "center", height: "100vh" }}>
      <h2>Connect to RemoteLink Host</h2>
      <form onSubmit={handleConnect} style={{ display: "flex", flexDirection: "column", gap: "1rem" }}>
        <input 
          type="text" 
          value={ip} 
          onChange={(e) => setIp(e.target.value)} 
          placeholder="Host IP Address" 
        />
        <input 
          type="number" 
          value={port} 
          onChange={(e) => setPort(e.target.value)} 
          placeholder="Port" 
        />
        <button type="submit" disabled={connecting}>
          {connecting ? "Connecting..." : "Connect"}
        </button>
      </form>
    </div>
  );
};
