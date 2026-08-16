import React from "react";

interface Props {
  onDisconnect: () => void;
  onFullscreen: () => void;
}

export const Toolbar: React.FC<Props> = ({ onDisconnect, onFullscreen }) => {
  return (
    <div style={{ 
      position: "absolute", 
      top: 0, 
      left: "50%", 
      transform: "translateX(-50%)", 
      backgroundColor: "#333", 
      color: "#fff",
      padding: "8px 16px",
      borderRadius: "0 0 8px 8px",
      display: "flex",
      gap: "12px",
      zIndex: 1000,
    }}>
      <button onClick={onFullscreen}>Fullscreen</button>
      <button onClick={onDisconnect}>Disconnect</button>
    </div>
  );
};
