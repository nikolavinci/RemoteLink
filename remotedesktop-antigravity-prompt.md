# RemoteLink Minimal: Antigravity Build Prompt

**Project:** Personal P2P Remote Desktop
**Target:** Windows desktop control from another Windows machine (LAN / SSH tunnel)
**Scope:** Indefinite connection, mouse/keyboard/clipboard, no backend infrastructure
**Developer:** Anil (solo build with Claude Code agents)
**Framework:** Antigravity (React/Tauri frontend, Rust host agent)
**Timeline:** 6–8 weeks with multi-agent task decomposition

---

## 1. Project Vision & Constraints

**Vision Statement:**
Build a minimal, fast, direct remote desktop controller that lets you see and interact with another Windows machine over a trusted network connection. No pairing UI, no backend servers, no audit logs—just install, connect by IP, and control.

**Core Pillars:**
- **Direct P2P only** — TCP connection; SSH tunnel or LAN direct
- **Minimal scope** — Screen, mouse, keyboard, clipboard (text only)
- **No infrastructure** — Self-contained agents; zero backend cost
- **Personal use** — Indefinite access, trusted users only
- **Antigravity aesthetic** — Clean, polished, responsive UI/UX

**Hard Constraints:**
- Windows primary target (macOS/Linux future-friendly but not MVP)
- No auth system, no device dashboard, no session audit
- No multi-monitor, no audio, no file transfer in MVP
- No WebRTC/STUN/TURN; direct TCP + TLS only
- Single user session per host

**Success Criteria:**
1. Host installs and listens on a port
2. Viewer connects by IP address (or device ID over LAN)
3. Desktop renders at 30 FPS (1080p max)
4. Mouse/keyboard control responds in <100ms
5. Clipboard text syncs bidirectionally
6. Connection survives 8 hours of continuous use
7. Memory footprint <200MB host idle, <150MB viewer idle

---

## 2. Technical Architecture

### 2.1 System Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     RemoteLink MVP                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Host Machine (Windows)                                      │
│  ┌──────────────────────────────────┐                       │
│  │  remotelink-host (Rust binary)   │                       │
│  │                                  │                       │
│  │  ┌────────────────────────────┐  │                       │
│  │  │ Desktop Capture Module     │  │                       │
│  │  │ - DirectX11 DDA API        │  │                       │
│  │  │ - Detect screen changes    │  │                       │
│  │  │ - 30 FPS target            │  │                       │
│  │  └────────────────────────────┘  │                       │
│  │                │                  │                       │
│  │  ┌────────────▼────────────────┐  │                       │
│  │  │ H.264 Encoder Module        │  │                       │
│  │  │ - NVENC (GPU) or libx264    │  │                       │
│  │  │ - Adaptive bitrate          │  │                       │
│  │  │ - Changed-region detection  │  │                       │
│  │  └────────────────────────────┘  │                       │
│  │                │                  │                       │
│  │  ┌────────────▼────────────────┐  │                       │
│  │  │ TCP Server + TLS            │  │                       │
│  │  │ - Listen :5900              │  │                       │
│  │  │ - Self-signed cert + pin    │  │                       │
│  │  │ - Stream H.264 frames       │  │                       │
│  │  └────────────────────────────┘  │                       │
│  │                ▲ ▼                │                       │
│  │  ┌────────────────────────────┐  │                       │
│  │  │ Input Handler              │  │                       │
│  │  │ - Mouse events (move/click)│  │                       │
│  │  │ - Keyboard events          │  │                       │
│  │  │ - Windows SendInput API     │  │                       │
│  │  └────────────────────────────┘  │                       │
│  │                ▲                  │                       │
│  │  ┌────────────┴────────────────┐  │                       │
│  │  │ Clipboard Monitor           │  │                       │
│  │  │ - Poll Windows clipboard    │  │                       │
│  │  │ - Sync with viewer          │  │                       │
│  │  └────────────────────────────┘  │                       │
│  └──────────────────────────────────┘  │                    │
│                                         │                    │
│       ↕ (TLS-encrypted TCP stream)     │                    │
│                                         │                    │
│  Viewer Machine (Windows)               │                    │
│  ┌─────────────────────────────────┐   │                    │
│  │ remotelink-viewer (Tauri)       │   │                    │
│  │                                 │   │                    │
│  │  ┌──────────────────────────┐   │   │                    │
│  │  │ H.264 Decoder Module     │   │   │                    │
│  │  │ - ffmpeg.wasm or libav.js│   │   │                    │
│  │  │ - Decode frames in-place │   │   │                    │
│  │  └──────────────────────────┘   │   │                    │
│  │              │                   │   │                    │
│  │  ┌──────────▼──────────────┐     │   │                    │
│  │  │ WebGL Renderer          │     │   │                    │
│  │  │ - Display video stream  │     │   │                    │
│  │  │ - Render at monitor FPS │     │   │                    │
│  │  └──────────────────────────┘     │   │                    │
│  │              ▲                    │   │                    │
│  │  ┌──────────┴──────────────┐     │   │                    │
│  │  │ Input Capture & Send    │     │   │                    │
│  │  │ - Mouse/keyboard hooks  │     │   │                    │
│  │  │ - Serialize to wire fmt │     │   │                    │
│  │  └──────────────────────────┘     │   │                    │
│  │              ▲                    │   │                    │
│  │  ┌──────────┴──────────────┐     │   │                    │
│  │  │ Clipboard Integration   │     │   │                    │
│  │  │ - Copy/paste events     │     │   │                    │
│  │  └──────────────────────────┘     │   │                    │
│  └─────────────────────────────────┘   │                    │
│                                         │                    │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Technology Stack

