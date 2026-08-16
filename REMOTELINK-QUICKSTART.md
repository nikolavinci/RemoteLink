# RemoteLink MVP: Quick Start Guide

**What:** Minimal personal remote desktop tool (not a market product)
**Build time:** 6–8 weeks solo with Claude Code agents
**Cost:** $0 infrastructure
**Scope:** Mouse, keyboard, clipboard (text) over direct P2P TCP+TLS

---

## The Vision

Install on two Windows machines. Run host. Run viewer. Enter IP. Control the other machine instantly. No servers, no pairing UI, no complexity.

```
Host: $ remotelink-host --listen
      Device ID: 7F3K-92LM | Listening on :5900

Viewer: $ remotelink-viewer
        Enter host: 192.168.1.100:5900
        [Connected]
        → See desktop, control mouse/keyboard, sync clipboard
```

---

## Architecture (1-Minute Version)

```
Host (Windows)                         Viewer (Windows)
┌─────────────────────┐              ┌─────────────────────┐
│ Desktop Capture     │ ─────TLS────→ │ H.264 Decoder       │
│ ↓                   │              │ ↓                   │
│ H.264 Encoder       │              │ WebGL Renderer      │
│ ↓                   │              │ ↓                   │
│ TCP Server :5900    │ ←────TLS───── │ Input Capture       │
│ (SendInput)         │              │ (Mouse/Keyboard)    │
│ (Clipboard polling) │              │ (Clipboard sync)    │
└─────────────────────┘              └─────────────────────┘
```

**Why TLS + TCP?**
- Simple to implement (no ICE, no STUN/TURN complexity)
- Fast on LAN (<10ms latency)
- SSH tunnelable for remote (via port forwarding)
- Self-signed cert + public key pinning = secure enough for personal use

---

## Tech Stack

| Component | Technology | Why |
|-----------|-----------|-----|
| Host Agent | Rust | Fast, memory-safe, native Windows APIs |
| Screen Capture | DirectX11 DDA API | Efficient, hardware-accelerated |
| Encoding | H.264 (NVENC/libx264) | Ubiquitous, low latency |
| Networking | Rust + Tokio | Async I/O, TLS 1.3 |
| Viewer | Tauri + React | Lightweight desktop app |
| Decoding | ffmpeg.wasm | Pure WASM, works in browser context |
| Rendering | WebGL | Hardware-accelerated |

---

## Success Criteria (Launch Ready)

- [ ] Input latency <100ms (click to response)
- [ ] 30 FPS sustained on LAN
- [ ] Host idle: <1% CPU, <100MB RAM
- [ ] Viewer idle: <2% CPU, <150MB RAM
- [ ] 8-hour session without crash
- [ ] Graceful disconnect/reconnect
- [ ] Installer + docs complete

---

## The 8-Sprint Roadmap

| Sprint | Week | What | Agents | Status |
|--------|------|------|--------|--------|
| 1 | 1 | Desktop capture + H.264 encoding | 1.1, 1.2 (parallel) | ⏳ |
| 2 | 2 | TCP server + input injection + clipboard | 2.1, 2.2 (parallel) | ⏳ |
| 3 | 3 | Video streaming + adaptive bitrate | 3.1, 3.2 | ⏳ |
| 4 | 4 | Tauri scaffold + H.264 decoding | 4.1, 4.2 (parallel) | ⏳ |
| 5 | 5 | Input capture + Tauri integration | 5.1, 5.2 | ⏳ |
| 6 | 6 | Clipboard sync + UI polish | 6.1, 6.2 (parallel) | ⏳ |
| 7 | 7 | Stability testing + perf optimization | 7.1, 7.2 | ⏳ |
| 8 | 8 | Installer + documentation | 8.1, 8.2 (parallel) | ⏳ |

**Parallel execution:** Agents can run concurrently within sprints (e.g., 1.1 and 1.2 simultaneously in Week 1).

---

## How to Use These Prompts

### Main Build Prompt
**File:** `remotedesktop-antigravity-prompt.md`

This is the complete specification. Reference it for:
- Detailed requirements (FR-01 through FR-16)
- Protocol specification (message framing, wire format)
- Performance targets (latency, FPS, memory)
- Technology stack rationale
- Risk register + mitigation strategies

**When:** Read this first; refer throughout build.

### Agent Task Decomposition
**File:** `remotedesktop-agent-tasks.md`

8 self-contained agent prompts (one per critical task). Each includes:
- Clear requirements
- Deliverables
- Test criteria
- Merge checklist

**How to use:**
1. Copy the prompt for Agent 1.1 (Desktop Capture)
2. Paste into Claude Code with instruction: "Use this prompt to guide your implementation"
3. Agent returns complete, tested source code
4. Code review + merge
5. Repeat for Agent 1.2 in parallel
6. After Sprint 1 integrates, move to Sprint 2 agents

---

## Execution Flow (Week-by-Week)

### Week 1: Host Foundation
```
Monday–Wednesday:   Agent 1.1 (Desktop Capture)
                    └→ Returns capture loop + FPS test
                    
Monday–Wednesday:   Agent 1.2 (H.264 Encoder)
                    └→ Returns encoder module + test frames
                    
Thursday–Friday:    Integration sprint
                    └→ Wire 1.1 → 1.2 → file output
                    └→ Test: produces valid H.264 files
```

