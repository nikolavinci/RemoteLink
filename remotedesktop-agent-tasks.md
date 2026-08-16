# RemoteLink: Agent Task Decomposition

**Framework:** Antigravity Multi-Agent Execution
**Orchestrator:** Claude Code (or manual agent spawn)
**Sprint Duration:** 1 week per sprint (Tasks 1.1–1.2 in parallel, then 2.1–2.2, etc.)
**Total Agents:** 8 specialized agents (can run 2–3 in parallel per sprint)

---

## Sprint 1: Host Foundation (Week 1)

### Agent 1.1: Desktop Capture & Screen Rendering
**Role:** Senior Systems Engineer (Windows/DirectX)
**Duration:** 5 days
**Dependencies:** None (can start immediately)

**Prompt:**

```
You are building the desktop capture module for RemoteLink, a minimal remote 
desktop tool for Windows. Your job is to capture the screen at 30 FPS using 
DirectX11's Desktop Duplication API.

## Requirements

1. Create a Rust binary that:
   - Initializes DirectX11 Device + Swap Chain
   - Uses DXGI Desktop Duplication API to capture frames
   - Outputs raw BGRA32 pixel data (or YUV420 if possible)
   - Targets 30 FPS (1920x1080 resolution)
   - Detects changed regions to optimize for encoding
   - Handles screen lock/unlock events
   - Logs frame count, resolution, and timing every 10 seconds

2. Project structure:
   ```
   remotelink-host/
   ├── src/
   │   ├── main.rs
   │   ├── capture.rs          # Your module
   │   ├── encoder.rs          # Stub for later
   │   └── network.rs          # Stub for later
   ├── Cargo.toml
   └── README.md
   ```

3. Dependencies to use:
   - `windows-rs` (latest) for DirectX bindings
   - `image` crate for image manipulation (optional)
   - Standard `std::time` for FPS calculation

4. Test coverage:
   - [ ] Binary runs and outputs "Listening..." message
   - [ ] Logs frame count every 10 sec (proof of capture)
   - [ ] Works on Windows 10 and Windows 11
   - [ ] No crashes after 1-hour continuous runtime
   - [ ] Memory footprint <100MB idle

5. Deliverables:
   - Rust source code in src/capture.rs
   - Cargo.toml with dependencies
   - README explaining setup (DirectX SDK if needed)
   - Test log (1-hour run, frame counts)
   - Performance notes (CPU %, RAM usage)

6. Edge cases to handle:
   - Display sleeping/waking
   - Screen lock/unlock
   - Resolution changes during capture
   - Multiple monitors (ignore non-primary for MVP)
   - Failed DDA (outdated GPU driver) → error message

## Output Format

Return:
1. Complete Rust source files (copy-paste ready)
2. Cargo.toml (dependencies finalized)
3. Instructions for building and running
4. Test results (frame count log)
5. Performance profile (screenshot of Task Manager during 1-hour run)

Do not build a full application yet; just the capture loop + file logging.
```

**Test Criteria:**
- Binary outputs frames at ~30 FPS
- Logs show frame count increasing steadily
- No crashes in 1-hour test
- Memory <100MB

**Merge Checklist:**
- [ ] Compiles without warnings
- [ ] Frame count log validates 30 FPS
- [ ] No memory leaks detected

---

### Agent 1.2: H.264 Encoder Pipeline
**Role:** Media Engineer (Video Codecs)
**Duration:** 5 days
**Dependencies:** Agent 1.1 output (will integrate frame data in Week 2)

**Prompt:**

```
You are building the H.264 encoder for RemoteLink. Your job is to accept 
raw BGRA32 frames and output H.264-encoded NALUs in Annex-B format.

## Requirements

1. Create a Rust module that:
   - Accepts BGRA32 pixel buffers (1920x1080, 30 FPS)
   - Encodes to H.264 using:
     a) NVIDIA NVENC (via CUDA SDK) if GPU available
     b) Intel Quick Sync (via Intel Media SDK) if iGPU available
     c) libx264 (software) as fallback
   - Outputs raw H.264 NALU stream (Annex-B format, with start codes)
   - Detects keyframe opportunities (IDR every 2 seconds)
   - Adapts bitrate: 500 kbps–8 Mbps (simulate with hardcoded rates for now)
   - Logs encoding latency and bitrate every 10 frames

2. Project structure (add to remotelink-host/):
   ```
   src/
   ├── encoder.rs               # Your module
   ├── encoder/
   │   ├── nvenc.rs            # NVENC encoder (if available)
   │   ├── intel_qsv.rs        # Intel QSV encoder (if available)
   │   └── libx264.rs          # libx264 fallback
   └── ...
   ```

3. Dependencies:
   - `windows-rs` for Windows API
   - `libx264-sys` or FFI binding for libx264
   - Optional: `nvidia-encoding-sys` (NVENC)
   - `tokio` for async if needed

4. Behavior:
   - Detect GPU at startup; log which encoder will be used
   - Accept frame buffer + metadata (timestamp, changed_regions)
   - Output raw H.264 NALU stream immediately
   - Handle encoder state machine (open → encode → close)
   - Graceful fallback if NVENC unavailable

5. Test coverage:
   - [ ] Encodes synthetic test frames (solid color, color gradients)
   - [ ] Produces valid H.264 NALU streams (parseable)
   - [ ] Encoding latency logged (target <50ms per frame at 30 FPS)
   - [ ] Bitrate adapts when simulated (logs show changes)
   - [ ] Works on:
     - GPU with NVIDIA
     - GPU without NVIDIA (fallback to libx264)
     - Headless environment (handle gracefully)

6. Deliverables:
   - Rust encoder.rs + submodules
   - Test binary that reads captured frames and encodes them
   - Encoding latency log (1-hour run)
   - Bitrate adaptation proof (logs showing 500k→8M transition)

## Output Format

Return:
1. Complete Rust source (encoder.rs + submodules)
2. Cargo.toml additions
3. Integration notes (how to pass frames from capture.rs)
4. Test results (latency log, bitrate changes)
5. Fallback strategy (what happens if NVENC fails?)

Focus on correctness and latency; image quality can be tuned later.
```