**Host Agent:**
- **Language:** Rust
- **Screen Capture:** `windows-rs` + DirectX11 Desktop Duplication API
- **Encoding:** `libx264` (fallback) + `cuvid` / Intel QSV (if available)
- **Networking:** `tokio` + `quinn` for async TCP (or `rustls` for TLS wrapper)
- **Input:** `windows-rs` SendInput API
- **Clipboard:** `clipboard-win` or `windows-rs` directly
- **Binary size:** ~30MB (stripped)

**Viewer:**
- **Framework:** Tauri + React
- **Language:** TypeScript
- **Decoding:** `ffmpeg.wasm` (bundled) or `libav.js`
- **Rendering:** WebGL via `react-three-fiber` or Canvas 2D
- **Input:** Native window events + Tauri IPC
- **Clipboard:** Tauri clipboard API
- **Bundle:** ~80MB installer

**Protocol:**
- **Transport:** TLS 1.3 over TCP
- **Message format:** Minimal binary framing (see Section 4)
- **Encoding:** H.264 Annex-B (raw NALU stream)

---

## 3. Functional Requirements (Prioritized)

### Phase 1: MVP (Weeks 1–6)

#### FR-01: Host Agent Initialization
- [ ] Install to `%APPDATA%\RemoteLink\` on Windows
- [ ] Generate RSA keypair on first run (stored locally)
- [ ] Output device ID (SHA256 of pubkey, first 8 chars)
- [ ] Display listening IP and port on console
- [ ] CLI: `remotelink-host --listen [--port 5900]`

#### FR-02: Desktop Capture
- [ ] Use DirectX11 Desktop Duplication API
- [ ] Capture primary monitor only (MVP)
- [ ] Target 30 FPS, 1920x1080 resolution
- [ ] Detect changed regions to reduce encoding overhead
- [ ] Skip frames if encoding can't keep up (drop rather than queue)
- [ ] Handle screen lock/unlock gracefully
- [ ] Performance target: <10ms capture latency

#### FR-03: H.264 Encoding
- [ ] Use NVIDIA NVENC if GPU available (via CUDA)
- [ ] Fallback to Intel Quick Sync (if CPU has iGPU)
- [ ] Final fallback: libx264 software encoder
- [ ] Adaptive bitrate: 500 kbps (poor) → 8 Mbps (excellent)
- [ ] Adaptive FPS: 15 → 30 based on bandwidth/CPU
- [ ] Keyframe every 2 seconds (IDR frame interval)
- [ ] Performance target: <50ms encode latency at 30 FPS

#### FR-04: TCP Server + TLS
- [ ] Listen on 0.0.0.0:5900 (or configurable port)
- [ ] Generate self-signed cert on first run
- [ ] TLS 1.3 handshake
- [ ] Public key pinning: viewer stores host's pubkey after first connect
- [ ] Accept one connection at a time (reject others)
- [ ] Graceful disconnect handling
- [ ] Connection timeout: 5 min idle → auto-close

#### FR-05: Input Handler (Host)
- [ ] Receive mouse move events; apply to system cursor
- [ ] Receive mouse click events (left, right, middle, double-click)
- [ ] Receive keyboard events (all keys, key combos)
- [ ] Use Windows `SendInput` API for all injections
- [ ] Queue input events; process in order
- [ ] Input latency target: <50ms end-to-end

#### FR-06: Clipboard Sync (Host)
- [ ] Monitor Windows clipboard for changes
- [ ] Poll every 500ms (not a native hook; simpler)
- [ ] Send clipboard text to viewer on change
- [ ] Accept clipboard updates from viewer
- [ ] Text only (no images, no rich formatting in MVP)

#### FR-07: Viewer Connection & Handshake
- [ ] Accept hostname or IP:port as CLI argument
- [ ] Attempt TLS connection; fail gracefully if host unreachable
- [ ] Display "Connecting…" UI during negotiation
- [ ] On success, show "Connected" status
- [ ] On failure, show error message with retry option
- [ ] Connection timeout: 10 seconds

#### FR-08: H.264 Decoding
- [ ] Use `ffmpeg.wasm` for in-browser decoding
- [ ] Decode each arriving frame
- [ ] Handle dropped/corrupted frames gracefully
- [ ] Queue decoded frames for rendering

#### FR-09: WebGL Rendering
- [ ] Render H.264-decoded frames to canvas
- [ ] Full-screen mode: scale to fit window
- [ ] Maintain aspect ratio (1920x1080 by default)
- [ ] Render at monitor refresh rate (60 FPS target)
- [ ] Show FPS counter (debug mode only)
- [ ] Rendering latency target: <16ms (60 FPS)

#### FR-10: Input Capture & Send (Viewer)
- [ ] Capture mouse movement over canvas
- [ ] Detect clicks (left, right, middle, double)
- [ ] Capture keyboard events (all keys)
- [ ] Serialize to wire format
- [ ] Send via TLS to host
- [ ] Local echo: show cursor position immediately (client-side prediction)

#### FR-11: Clipboard Integration (Viewer)
- [ ] Keyboard shortcut: Ctrl+C = copy from remote
- [ ] Keyboard shortcut: Ctrl+V = paste to remote
- [ ] Alternative: button in toolbar
- [ ] Display clipboard content in a small panel (optional)

#### FR-12: Graceful Disconnection
- [ ] Host: detect viewer disconnect; close session; remain listening
- [ ] Viewer: detect host disconnect; show "Connection lost" message
- [ ] Viewer: auto-reconnect checkbox (optional for MVP)
- [ ] Both: clear state on disconnect

---

### Phase 1.5: Polish (Weeks 6–8)

#### FR-13: Adaptive Bitrate
- [ ] Monitor network latency (RTT)
- [ ] Monitor packet loss (estimate via decoder errors)
- [ ] Adjust encoder bitrate based on metrics
- [ ] Strategy: 500 kbps base, +10% per Mbps available BW

#### FR-14: Cursor Optimization
- [ ] Don't stream cursor as part of video frame
- [ ] Send cursor position + shape separately
- [ ] Render cursor client-side from shape metadata
- [ ] Reduces bandwidth by ~5%, improves responsiveness

#### FR-15: Host Tray UI (optional)
- [ ] System tray icon
- [ ] Right-click menu: "Show device ID", "Stop listening", "Quit"
- [ ] Show connection status (idle vs. connected)

#### FR-16: Viewer Toolbar UI
- [ ] Minimal toolbar: [Disconnect] [Settings] [Fullscreen]
- [ ] Auto-hide after 2 seconds of inactivity
- [ ] Display connection stats (FPS, latency, bitrate) in corner

---

## 4. Protocol Specification (Minimal)

### 4.1 Connection Handshake

```
Viewer → Host (TLS)
  [HELLO]
  - version: 1
  - capabilities: [video, input, clipboard]

