import React, { useState, useEffect } from "react";
import { ConnectionDialog } from "./components/ConnectionDialog";
import { Toolbar } from "./components/Toolbar";
import { initDecoder } from "./hooks/decoder";
import "./App.css";

function App() {
  const [connected, setConnected] = useState(false);
  const [decoderReady, setDecoderReady] = useState(false);

  useEffect(() => {
    // Initialize FFmpeg WASM decoder on startup
    initDecoder().then(() => {
      setDecoderReady(true);
      console.log("Decoder initialized");
    });
  }, []);

  const handleConnect = (ip: string, port: number) => {
    console.log(`Connecting to ${ip}:${port}`);
    setConnected(true);
  };

  const handleDisconnect = () => {
    setConnected(false);
  };

  const handleFullscreen = () => {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  };

  return (
    <div className="container">
      {!connected ? (
        <ConnectionDialog onConnect={handleConnect} />
      ) : (
        <div style={{ width: "100vw", height: "100vh", position: "relative", backgroundColor: "#000" }}>
          <Toolbar onDisconnect={handleDisconnect} onFullscreen={handleFullscreen} />
          {/* WebGL Canvas will go here in Agent 4.2 */}
          <div style={{ color: "white", textAlign: "center", paddingTop: "50vh" }}>
            {decoderReady ? "Decoder Ready. Waiting for stream..." : "Initializing Decoder..."}
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