**Test Criteria:**
- Encodes test frames to valid H.264
- Encoding latency <50ms per frame
- Graceful fallback if GPU unavailable

**Merge Checklist:**
- [ ] Valid NALU output (parseable by external tools)
- [ ] Latency within targets
- [ ] No memory leaks after 1-hour run

---

## Sprint 2: Host Networking & Input (Week 2)

### Agent 2.1: TCP Server + TLS Handshake
**Role:** Network Engineer (Async I/O, TLS)
**Duration:** 5 days
**Dependencies:** None (parallel with 1.x)

**Prompt:**

```
You are building the network layer for RemoteLink Host Agent. Your job is to 
create a TLS server on port 5900 that:
1. Accepts ONE connection at a time
2. Negotiates TLS 1.3
3. Handles message framing (type, length, payload)
4. Stays open for 8+ hours
5. Disconnects gracefully

## Requirements

1. Create a Rust async TCP server:
   - Listen on 0.0.0.0:5900 (or --port flag)
   - Generate self-signed RSA cert on first run
   - Store cert in %APPDATA%\RemoteLink\
   - Accept TLS 1.3 connections only
   - Reject additional connections when one is active (queue or reject?)
   - Implement message framing: [type:u8][length:u32][payload:bytes]

2. Message types (for this module, just forward):
   - 0x01 = VIDEO_FRAME (ignore for now)
   - 0x02 = MOUSE_MOVE (ignore for now)
   - 0x03 = MOUSE_CLICK (ignore for now)
   - 0x04 = KEY_EVENT (ignore for now)
   - 0x05 = CLIPBOARD_UPDATE (ignore for now)
   - 0x06 = HEARTBEAT (echo back immediately)
   - 0x07 = DISCONNECT (close session gracefully)

3. Project structure:
   ```
   src/
   ├── network.rs               # Your module
   └── network/
       ├── server.rs           # TCP listening loop
       ├── tls.rs              # TLS cert generation + handshake
       ├── frame.rs            # Message framing/parsing
       └── session.rs          # Per-connection state
   ```

4. Dependencies:
   - `tokio` (async runtime)
   - `rustls` (TLS)
   - `rcgen` (cert generation)

5. Behavior:
   - On startup: generate cert if not exists; output "Listening on :5900"
   - On viewer connect: TLS handshake; output "Client connected"
   - Forward received messages to handler (stub for now)
   - Every 30 seconds: send HEARTBEAT
   - On 2-minute idle: close connection
   - Log all connections/disconnections with timestamp

6. Test coverage:
   - [ ] Server starts and listens
   - [ ] TLS cert generates correctly
   - [ ] Can accept TLS connection from test client
   - [ ] Rejects second concurrent connection
   - [ ] Message framing works (parse/serialize)
   - [ ] Heartbeat works (sends every 30s)
   - [ ] Idle timeout works (close after 2 min)
   - [ ] Graceful shutdown on DISCONNECT message

7. Deliverables:
   - Rust network.rs + submodules
   - Standalone test binary (mock viewer client)
   - Connection logs (shows connect/disconnect/heartbeat)
   - Performance notes (CPU, memory during idle)

## Output Format

Return:
1. Complete Rust source (network.rs + submodules)
2. Cargo.toml updates
3. Test client (for verifying connection)
4. Log showing 1-hour idle run (heartbeats, no crashes)
5. Cert generation example output

This module is the backbone of the host; correctness > speed.
```

**Test Criteria:**
- Server listens and accepts TLS connection
- Heartbeat mechanism works
- Connection survives 1 hour idle

**Merge Checklist:**
- [ ] TLS handshake correct (verified with test client)
- [ ] Heartbeat sends/receives
- [ ] Idle timeout triggers
- [ ] No memory leaks

---

### Agent 2.2: Input Handler + Clipboard Monitor
**Role:** Windows Integration Engineer
**Duration:** 5 days
**Dependencies:** None (parallel with 2.1)

**Prompt:**