Host → Viewer (TLS)
  [HELLO_ACK]
  - version: 1
  - screen_width: 1920
  - screen_height: 1080
  - capabilities: [video, input, clipboard]
```

### 4.2 Message Frame Format

All messages sent over TLS are framed as:

```
[type: u8][length: u32][payload: bytes]

type codes:
  0x01 = VIDEO_FRAME
  0x02 = MOUSE_MOVE
  0x03 = MOUSE_CLICK
  0x04 = KEY_EVENT
  0x05 = CLIPBOARD_UPDATE
  0x06 = HEARTBEAT
  0x07 = DISCONNECT
```

### 4.3 Video Frame Format

```
[type: 0x01]
[length: u32]
[timestamp: u64]           // milliseconds since epoch
[frame_type: u8]           // 0 = P-frame, 1 = I-frame
[data: bytes]              // Raw H.264 NALU stream (Annex-B)
```

### 4.4 Input Events

```
MOUSE_MOVE
  [type: 0x02]
  [x: u16]
  [y: u16]

MOUSE_CLICK
  [type: 0x03]
  [button: u8]             // 1=left, 2=right, 3=middle
  [action: u8]             // 0=down, 1=up

KEY_EVENT
  [type: 0x04]
  [key_code: u16]          // Windows VK_* constant
  [action: u8]             // 0=down, 1=up