### Week 2: Host Networking & Input
```
Monday–Wednesday:   Agent 2.1 (TCP Server + TLS)
                    └→ Returns listening server + test client
                    
Monday–Wednesday:   Agent 2.2 (Input Handler + Clipboard)
                    └→ Returns input injection + clipboard monitor
                    
Thursday–Friday:    Integration
                    └→ Wire network + input
                    └→ Test: send mouse events from client, verify on host
```

### Week 3: Host Streaming
```
Monday–Wednesday:   Agent 3.1 (Video Streaming Protocol)
                    └→ Returns message framing + serialization
                    
Wednesday–Friday:   Agent 3.2 (Adaptive Bitrate)
                    └→ Returns metrics collection + bitrate control
```

### Week 4: Viewer Foundation
```
Monday–Wednesday:   Agent 4.1 (Tauri Scaffold + Decoder)
                    └→ Returns React app + ffmpeg.wasm integration
                    
Wednesday–Friday:   Agent 4.2 (WebGL Rendering)
                    └→ Returns video canvas + shader code
```

### Week 5: Viewer Input & Integration
```
Monday–Thursday:    Agent 5.1 (Input Capture)
                    └→ Returns useInputCapture hook
                    
Monday–Thursday:    Agent 5.2 (Tauri Backend Integration)
                    └→ Returns Rust TCP client + IPC handlers
                    
Friday:             End-to-end test
                    └→ Host sends video → Viewer decodes & renders
                    └→ Viewer sends input → Host receives
```

### Week 6: Clipboard & Polish
```
Monday–Wednesday:   Agent 6.1 (Clipboard Sync)
                    └→ Manual copy/paste test
                    
Wednesday–Friday:   Agent 6.2 (UI Polish)
                    └→ Connection dialog, toolbar, status display
```

### Week 7: Stability & Performance
```
Monday–Wednesday:   Agent 7.1 (Error Handling & 8-Hour Test)
                    └→ Test matrix, crash recovery, stress testing
                    
Wednesday–Friday:   Agent 7.2 (Performance Optimization)
                    └→ Profile, measure latency, apply optimizations
```

### Week 8: Release
```
Monday–Wednesday:   Agent 8.1 (Installer & Signing)
                    └→ .msi and .exe ready
                    
Wednesday–Friday:   Agent 8.2 (Documentation)
                    └→ README, setup guide, troubleshooting
                    
Friday:             Full integration test
                    └→ Fresh Windows VM install → launch → connect
```

---

## Quick Performance Checklist

After each sprint, measure:

```bash
# Host Agent
$ remotelink-host --listen &
$ sleep 5
$ tasklist | grep remotelink
  # Should show <100MB for idle

# Run 30 minutes, monitor:
$ Get-Process remotelink-host | Select-Object WorkingSet, CPU

# Viewer
$ remotelink-viewer &
$ Connect to host
$ # Monitor Chrome DevTools: FPS, memory, network

# End-to-end test
$ Time mouse click → observe cursor movement
$ Expected: <100ms
```

---

## Decision Reference

| Decision | Choice | Why |
|----------|--------|-----|
| Networking | Direct TCP + TLS | No relay infra cost; SSH-tunnelable |
| Encoding | H.264 + NVENC fallback | Widely supported; hardware encoders fast |
| Clipboard | Text only, poll-based | MVP simplicity; rich content later |
| Cursor | Send separately (not in video) | Saves bandwidth, improves responsiveness |
| Connection model | One session per host | Simpler state management |
| FPS target | 30 (LAN), 15–20 (poor) | Responsive; adaptive fallback |
| Bitrate strategy | Adaptive, 500k–8M | Graceful degradation |

---

## Common Questions Answered

**Q: Will this work over the internet?**
A: Yes, via SSH port forwarding: `ssh -L 5900:localhost:5900 remote-host` then connect to `localhost:5900`.

**Q: What if GPU doesn't support NVENC?**
A: Falls back to CPU-based libx264; slower but still works at 15–20 FPS.

**Q: Can I use this for unattended access?**
A: For MVP, no. You run both on trusted machines. Future: add TLS cert-based unattended mode.

**Q: Multi-monitor support?**
A: MVP captures primary only; extend in Phase 2 with monitor selection.

**Q: Audio?**
A: Out of scope for MVP; add in Phase 3.

**Q: File transfer?**
A: Out of scope for MVP; use SCP/SMB for now.

---

## Next Steps

1. **Read:** `remotedesktop-antigravity-prompt.md` (full specification)
2. **Start:** Agent 1.1 from `remotedesktop-agent-tasks.md` (Desktop Capture)
3. **Run:** Feed prompt to Claude Code
4. **Build:** Code appears; test locally
5. **Merge:** Commit + move to Agent 1.2 (parallel)
6. **Iterate:** Weekly sprints, 8 weeks total

---

## Files & Locations

- **Main Specification:** `/home/claude/remotedesktop-antigravity-prompt.md` (16 sections, 500+ lines)
- **Agent Tasks:** `/home/claude/remotedesktop-agent-tasks.md` (8 agents, executable)
- **Memory:** `/areas/remotelink-minimal.md` (tracking + decisions)
- **This File:** `/home/claude/REMOTELINK-QUICKSTART.md` (reference)

---

## Success = Ship

You win when:
- Both binaries exist and run
- One click connects you to the other computer
- Mouse moves, keyboard types, clipboard syncs
- It doesn't crash
- It doesn't leak memory
- It survives a workday

**Launch target: End of Week 8 (late October 2026)**

---

*RemoteLink MVP | Personal P2P remote desktop | 6–8 week build | Antigravity Prompt v1.0*