```
You are building the input injection and clipboard monitoring for RemoteLink.

## Requirements

1. Create a Rust module that:
   - Accepts mouse events (MOVE, CLICK) and injects via Windows SendInput
   - Accepts keyboard events (KEY_DOWN, KEY_UP) and injects via SendInput
   - Monitors Windows clipboard for changes (text only)
   - Sends clipboard updates to network layer
   - Syncs clipboard from viewer to host

2. Project structure:
   ```
   src/
   ├── input.rs                 # Your module
   └── input/
       ├── mouse.rs
       ├── keyboard.rs
       ├── clipboard.rs
       └── event.rs             # Structs for input events
   ```

3. Dependencies:
   - `windows-rs` for Windows APIs
   - `clipboard-win` for clipboard access
   - `tokio` if using async polling

4. Behavior - Mouse:
   - MOUSE_MOVE(x, y) → Windows cursor move (via SetCursorPos or SendInput)
   - MOUSE_CLICK(button, action) → left/right/middle/double click
   - MOUSE_SCROLL(delta) → wheel scroll
   - Latency target: <5ms from event received to injection

5. Behavior - Keyboard:
   - KEY_EVENT(vk_code, action) → key down/up
   - Support all Windows virtual key codes (A-Z, 0-9, F1-F12, etc.)
   - Support key combinations (Ctrl+C, Shift+F1, etc.)
   - Latency target: <5ms

6. Behavior - Clipboard:
   - Poll Windows clipboard every 500ms (simple approach)
   - Detect text changes (compare to last known value)
   - Send CLIPBOARD_UPDATE message on change
   - Accept incoming CLIPBOARD_UPDATE and set Windows clipboard
   - Handle Unicode text (UTF-8)

7. Test coverage:
   - [ ] Mouse movement works (verify with cursor tracking)
   - [ ] Mouse clicks work (click buttons, verify in web browser)
   - [ ] Keyboard input works (type in Notepad, verify output)
   - [ ] Clipboard read works (set clipboard externally, verify received)
   - [ ] Clipboard write works (receive update, verify in Notepad)
   - [ ] No permission errors or crashes
   - [ ] Latency <5ms per injection

8. Deliverables:
   - Rust input.rs + submodules
   - Test harness (manual verification steps)
   - Clipboard sync test (copy → send → paste)
   - Performance profile (CPU during polling)

## Output Format

Return:
1. Complete Rust source (input.rs + submodules)
2. Cargo.toml updates
3. Test results (manual test checklist)
4. Clipboard sync proof (before/after screenshot)
5. Latency measurements (time input to injection)

This module is tested manually; include detailed steps.
```

**Test Criteria:**
- Mouse/keyboard injection works (manual test)
- Clipboard sync works bidirectionally
- Latency <5ms per injection

**Merge Checklist:**
- [ ] Manual keyboard test passed (Notepad output)
- [ ] Manual mouse test passed (cursor moves)
- [ ] Clipboard test passed (copy/paste works)

---

## Sprint 3: Host Streaming & Adaptation (Week 3)

### Agent 3.1: Video Stream Protocol & Framing
**Role:** Protocol Engineer
**Duration:** 4 days
**Dependencies:** Agent 1.2 (encoder), Agent 2.1 (network)

**Prompt:**

```
You are building the video streaming protocol for RemoteLink. Your job is to 
package encoded H.264 frames into wire-format messages and send them over TLS.

## Requirements

1. Implement message framing for VIDEO_FRAME:
   ```
   [type: u8 = 0x01]
   [length: u32]              # Length of payload
   [timestamp: u64]           # ms since epoch
   [frame_type: u8]           # 0=P-frame, 1=I-frame (keyframe)
   [width: u16]
   [height: u16]
   [bitrate: u32]             # kbps (for stats)
   [data: bytes]              # Raw H.264 NALU stream
   ```

2. Create a Rust module that:
   - Accepts H.264 NALU from encoder
   - Wraps in VIDEO_FRAME message
   - Sends over TLS (using Agent 2.1's network layer)
   - Tracks frame count, bitrate, fps
   - Logs stats every 10 frames

3. Integration points:
   - Input: from encoder.rs (NALU stream)
   - Output: to network.rs (message framing)
   - Logging: FPS, bitrate, dropped frames

4. Project structure:
   ```
   src/
   ├── streaming.rs            # Your module
   └── streaming/
       ├── frame.rs            # VIDEO_FRAME wrapper
       ├── stats.rs            # FPS/bitrate tracking
       └── framing.rs          # Serialization
   ```

5. Behavior:
   - Queue frames from encoder
   - Serialize each frame to wire format
   - Send over active TLS connection
   - Drop frames if encoder output exceeds network speed
   - Log: "Frame 1234 | Type:I | 8192 bytes | 2.5 Mbps | 30 FPS"

6. Test coverage:
   - [ ] VIDEO_FRAME serialization correct (can parse on other end)
   - [ ] FPS calculation accurate (10 frames in 333ms = 30 FPS)
   - [ ] Bitrate calculation correct (8192 bytes/frame × 30 fps = 2 Mbps)
   - [ ] Frame dropping works under load
   - [ ] Stats logging correct format

7. Deliverables:
   - Rust streaming.rs + submodules
   - Integration guide (how to wire encoder → streaming → network)
   - Test output (10-frame log with stats)
   - Wire format spec (detailed byte-level example)

## Output Format

Return:
1. Complete Rust source (streaming.rs + submodules)
2. Cargo.toml updates
3. Wire format examples (hex dump of 3 sample frames)
4. Stats output (FPS/bitrate log)
5. Integration notes (call sequence from encoder.rs)

Focus on correctness of serialization; performance tuning is Phase 1.5.
```

