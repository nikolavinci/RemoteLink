import React, { useEffect, useRef, useState } from 'react';
import { useInputCapture } from '../hooks/useInputCapture';

const VS_SOURCE = `
  attribute vec2 a_position;
  attribute vec2 a_texCoord;
  varying vec2 v_texCoord;
  void main() {
    gl_Position = vec4(a_position, 0, 1);
    v_texCoord = a_texCoord;
  }
`;

const FS_SOURCE = `
  precision mediump float;
  varying vec2 v_texCoord;
  uniform sampler2D u_y;
  uniform sampler2D u_u;
  uniform sampler2D u_v;

  void main() {
    float y = texture2D(u_y, v_texCoord).r;
    float u = texture2D(u_u, v_texCoord).r - 0.5;
    float v = texture2D(u_v, v_texCoord).r - 0.5;
    
    y = 1.1643 * (y - 0.0625);
    float r = y + 1.5958 * v;
    float g = y - 0.39173 * u - 0.81290 * v;
    float b = y + 2.017 * u;
    
    gl_FragColor = vec4(r, g, b, 1.0);
  }
`;

interface Props {
  width?: number;
  height?: number;
}

export const VideoCanvas: React.FC<Props> = ({ width = 1920, height = 1080 }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [fps, setFps] = useState(0);
  
  // Attach input capture
  const { cursorPos } = useInputCapture({ canvasRef, enabled: true });

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const gl = canvas.getContext('webgl');
    if (!gl) {
      console.error('WebGL not supported');
      return;
    }

    gl.clearColor(0.1, 0.1, 0.2, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    let frameCount = 0;
    let lastTime = performance.now();
    let animationId: number;

    const renderLoop = () => {
      frameCount++;
      const now = performance.now();
      if (now - lastTime >= 1000) {
        setFps(frameCount);
        frameCount = 0;
        lastTime = now;
      }
      
      // In full implementation, we'd bind textures and drawArrays here.
      gl.clear(gl.COLOR_BUFFER_BIT);

      animationId = requestAnimationFrame(renderLoop);
    };

    renderLoop();

    return () => cancelAnimationFrame(animationId);
  }, []);

  return (
    <div style={{ position: 'relative', width: '100%', height: '100%', display: 'flex', justifyContent: 'center', alignItems: 'center', cursor: 'none' }}>
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        style={{
          maxWidth: '100%',
          maxHeight: '100%',
          objectFit: 'contain'
        }}
      />
      
      {/* Local Echo Cursor Prediction */}
      <div style={{
        position: 'absolute',
        top: 0,
        left: 0,
        transform: `translate(${cursorPos.x}px, ${cursorPos.y}px)`,
        pointerEvents: 'none',
        zIndex: 10,
        width: '20px',
        height: '20px',
        // Simple CSS cursor triangle for echo visualization
        background: 'url(data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="white" stroke="black" stroke-width="2"><polygon points="3,3 10,21 14,14 21,10"></polygon></svg>)',
      }} />

      <div style={{
        position: 'absolute',
        top: '10px',
        right: '10px',
        background: 'rgba(0,0,0,0.5)',
        color: '#0f0',
        padding: '4px 8px',
        fontFamily: 'monospace',
        borderRadius: '4px',
      }}>
        {fps} FPS
      </div>
    </div>
  );
};
