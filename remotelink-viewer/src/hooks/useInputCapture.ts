import { useEffect, useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getVkCode } from "../utils/keys";

interface InputCaptureOptions {
  canvasRef: React.RefObject<HTMLCanvasElement | null>;
  enabled: boolean;
}

export const useInputCapture = ({ canvasRef, enabled }: InputCaptureOptions) => {
  const [cursorPos, setCursorPos] = useState({ x: 0, y: 0 });

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!enabled || !canvasRef.current) return;
    const rect = canvasRef.current.getBoundingClientRect();
    const x = Math.round(((e.clientX - rect.left) / rect.width) * 1920);
    const y = Math.round(((e.clientY - rect.top) / rect.height) * 1080);
    
    setCursorPos({ x: e.clientX, y: e.clientY });

    // Try sending to Tauri
    invoke("send_mouse_move", { x, y }).catch(console.error);
  }, [enabled, canvasRef]);

  const handleMouseClick = useCallback((e: MouseEvent) => {
    if (!enabled) return;
    const buttonMap: Record<number, string> = { 0: "left", 1: "middle", 2: "right" };
    const btn = buttonMap[e.button];
    if (btn) {
      invoke("send_mouse_click", { button: btn, down: e.type === "mousedown" }).catch(console.error);
    }
  }, [enabled]);

  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    if (!enabled) return;
    e.preventDefault();
    const vk = getVkCode(e.code);
    if (vk !== null) {
      invoke("send_key_event", { vk, down: true }).catch(console.error);
    }
  }, [enabled]);

  const handleKeyUp = useCallback((e: KeyboardEvent) => {
    if (!enabled) return;
    e.preventDefault();
    const vk = getVkCode(e.code);
    if (vk !== null) {
      invoke("send_key_event", { vk, down: false }).catch(console.error);
    }
  }, [enabled]);

  useEffect(() => {
    if (!enabled) return;
    
    // Attach event listeners to window to catch everything while connected
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mousedown", handleMouseClick);
    window.addEventListener("mouseup", handleMouseClick);
    window.addEventListener("keydown", handleKeyDown, { passive: false });
    window.addEventListener("keyup", handleKeyUp, { passive: false });
    
    // Prevent context menu
    const preventContextMenu = (e: Event) => e.preventDefault();
    window.addEventListener("contextmenu", preventContextMenu);

    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mousedown", handleMouseClick);
      window.removeEventListener("mouseup", handleMouseClick);
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
      window.removeEventListener("contextmenu", preventContextMenu);
    };
  }, [enabled, handleMouseMove, handleMouseClick, handleKeyDown, handleKeyUp]);

  return { cursorPos };
};