**Test Criteria:**
- VIDEO_FRAME messages are parseable
- Stats accurate (FPS, bitrate calculations)
- Frame dropping works under load

**Merge Checklist:**
- [ ] Serialization correct (wire format matches spec)
- [ ] Stats logging accurate

---

### Agent 3.2: Adaptive Bitrate & Network Metrics
**Role:** Network Optimization Engineer
**Duration:** 3 days
**Dependencies:** Agent 3.1 (streaming), Agent 2.1 (network)

**Prompt:**

```
You are building adaptive bitrate control for RemoteLink. Your job is to 
monitor network conditions and adjust encoder bitrate dynamically.

## Requirements

1. Implement network metrics collection:
   - Round-trip time (RTT): estimate via HEARTBEAT
   - Frame transmission rate (infer from timestamps)
   - Packet loss: estimate if viewer stops acknowledging

2. Bitrate adaptation strategy:
   - Base: 500 kbps (poor network)
   - Target: 4 Mbps (good network)
   - Max: 8 Mbps (excellent network)
   - Adjustment: +10% per Mbps available BW, -20% per 10% packet loss

3. Create a Rust module:
   ```
   src/
   ├── metrics.rs              # Your module
   └── metrics/
       ├── rtt.rs              # RTT estimation
       ├── throughput.rs       # BW estimation
       ├── adaptation.rs       # Bitrate control
       └── stats.rs            # Logging
   ```

4. Dependencies:
   - Standard `std::time` for latency measurement
   - Encoder bitrate setter (stub for now)

5. Behavior:
   - Every 5 seconds: calculate RTT from HEARTBEAT round-trip time
   - Estimate throughput: total_bytes_sent / elapsed_time
   - Adjust encoder bitrate based on metrics
   - Log: "RTT: 15ms | Throughput: 5.2 Mbps | Target bitrate: 5.5 Mbps"

6. Test coverage:
   - [ ] RTT calculation works (measure 10 HEARTBEATs)
   - [ ] Throughput estimation accurate
   - [ ] Bitrate adjustment algorithm works (sample scenarios)
   - [ ] Adaptation responds quickly to network changes
   - [ ] No oscillation (bitrate doesn't thrash)

7. Deliverables:
   - Rust metrics.rs + submodules
   - Adaptation algorithm (pseudocode + implementation)
   - Test scenarios (good BW → poor BW transition)
   - Stats log showing bitrate changes

## Output Format

Return:
1. Complete Rust source (metrics.rs + submodules)
2. Cargo.toml updates
3. Adaptation algorithm (explained + pseudocode)
4. Test log (showing RTT/throughput/bitrate over time)
5. Edge case handling (what if RTT spikes? what if zero throughput?)

This is Phase 1.5 nice-to-have; ensure it's optional and doesn't break baseline.
```

**Test Criteria:**
- RTT calculation works
- Bitrate adaptation responds to network changes
- No oscillation/thrashing

**Merge Checklist:**
- [ ] Algorithm logic correct
- [ ] Stats logging accurate
- [ ] Optional flag to disable (for testing baseline)

---

## Sprint 4: Viewer Foundation (Week 4)

### Agent 4.1: Tauri + React Scaffold & H.264 Decoding
**Role:** Frontend + Media Engineer
**Duration:** 5 days
**Dependencies:** None (parallel with 3.x)

**Prompt:**

```
You are building the viewer frontend for RemoteLink. Your job is to:
1. Create a Tauri desktop app
2. Set up React + TypeScript scaffold
3. Integrate H.264 decoding

## Requirements

1. Project structure:
   ```
   remotelink-viewer/
   ├── src-tauri/              # Rust backend (minimal)
   │   ├── src/
   │   │   ├── main.rs
   │   │   └── protocol.rs    # Message parsing
   │   └── Cargo.toml
   ├── src/                    # React frontend
   │   ├── App.tsx
   │   ├── components/
   │   │   ├── ConnectionDialog.tsx
   │   │   ├── VideoCanvas.tsx
   │   │   └── Toolbar.tsx
   │   ├── hooks/
   │   │   └── useRemoteDesktop.ts
   │   └── index.tsx
   ├── public/
   │   └── index.html
   ├── package.json
   └── tauri.conf.json
   ```

2. Tauri configuration:
   - Target: Windows only (for MVP)
   - Window size: 1280×720 (resizable)
   - Enable all required APIs (clipboard, window control)

3. React Components:
   - `ConnectionDialog`: Input field for IP:port
   - `VideoCanvas`: WebGL canvas for H.264 decoded video
   - `Toolbar`: Disconnect, fullscreen, settings buttons
   - `App`: Main state management

4. H.264 Decoding:
   - Integrate `ffmpeg.wasm` (via CDN)
   - Load H.264 codec on startup
   - Accept raw NALU stream
   - Decode to YUV420 or RGB24 pixel data

5. Message Handling:
   - Parse incoming VIDEO_FRAME messages
   - Queue decoded frames
   - Render at 60 FPS (match monitor refresh rate)

6. Deliverables:
   - React + TypeScript scaffold (copy-paste ready)
   - Tauri main.rs (minimal, Rust backend stub)
   - H.264 decoder integration (ffmpeg.wasm setup)
   - package.json with dependencies
   - Build instructions

## Output Format

Return:
1. Complete React/TypeScript source (App.tsx + components)
2. Tauri scaffold (src-tauri/ structure)
3. package.json
4. Build instructions (npm install → npm run tauri dev)
5. Screenshot of running app (empty state)

No actual streaming yet; this is UI scaffolding + decoder setup.
```