```

### 4.5 Clipboard

```
CLIPBOARD_UPDATE
  [type: 0x05]
  [text_length: u32]
  [text: utf8 bytes]
  [source: u8]             // 0=host, 1=viewer
```

### 4.6 Heartbeat (Keep-Alive)

```
[type: 0x06]
[payload: empty]

Sent by either side every 30 seconds.
Receiver responds with HEARTBEAT echo.
Timeout: no heartbeat for 2 minutes → close connection.
```

---

## 5. Development Roadmap (Agent Tasks)

**Total estimated: 6–8 weeks solo**

### Week 1: Host Agent Foundation

**Task 1.1: Project Setup & Screen Capture**
- Create Rust project structure
- Add dependencies: `windows-rs`, `tokio`, `rustls`
- Implement DirectX11 Desktop Duplication API wrapper
- Capture frames at 30 FPS
- Test on Windows 10/11
- Deliverable: Binary that captures and logs frame count

**Task 1.2: H.264 Encoding Pipeline**
- Integrate libx264 (or NVENC if GPU available)
- Build encoding loop that accepts captured frames
- Implement changed-region detection
- Output raw NALU stream (Annex-B format)
- Deliverable: H.264-encoded output files (test only)

### Week 2: Host Networking & Input

**Task 2.1: TCP Server + TLS**
- Set up `tokio` async TCP listener on :5900
- Generate self-signed cert on first run
- Implement TLS 1.3 handshake
- Add connection state machine
- Deliverable: Host listens, accepts one TLS connection

**Task 2.2: Input Handler + Clipboard**
- Implement Windows `SendInput` for mouse/keyboard
- Add clipboard monitor (poll-based)
- Connect input events from network → SendInput
- Deliverable: Viewer can control mouse/keyboard (over TCP, unencrypted test)

### Week 3: Host Streaming

**Task 3.1: Video Stream Protocol**
- Implement message framing (type, length, payload)
- Send VIDEO_FRAME messages with H.264 NALU data
- Add timestamp and frame metadata
- Implement heartbeat mechanism
- Deliverable: Host streams H.264 over TLS

**Task 3.2: Adaptive Bitrate & Metrics**
- Monitor RTT and packet loss (estimate)
- Adjust encoder bitrate based on network conditions
- Add bandwidth estimation from frame size
- Deliverable: Bitrate scales with network quality

---

### Week 4: Viewer Foundation

**Task 4.1: Tauri Project Setup**
- Create Tauri + React + TypeScript scaffold
- Set up build pipeline (Windows only for MVP)
- Add dependencies: `ffmpeg.wasm`, `three.js` / Canvas API
- Deliverable: Tauri window opens, React app loads

**Task 4.2: H.264 Decoding**
- Integrate `ffmpeg.wasm` for frame decoding
- Parse incoming VIDEO_FRAME messages
- Decode to YUV420 format
- Handle corrupted/dropped frames
- Deliverable: Decoded frames logged to console

### Week 5: Viewer Rendering & Input

**Task 5.1: WebGL Rendering**
- Render decoded YUV420 frames to canvas via WebGL
- Scale to fit window; maintain aspect ratio
- Implement 60 FPS rendering loop
- Add FPS counter (debug UI)
- Deliverable: Video stream displays on screen

**Task 5.2: Input Capture & Send**
- Capture mouse events; send MOUSE_MOVE/MOUSE_CLICK
- Capture keyboard events; send KEY_EVENT
- Local echo: show cursor immediately
- Implement serialization to wire format
- Deliverable: Viewer can control host's mouse/keyboard

### Week 6: Clipboard & Polish

**Task 6.1: Clipboard Sync**
- Implement CLIPBOARD_UPDATE message handling
- Tauri clipboard API integration
- Bidirectional sync (host ↔ viewer)
- Keyboard shortcuts: Ctrl+C/Ctrl+V
- Deliverable: Text clipboard works both ways

**Task 6.2: Connection Flow & UI**
- Build connection dialog (IP:port input)
- Implement retry logic and error display
- Add toolbar with disconnect/fullscreen buttons
- Auto-hide toolbar after 2s inactivity
- Connection status indicator
- Deliverable: Full end-to-end workflow works

### Week 7: Stability & Performance

**Task 7.1: Error Handling & Resilience**
- Graceful disconnects (both sides)
- Reconnection logic (optional auto-retry)
- Handle screen lock/unlock on host
- Recover from frame decode errors
- Test long-running sessions (8+ hours)
- Deliverable: Zero crashes in 24-hour test

**Task 7.2: Performance Optimization**
- Profile CPU/memory usage (host and viewer)
- Optimize encoder keyframe interval
- Tune decoder buffer size
- Reduce latency in input pipeline
- Benchmark: target <100ms mouse-to-screen latency
- Deliverable: Performance report + metrics

### Week 8: Testing & Release

**Task 8.1: Integration Testing**
- End-to-end test matrix (LAN, over SSH tunnel, poor network)
- Test on Windows 10, Windows 11
- Multi-resolution testing (1920x1080, 2560x1440, 4K)
- Test long-duration sessions
- Deliverable: Test report + known issues

**Task 8.2: Installer & Documentation**
- Build installer with WiX or NSIS
- Create README with setup instructions
- Document CLI flags and configuration
- Create troubleshooting guide
- Deliverable: Production-ready binaries + docs

---

## 6. Success Metrics & Milestones

### Milestone 1: Host Agent Proof of Concept (End of Week 2)
- [ ] Host captures desktop at 30 FPS
- [ ] Encodes to H.264
- [ ] Listens on port 5900
- [ ] Accepts one TLS connection
- [ ] Receives input events and injects via SendInput

### Milestone 2: Viewer Basic Display (End of Week 5)
- [ ] Viewer connects to host
- [ ] Receives H.264 frames
- [ ] Decodes and displays video
- [ ] Sends mouse/keyboard input
- [ ] Displays at ≥20 FPS

### Milestone 3: MVP Complete (End of Week 6)
- [ ] Clipboard sync works bidirectionally
- [ ] Connection survives >1 hour
- [ ] UI is polished (toolbar, connection dialog, status)
- [ ] All core features functional

### Milestone 4: Stable Release (End of Week 8)
- [ ] Passes 24-hour stress test
- [ ] Handles network changes gracefully
- [ ] Performance within targets (latency, CPU, memory)
- [ ] Installer and docs complete

---

## 7. Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Host startup | <2 sec | Time to "Listening on :5900" |
| Host idle RAM | <100 MB | `tasklist` memory column |
| Host idle CPU | <1% | `taskmgr` CPU when no connection |
| Viewer startup | <3 sec | Launch to "Ready to connect" |
| Viewer idle RAM | <150 MB | `tasklist` memory column |
| Connection handshake | <2 sec | TLS negotiation + Hello |
| Screen capture latency | <10 ms | DirectX DDA callback to frame ready |
| Encoding latency | <50 ms | Frame in → NALU stream ready (30 FPS) |
| Decoding latency | <20 ms | NALU stream → raw pixels |
| Rendering latency | <16 ms | Pixels → canvas display (60 FPS target) |
| **Total input latency** | **<100 ms** | Mouse click → visible response |
| Video FPS (LAN) | 30 | Sustained 30 FPS on 1920x1080 |
| Video FPS (poor net) | 15–20 | Adaptive fallback on slow links |
| Bitrate (excellent) | 6–8 Mbps | 1920x1080, 30 FPS, high quality |
| Bitrate (poor) | 500 kbps–2 Mbps | Adaptive, usable but lower quality |

---

## 8. Anti-Patterns & Constraints

**Do NOT:**
- Use WebRTC (not needed for direct P2P)
- Build a backend server (goes against minimalism)
- Implement device dashboard or web UI (CLI only)
- Add multi-monitor support in Phase 1
- Use UDP instead of TCP (simpler with TCP; retransmission matters here)
- Implement session audit logs (not needed for personal use)
- Add encryption key negotiation beyond cert pinning (TLS handles it)
- Build unattended access modes (not in scope)

**Do:**
- Prioritize latency over visual quality
- Use hardware encoding if available, fallback gracefully
- Design for 8+ hour continuous sessions
- Test on both LAN and SSH-tunneled connections
- Implement cursor prediction on viewer side
- Keep both agent binaries as single-file executables
- Use self-signed TLS certs (pin public key; no PKI)

---

## 9. Deployment Model

### Host Agent
1. User downloads `remotelink-host.exe` (~30 MB)
2. Runs once: generates keypair, prints device ID
3. Runs as a standalone executable; listens on :5900
4. Optionally add to Windows startup (registry or Task Scheduler)
5. No installer needed (single binary)

### Viewer
1. User downloads `remotelink-viewer.exe` (~80 MB)
2. Installs via WiX or NSIS (or portable ZIP)
3. Launches Tauri window
4. Enters host IP:port or device ID
5. Connects via TLS

### Updates
- (MVP scope: manual download + replace)
- (Phase 2: signed update checks, optional auto-install)

---

## 10. Open Questions & Decisions

**Decision 1: H.264 Hardware Encoding**
- NVIDIA NVENC (CUDA SDK) vs. Intel Quick Sync vs. libx264 fallback?
- **Proposal:** Try NVENC first; detect GPU at runtime; fallback to libx264 silently.

**Decision 2: Decoder Library**
- `ffmpeg.wasm` (pure WASM, slower) vs. `libav.js` (pre-compiled, faster)?
- **Proposal:** Start with `ffmpeg.wasm` (smaller bundle); switch to `libav.js` if perf is insufficient.

**Decision 3: Cursor Rendering**
- Render cursor server-side (part of video) vs. send cursor shape separately?
- **Proposal:** Send cursor shape separately (saves bandwidth + improves latency).

**Decision 4: Clipboard Format**
- Text only (MVP) vs. include images (Phase 2)?
- **Proposal:** Text only for MVP; add images in Phase 1.5 if time permits.

**Decision 5: Connection Retry**
- Auto-reconnect on disconnect vs. manual retry button?
- **Proposal:** Manual retry for MVP (simpler); auto-retry checkbox in Phase 1.5.

---

## 11. Risk Register

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|-----------|
| DirectX DDA API complex | High | Medium | Start with simple GDI; switch to DDA if perf insufficient |
| NVENC not available on all GPUs | Medium | High | Graceful fallback to libx264; test on no-GPU machine |
| H.264 bitstream format errors | Medium | Low | Use well-tested encoder; validate NALU structure in tests |
| ffmpeg.wasm size/perf | High | Medium | Benchmark early; plan for libav.js switch if needed |
| TLS cert pinning usability | Low | Low | Auto-pin on first connection; clear UI for manual override |
| Network latency > 100ms | Low | Medium | Acceptable for non-LAN; document as limitation |
| Frame drops under load | Medium | Medium | Implement adaptive bitrate; skip frames rather than queue |
| Cursor lag despite optimization | Medium | Low | Implement client-side prediction; test early |

---

## 12. Dependencies & Licenses

**Host Agent:**
- `windows-rs` (MIT) — Windows API bindings
- `tokio` (MIT) — Async runtime
- `rustls` (Apache 2.0 / MIT) — TLS
- `libx264` (GPL v2) — Video encoding
- `clipboard-win` (MIT) — Clipboard access

**Viewer:**
- `React` (MIT) — UI framework
- `Tauri` (Apache 2.0 / MIT) — Desktop app framework
- `ffmpeg.wasm` (MIT / LGPL) — Video decoding
- `three.js` (MIT) — 3D rendering (or Canvas 2D)

**All licenses are compatible with personal/commercial use.**

---

## 13. Success Definition & Launch Checklist

### ✓ Code Complete
- [ ] All FR-01 through FR-12 implemented
- [ ] Zero known critical bugs
- [ ] All 8 milestones passed

### ✓ Performance
- [ ] Input latency <100ms (measured end-to-end)
- [ ] Host idle CPU <1%, RAM <100MB
- [ ] Viewer idle CPU <2%, RAM <150MB
- [ ] 30 FPS sustained on LAN (1920x1080)
- [ ] Survives 24-hour continuous session

### ✓ Stability
- [ ] Zero crashes in 24-hour test
- [ ] Graceful reconnect after host restart
- [ ] Handles screen lock/unlock
- [ ] Handles display sleep/wake

### ✓ Documentation
- [ ] README with setup instructions
- [ ] CLI help text for both binaries
- [ ] Troubleshooting guide
- [ ] Known limitations documented

### ✓ Deployment
- [ ] Host binary (.exe) ready
- [ ] Viewer installer (.msi or .exe) ready
- [ ] Both signed if needed
- [ ] Tested on Windows 10 + 11

---

## 14. Post-MVP Future Phases

### Phase 1.5: Enhanced Features (Weeks 9–10)
- [ ] Cursor shape optimization
- [ ] Adaptive bitrate tuning
- [ ] System tray UI for host
- [ ] Connection history / bookmarks
- [ ] Fullscreen mode toggle

### Phase 2: Cross-Platform (Weeks 11–16)
- [ ] macOS host agent (using Core Graphics)
- [ ] Linux host agent (using X11/Wayland)
- [ ] macOS/Linux viewer (native Tauri build)
- [ ] Test P2P between different OSes

### Phase 3: Advanced Features (Weeks 17+)
- [ ] Multi-monitor support
- [ ] Audio streaming (low-latency codec)
- [ ] File transfer (drag-and-drop)
- [ ] Session recording (optional)
- [ ] Remote reboot command
- [ ] Wake-on-LAN (from viewer)

---

## 15. Antigravity Integration Notes

This prompt is designed for **multi-agent execution** within the Antigravity framework:

1. **Task Decomposition:** Each "Task X.Y" in Section 5 is a discrete agent prompt; agents execute in parallel where possible.
2. **Skill Generation:** Generate detailed SKILL.md files for each module (ScreenCapture, Encoder, TCPServer, Decoder, Renderer, etc.) from this prompt.
3. **State Tracking:** Use Redis or file-based state to track completed tasks, blockers, and test results.
4. **Code Review:** Each agent commits code; another agent reviews and runs tests before merge.
5. **Performance Measurement:** Automated benchmarking agents measure latency, CPU, memory after each sprint.

---

## 16. Final Note

**This is a personal, pragmatic build.** The goal is not to ship a product or scale to 1M users. It's to have a fast, reliable, trustworthy remote desktop tool that works for you and anyone you invite to use it.

Ship it when it works. Ship it when it's stable. Everything else is optional.

**Timeline: 6–8 weeks. Budget: ~0 infrastructure cost. Success metric: "I can control my other computer, and it feels instant."**

---

*Prompt version 1.0 | Created for Anil Bhattarai | Antigravity Framework*
