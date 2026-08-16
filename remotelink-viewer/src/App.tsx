import React, { useState, useEffect } from "react";
import { ConnectionDialog } from "./components/ConnectionDialog";
import { Toolbar } from "./components/Toolbar";
import { VideoCanvas } from "./components/VideoCanvas";
import { initDecoder } from "./hooks/decoder";
import "./App.css";

function App() {
  const [connected, setConnected] = useState(false);
  const [decoderReady, setDecoderReady] = useState(false);

  useEffect(() => {
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
          {decoderReady ? (
             <VideoCanvas />
          ) : (
            <div style={{ color: "white", textAlign: "center", paddingTop: "50vh" }}>
              Initializing Decoder...
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default App;