**Test Criteria:**
- Tauri window opens
- React app loads
- H.264 decoder initializes (no errors in console)

**Merge Checklist:**
- [ ] App compiles without errors
- [ ] UI renders (connection dialog visible)
- [ ] Decoder loads without crashing

---

### Agent 4.2: WebGL Rendering & Display Loop
**Role:** Graphics Engineer
**Duration:** 4 days
**Dependencies:** Agent 4.1 (scaffold + decoder)

**Prompt:**

```
You are building the video rendering engine for RemoteLink Viewer.

## Requirements

1. Render decoded frames to canvas:
   - Accept YUV420 or RGB pixel data from decoder
   - Display on WebGL canvas
   - Scale to fit window (maintain aspect ratio)
   - Render at monitor refresh rate (60 FPS target)

2. Create React component:
   ```tsx
   <VideoCanvas
     frames={decodedFrames}     // Queue of pixel data
     width={1920}
     height={1080}
     onFrameRendered={callback}
   />
   ```

3. WebGL pipeline:
   - Vertex shader: render quad covering canvas
   - Fragment shader: convert YUV420 → RGB (if needed)
   - Texture: bind decoded frame as input
   - FPS counter: display in corner (debug mode)

4. Performance targets:
   - Rendering latency: <16ms per frame (60 FPS)
   - Canvas resize handling: smooth scaling
   - Frame drop handling: render latest, skip old

5. Dependencies:
   - `react-three-fiber` (optional, can use Canvas 2D instead)
   - Or pure WebGL with TypeScript

6. Deliverables:
   - React VideoCanvas component
   - Shader code (vertex + fragment GLSL)
   - FPS calculation + display
   - Aspect ratio preservation logic

## Output Format

Return:
1. Complete React VideoCanvas.tsx
2. GLSL shader source (vertex + fragment)
3. WebGL setup code (context creation, texture binding)
4. Performance notes (FPS achieved in test)
5. Screenshot of rendering test (solid color frame)

Start simple (render solid color); then add YUV→RGB conversion.
```

**Test Criteria:**
- WebGL canvas renders without errors
- FPS counter displays correctly
- Aspect ratio maintained

**Merge Checklist:**
- [ ] Canvas renders (solid color test)
- [ ] FPS counter working
- [ ] No WebGL errors in console

---

## Sprint 5: Viewer Input & Integration (Week 5)

### Agent 5.1: Input Capture & Mouse/Keyboard Handling
**Role:** Input Systems Engineer
**Duration:** 5 days
**Dependencies:** Agent 4.1 (scaffold)

**Prompt:**

```
You are building input capture for RemoteLink Viewer. Your job is to catch 
mouse and keyboard events and send them to the host.

## Requirements

1. Capture from video canvas:
   - Mouse move events (relative to canvas)
   - Mouse clicks (left, right, middle, double)
   - Mouse scroll
   - Keyboard events (all keys)

2. Send events to host:
   - Serialize to wire format (from main PRD)
   - Use Tauri invoke to send via Rust backend
   - Queue if host connection temporarily unavailable

3. Local echo (client-side prediction):
   - Show cursor position immediately on canvas
   - Don't wait for round-trip; update instantly
   - Render cursor shape (arrow pointer)

4. Create React hook:
   ```tsx
   const { captureInput, sendEvent } = useInputCapture({
     canvasRef: ref,
     onMouseMove: (x, y) => sendEvent({type: 'MOUSE_MOVE', x, y}),
     onKeyPress: (key) => sendEvent({type: 'KEY_EVENT', key}),
   });
   ```

5. Keyboard mapping:
   - Map browser KeyCode → Windows VK_* codes
   - Handle modifiers (Ctrl, Shift, Alt, Win)
   - Support Ctrl+C, Ctrl+V, etc.

6. Deliverables:
   - React useInputCapture hook
   - Event serialization (match wire format)
   - KeyCode mapping table (browser → Windows)
   - Cursor rendering component
   - Test harness (manual keyboard input, log output)

## Output Format

Return:
1. useInputCapture hook (TypeScript)
2. KeyCode mapping (JavaScript object)
3. Event serialization code
4. Cursor rendering component
5. Test output (key presses logged)

No network integration yet; assume Tauri backend is ready.
```

**Test Criteria:**
- Mouse events captured (log to console)
- Keyboard events captured (log to console)
- Cursor prediction renders

**Merge Checklist:**
- [ ] Console logs show correct event data
- [ ] KeyCode mapping includes common keys
- [ ] Cursor visible on canvas

---

### Agent 5.2: Tauri Backend Integration (TCP → React)
**Role:** Desktop App Engineer
**Duration:** 4 days
**Dependencies:** Agent 5.1 (input capture), Agent 2.1 (host network)

**Prompt:**

```
You are building the Tauri backend for RemoteLink Viewer. Your job is to:
1. Accept input events from React
2. Connect to host via TCP
3. Send/receive messages
4. Update React state with video frames

## Requirements

1. Tauri main.rs handles:
   - TCP connection to host (IP:port)
   - TLS handshake
   - Message frame parsing/serialization
   - Event loop (read from network, dispatch to React via IPC)

2. IPC commands (Tauri → React):
   - `invoke('connect', {host, port})` → returns success/error
   - `invoke('sendInput', {type, x, y, ...})` → send input event
   - `invoke('disconnect')` → close connection
   - Event listener: `listen('video-frame', (frame) => {...})`

3. Network loop (Rust async):
   ```
   loop {
     read TLS socket
     parse frame
     dispatch to React via emit('video-frame', frame)
   }
   ```

4. Message framing (reuse from Host):
   - VIDEO_FRAME, MOUSE_MOVE, KEY_EVENT, CLIPBOARD_UPDATE, etc.
   - Serialize/deserialize correctly

5. Deliverables:
   - Tauri src/main.rs
   - TCP client implementation
   - IPC command handlers
   - Event emission code
   - Integration guide

## Output Format

Return:
1. Complete Tauri main.rs
2. Cargo.toml (src-tauri/)
3. IPC command definitions
4. Connection flow diagram
5. Test results (connect to host, receive dummy frame)

This glues Host to React; must work perfectly.
```

**Test Criteria:**
- Tauri app connects to host (with test server)
- Can send input events
- Receives video frames (test data)

**Merge Checklist:**
- [ ] TLS connection succeeds
- [ ] Message framing works bidirectionally
- [ ] IPC commands functional

---

## Sprint 6: Clipboard & Polish (Week 6)

### Agent 6.1: Clipboard Sync (Bidirectional)
**Role:** Integration Engineer
**Duration:** 3 days
**Dependencies:** Agent 2.2 (host clipboard), Agent 5.2 (viewer network)

**Prompt:**

```
You are implementing clipboard sync for RemoteLink. Text only, both directions.

## Requirements

1. Host side (Rust):
   - Poll Windows clipboard every 500ms
   - On change, send CLIPBOARD_UPDATE message to viewer
   - Accept incoming CLIPBOARD_UPDATE and set Windows clipboard

2. Viewer side (React + Tauri):
   - Listen for CLIPBOARD_UPDATE messages from host
   - Update browser clipboard (if permission granted)
   - On Ctrl+C: copy current selection to remote clipboard
   - On Ctrl+V: paste from remote clipboard

3. Tauri Clipboard API:
   - Use `tauri::api::clipboard::read_text()` and `write_text()`
   - Permissions: request clipboard access on startup

4. Wire format (from main spec):
   ```
   CLIPBOARD_UPDATE
     [type: 0x05]
     [text_length: u32]
     [text: utf8 bytes]
     [source: u8]     // 0=host, 1=viewer
   ```

5. Deliverables:
   - Host: clipboard.rs polling loop
   - Viewer: React hook useClipboard() + Tauri integration
   - Test steps: Copy on host → paste on viewer (manual verification)
   - Handling: Unicode, long text (>1MB rejection)

## Output Format

Return:
1. Host clipboard.rs implementation
2. Viewer useClipboard hook
3. Tauri clipboard integration code
4. Test steps + screenshots (copy/paste proof)
5. Edge case handling (empty clipboard, non-text content)

Simple module, high value for usability.
```

**Test Criteria:**
- Copy on host → visible on viewer
- Paste on viewer → visible on host
- Unicode text works
- Long text handled gracefully

**Merge Checklist:**
- [ ] Manual copy/paste test passed
- [ ] No crashes on empty clipboard
- [ ] Permissions requested correctly

---

### Agent 6.2: UI Polish & Connection Flow
**Role:** UX/UI Engineer
**Duration:** 4 days
**Dependencies:** Agent 5.2, Agent 6.1

**Prompt:**

```
You are polishing the RemoteLink Viewer UI. Make it feel professional and responsive.

## Requirements

1. Connection Dialog:
   - Text input for "Host IP:Port" (e.g., "192.168.1.100:5900")
   - Button: "Connect"
   - Status: "Ready", "Connecting...", "Connected", "Error: [message]"
   - Quick connect history (remember last 3 hosts)

2. Video Display:
   - Full-screen canvas (minus toolbar)
   - Aspect ratio maintained
   - Toolbar auto-hide after 2s inactivity
   - Cursor overlay (client-side prediction)

3. Toolbar (shown on mouse move):
   - [Disconnect] button
   - [Fullscreen] toggle
   - Connection status (ms latency, FPS, bitrate)
   - [Settings] button (placeholder)

4. Styling:
   - Dark theme (Antigravity aesthetic)
   - Minimal chrome
   - Responsive to window resizing
   - Smooth transitions (fade in/out toolbar)

5. Deliverables:
   - React components (all updated for polish)
   - CSS/Tailwind styling
   - Connection flow (screenshots)
   - Toolbar interactions (video demo or description)

## Output Format

Return:
1. Updated React components (App.tsx, ConnectionDialog.tsx, Toolbar.tsx)
2. Tailwind config + CSS
3. Connection flow screenshots (empty → connecting → connected → error)
4. Toolbar auto-hide logic
5. Styling notes (colors, fonts, spacing)

Focus on clarity and responsiveness; animations are nice-to-have.
```

**Test Criteria:**
- Connection dialog works (connect/error states)
- Toolbar auto-hides correctly
- Styling matches Antigravity aesthetic

**Merge Checklist:**
- [ ] Visual proof (screenshots of each state)
- [ ] No TypeScript errors
- [ ] Responsive to window resize

---

## Sprint 7: Stability Testing (Week 7)

### Agent 7.1: Error Handling & Resilience
**Role:** QA Engineer (Reliability)
**Duration:** 5 days
**Dependencies:** All previous agents (full end-to-end integration)

**Prompt:**

```
You are testing RemoteLink for stability and error resilience.

## Requirements

1. Error scenarios to test:
   - [ ] Host restarts during connection
   - [ ] Viewer restarts during connection
   - [ ] Network cable unplugged (sudden disconnect)
   - [ ] Host goes to sleep/locks screen
   - [ ] Viewer window minimized/hidden
   - [ ] High packet loss (via NetEM or proxy)
   - [ ] Long-duration session (8+ hours)

2. Graceful handling:
   - Both sides detect disconnect within 5 seconds
   - Display clear error message to user
   - Allow reconnection without restart
   - No resource leaks (monitor memory over time)

3. Testing tools:
   - Windows Task Manager (memory, CPU)
   - Wireshark (packet loss simulation)
   - Linux NetEM (via SSH tunnel)
   - Stress tools (disk I/O, network saturation)

4. Test matrix:
   ```
   ┌──────────────────────┬──────────┬─────────────┐
   │ Scenario             │ Expected │ Pass/Fail   │
   ├──────────────────────┼──────────┼─────────────┤
   │ 8-hour continuous    │ No crash │ [ ]         │
   │ Host restart         │ Reconnect│ [ ]         │
   │ Network drop         │ Error msg│ [ ]         │
   │ High packet loss     │ Degrade  │ [ ]         │
   │ Screen lock          │ Video ok │ [ ]         │
   │ Memory leak check    │ <5MB/hr  │ [ ]         │
   └──────────────────────┴──────────┴─────────────┘
   ```

5. Deliverables:
   - Test plan document
   - Test results matrix (pass/fail)
   - Memory graphs (1-hour test)
   - Known issues list
   - Recovery procedures

## Output Format

Return:
1. Detailed test plan (step-by-step)
2. Test results (matrix + screenshots)
3. Memory usage graph (Task Manager export)
4. Identified bugs (with reproduction steps)
5. Fixes applied (reference commits or patch files)

This is integration testing; requires patience and manual verification.
```

**Test Criteria:**
- No crashes in 8-hour test
- Graceful disconnect handling
- Memory <200MB on host, <150MB on viewer after 1 hour

**Merge Checklist:**
- [ ] All test scenarios passed
- [ ] Memory graph shows stable usage
- [ ] Known issues documented

---

### Agent 7.2: Performance Optimization & Profiling
**Role:** Performance Engineer
**Duration:** 4 days
**Dependencies:** Agent 7.1 output (identify bottlenecks)

**Prompt:**

```
You are optimizing RemoteLink for performance. Measure, profile, improve.

## Requirements

1. Metrics to measure:
   - Input latency: mouse click → screen response (target <100ms)
   - FPS: sustained frame rate (target 30 LAN, 15-20 internet)
   - Encoding latency: capture → encode → send (target <50ms)
   - Decoding latency: receive → decode → render (target <20ms)
   - CPU usage: host idle <1%, viewer idle <2%
   - Memory: host <100MB, viewer <150MB

2. Profiling tools:
   - Host: Windows Perf Analyzer, Intel VTune
   - Viewer: Chrome DevTools, React Profiler
   - Network: Wireshark, ping RTT measurement

3. Optimization opportunities:
   - Reduce keyframe frequency if CPU-bound
   - Tune encoder bitrate defaults
   - Optimize texture upload in WebGL
   - Batch input events if network-latent
   - Reduce polling frequency for clipboard

4. Deliverables:
   - Performance profile (CPU flame graph, memory timeline)
   - Latency measurements (before/after optimization)
   - Optimization patches (code changes with justification)
   - Benchmark script (repeatable test)

## Output Format

Return:
1. Performance report (metrics, baselines, targets)
2. Profiling data (flame graphs, screenshots)
3. Optimizations applied (patches)
4. Benchmark results (before/after)
5. Remaining bottlenecks (if any)

Aim for top 3 impactful optimizations; diminishing returns after that.
```

**Test Criteria:**
- Input latency <100ms measured end-to-end
- CPU usage within targets
- Memory stable over 1 hour

**Merge Checklist:**
- [ ] Performance report complete
- [ ] Benchmarks repeatable
- [ ] Optimizations improve measurable metrics

---

## Sprint 8: Release & Documentation (Week 8)

### Agent 8.1: Installer, Signing & Distribution
**Role:** DevOps / Release Engineer
**Duration:** 4 days
**Dependencies:** All code complete (from Sprint 7)

**Prompt:**

```
You are building the final installers and distribution setup for RemoteLink.

## Requirements

1. Host Agent Installer:
   - Language: WiX Toolset or NSIS
   - Creates %APPDATA%\RemoteLink\ directory
   - Installs remotelink-host.exe
   - Option: Add to Windows startup (via registry or Task Scheduler)
   - Generates device keypair on first install
   - Creates "RemoteLink Host" in Add/Remove Programs

2. Viewer Installer:
   - Language: WiX or NSIS
   - Installs remotelink-viewer.exe + dependencies
   - Start Menu shortcut
   - Uninstall option
   - Clean removal (no leftover files)

3. Code Signing (optional but recommended):
   - Self-sign EXEs with development cert
   - Avoid "This app is from an unknown publisher" warning
   - Instructions for generating self-signed certs

4. Deliverables:
   - WiX/NSIS source files
   - Build scripts (generate .msi and .exe)
   - Installation instructions (user guide)
   - Uninstall verification (checklist)

## Output Format

Return:
1. WiX project files (Host + Viewer)
2. Build instructions (Visual Studio or command-line)
3. Generated .msi / .exe files (test)
4. Installation screenshots
5. Uninstall verification checklist

User-friendly installation is critical for adoption.
```

**Test Criteria:**
- Installer runs without errors
- Files installed to correct location
- Uninstaller works cleanly
- Host binary starts after install

**Merge Checklist:**
- [ ] .msi/.exe builds successfully
- [ ] Installation tested on clean Windows VM
- [ ] Uninstall verified

---

### Agent 8.2: Documentation & README
**Role:** Technical Writer
**Duration:** 3 days
**Dependencies:** All code + installers complete

**Prompt:**

```
You are writing complete documentation for RemoteLink.

## Requirements

1. README.md (main entry point):
   - What is RemoteLink?
   - Quick start (install, run host, connect viewer)
   - Requirements (Windows 10/11, .NET runtime?)
   - Screenshots (connection dialog, connected state)
   - Troubleshooting (common errors + solutions)

2. Setup Guide:
   - Step-by-step host installation
   - Step-by-step viewer installation
   - First connection walkthrough
   - Screenshot at each step

3. CLI Reference:
   - `remotelink-host --help`
   - `remotelink-viewer --help`
   - Example commands
   - Configuration options

4. Troubleshooting:
   - "Can't connect to host"
   - "Video is very choppy"
   - "Keyboard not responding"
   - "Connection keeps dropping"
   - Solutions for each

5. Known Limitations:
   - Single monitor only (for now)
   - No audio
   - Text clipboard only
   - LAN preferred (TURN not supported)

6. Deliverables:
   - README.md (GitHub-friendly)
   - SETUP.md (installation guide)
   - TROUBLESHOOTING.md
   - CLI_REFERENCE.md
   - ARCHITECTURE.md (for developers)

## Output Format

Return:
1. All .md files (complete, formatted)
2. Screenshots (connection dialog, toolbar, etc.)
3. Example terminal output (host listening, viewer connecting)
4. Checklist for documentation completeness

Clear, jargon-free language; new users should understand immediately.
```

**Test Criteria:**
- README is clear and actionable
- Setup steps are accurate
- Screenshots match current UI
- All CLI options documented

**Merge Checklist:**
- [ ] README is complete and accurate
- [ ] No broken links or references
- [ ] Screenshots current

---

## Integration & Final Checks

### Week 8 Afternoon: Full Integration Test
**Participants:** All agents + orchestrator
**Duration:** 1 day

**Final Test Checklist:**
- [ ] Clone fresh repo
- [ ] Install dependencies (`cargo build`, `npm install`)
- [ ] Build host binary (`cargo build --release`)
- [ ] Build viewer app (`npm run tauri build`)
- [ ] Run host: observe "Listening on :5900"
- [ ] Run viewer: connection dialog appears
- [ ] Enter host IP, connect
- [ ] View desktop stream
- [ ] Move mouse: cursor follows
- [ ] Type text: appears on host
- [ ] Copy text on host: paste in viewer
- [ ] Copy text in viewer: paste on host
- [ ] Disconnect: both sides graceful
- [ ] Run 8-hour stability test
- [ ] Measure: CPU, memory, latency
- [ ] Package: .msi and .exe ready for distribution

---

## Success Metrics (Final)

| Metric | Target | Status |
|--------|--------|--------|
| Input latency | <100ms | [ ] Pass |
| FPS (LAN) | 30 | [ ] Pass |
| Host idle CPU | <1% | [ ] Pass |
| Host idle RAM | <100MB | [ ] Pass |
| Viewer idle RAM | <150MB | [ ] Pass |
| 8-hour stability | 0 crashes | [ ] Pass |
| Installer works | Clean install | [ ] Pass |
| Documentation | Complete & clear | [ ] Pass |

---

## Summary

**Total agents: 8**
**Total duration: 8 weeks**
**Lines of code (estimated): ~10k (Rust host + Viewer TS/React)**
**Infrastructure cost: $0**
**Team: Solo (you) + Claude Code agents**

Each agent task is self-contained and can be parallelized. Use this as a checklist for Claude Code sprints; each task prompt can be fed directly to an agent.

**Launch readiness: Week 8 end → ship to yourself.**

---

*Prompt version 1.0 | Agent decomposition for Antigravity RemoteLink*
