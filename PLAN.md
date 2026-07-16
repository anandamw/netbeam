# 📁 Netbeam Desktop App
> Aplikasi desktop / netbeam untuk berbagi file antar perangkat melalui jaringan LAN/Wi-Fi menggunakan TCP Socket.

---

# 1. Project Overview

## Deskripsi

Netbeam adalah aplikasi desktop yang memungkinkan pengguna mengirim file secara langsung antar perangkat yang berada dalam jaringan lokal (LAN/Wi-Fi) tanpa memerlukan koneksi internet.

Inspirasi aplikasi:

- SHAREit
- Nearby Share
- LANDrop
- Snapdrop

---

# 2. Goals

- Transfer file cepat melalui LAN
- Tidak membutuhkan internet
- Cross Platform (Windows, Linux, macOS)
- UI modern
- Ringan
- Aman
- Mudah digunakan

---

# 3. Tech Stack

## Frontend

- Tauri v2
- React 19
- TypeScript
- Vite
- TailwindCSS
- shadcn/ui
- **Zustand** — state management ringan untuk device list, transfer queue, dan progress (Context API akan terlalu berat untuk update progress per-chunk yang sering)
- **TanStack Query** — opsional, untuk cache status transfer/history jika nanti ada backend query yang lebih kompleks

## Backend

- Rust

## Networking

- TCP Socket
- Tokio
- **rustls** — TLS di atas TCP Socket, dipakai setelah pairing untuk mengenkripsi seluruh channel (bukan hanya payload)

## Device Discovery

- mDNS (Bonjour/Zeroconf) — via crate `mdns-sd`
- UDP Broadcast (Fallback)

## Serialization

- Serde
- serde_json
- **bincode** — untuk framing chunk biner (metadata pakai JSON, chunk pakai binary framing agar tidak ada overhead base64/JSON pada data besar)

## File Management

- tokio::fs

## Security

- **x25519-dalek** — key exchange untuk pairing device (ECDH)
- **aes-gcm** — enkripsi channel setelah pairing
- **sha2** — checksum integritas file dan per-chunk

## Utilities

- uuid
- chrono
- **thiserror** / **anyhow** — error handling terstruktur di Rust
- **tracing** — logging terstruktur (server, client, discovery events)

---

# 4. Folder Structure

```text
netbeam/

├── src/
│
├── components/
├── pages/
├── hooks/
├── services/
├── stores/          # Zustand stores
├── lib/
├── types/
│
└── src-tauri/
    │
    ├── network/
    │   ├── server.rs
    │   ├── client.rs
    │   ├── discovery.rs
    │   ├── protocol.rs
    │   ├── handshake.rs      # pairing & key exchange
    │   └── mod.rs
    │
    ├── file/
    │   ├── sender.rs
    │   ├── receiver.rs
    │   ├── progress.rs
    │   ├── checksum.rs
    │   └── mod.rs
    │
    ├── security/
    │   ├── crypto.rs         # AES-GCM encrypt/decrypt
    │   ├── pairing.rs
    │   └── mod.rs
    │
    ├── state/
    │   ├── app_state.rs      # shared state across Tauri commands (Arc<Mutex<...>>)
    │   └── mod.rs
    │
    ├── commands/
    │
    ├── config/
    │   ├── settings.rs       # persisted settings (download folder, device name, port)
    │   └── mod.rs
    │
    ├── utils/
    │
    └── main.rs
```

---

# 5. Application Architecture

```text
┌────────────────────┐
│    React UI         │
└─────────┬──────────┘
          │  invoke() / listen()
          ▼
   Tauri Commands & Events
          │
          ▼
 Rust Networking Layer (Tokio runtime)
          │
          ▼
      TCP Socket (TLS via rustls after pairing)
          │
──────── LAN ────────
          │
      TCP Socket
          │
          ▼
 Rust Networking Layer
          │
          ▼
      React UI
```

### Concurrency Model

- Setiap koneksi masuk ditangani sebagai `tokio::spawn` task terpisah — server tidak boleh blocking pada satu transfer.
- Progress dan event dikirim dari task Rust ke frontend lewat **Tauri Event system** (`app_handle.emit()`), bukan polling dari React.
- State global (daftar device, transfer aktif) disimpan di `Arc<Mutex<AppState>>` yang dibagikan lintas command.
- Setiap transfer punya `transfer_id` (UUID) agar bisa di-cancel/pause secara independen tanpa mengganggu transfer lain.

---

# 6. Development Roadmap

## Phase 0 - Project Setup *(baru ditambahkan — sebelumnya belum ada tahap ini)*

### Objective

Menyiapkan fondasi proyek sebelum masuk ke networking.

### Tasks

- [ ] Init Tauri v2 + React + TS + Vite
- [ ] Setup TailwindCSS + shadcn/ui
- [ ] Setup struktur folder `src-tauri`
- [ ] Setup `tracing` logging
- [ ] Setup konfigurasi default (port, download folder)
- [ ] Setup CI dasar (build check tiap push)

## Phase 1 - Basic Networking

### Objective

Membuat koneksi TCP antar perangkat.

### Tasks

- [ ] Membuat TCP Server
- [ ] Membuat TCP Client
- [ ] Menjalankan server
- [ ] Melakukan koneksi client ke server
- [ ] Mengirim pesan teks
- [ ] Menerima pesan teks
- [ ] Handle port conflict (auto-pilih port lain jika default terpakai)

## Phase 2 - File Transfer

### Objective

Mengirim file sederhana.

### Tasks

- [ ] Memilih file
- [ ] Membaca metadata file
- [ ] Mengirim metadata
- [ ] Mengirim isi file
- [ ] Menyimpan file
- [ ] Verifikasi ukuran file
- [ ] Verifikasi checksum SHA256 penuh (bukan hanya ukuran)

## Phase 3 - Chunk Transfer

### Objective

Mengirim file besar tanpa membebani RAM.

### Tasks

- [ ] Chunk size 64 KB (adaptif — lihat catatan protokol di bagian 8)
- [ ] Streaming file
- [ ] Progress transfer
- [ ] Speed calculation
- [ ] ETA
- [ ] Backpressure handling (jangan buffer chunk lebih cepat dari kecepatan tulis disk penerima)
- [ ] Retry per-chunk jika checksum chunk gagal (tanpa mengulang seluruh file)

## Phase 4 - Device Discovery

### Objective

Mendeteksi perangkat secara otomatis.

### Tasks

- [ ] mDNS Discovery
- [ ] UDP Broadcast
- [ ] Device List
- [ ] Auto Refresh
- [ ] Handle duplikat device (device yang sama muncul dari mDNS dan UDP broadcast)

## Phase 5 - Modern UI

### Tasks

- [ ] Home Screen
- [ ] Device List
- [ ] File Picker
- [ ] Progress Dialog
- [ ] Success Dialog
- [ ] Error Dialog
- [ ] Cancel/Pause control di Progress Dialog

## Phase 6 - Advanced Features

### Tasks

- [ ] Multiple File Transfer
- [ ] Folder Transfer
- [ ] Drag & Drop
- [ ] Pause
- [ ] Resume
- [ ] Retry
- [ ] Transfer queue (antrian jika kirim ke banyak device sekaligus)

## Phase 7 - Security

### Tasks

- [ ] Pairing Device (PIN/QR code confirmation)
- [ ] Key exchange (X25519) saat pairing
- [ ] Confirmation Dialog (terima/tolak permintaan device baru)
- [ ] AES-256-GCM Encryption pada seluruh channel setelah pairing
- [ ] SHA256 Verification (per-file dan per-chunk)
- [ ] Trusted device list (device yang sudah pernah dipasangkan tidak perlu konfirmasi ulang)

## Phase 8 - History

### Tasks

- [ ] Transfer History (disimpan lokal, misal SQLite via `rusqlite` atau file JSON)
- [ ] Recent Device
- [ ] Search
- [ ] Filter

## Phase 9 - Packaging & Release *(baru ditambahkan)*

### Tasks

- [ ] Tauri bundler config untuk `.msi`/`.exe` (Windows), `.dmg` (macOS), `.deb`/`.AppImage` (Linux)
- [ ] Code signing (minimal untuk Windows/macOS agar tidak diblokir SmartScreen/Gatekeeper)
- [ ] Auto-update via Tauri updater plugin
- [ ] Versioning & changelog (semver)

---

# 7. Communication Flow

## Pairing (baru — sebelumnya tidak ada di dokumen asli, padahal Phase 7 butuh ini)

```text
Discover Device
      │
      ▼
Request Pairing (kirim device_id + public key)
      │
      ▼
Tampilkan Confirmation Dialog di kedua sisi
      │
      ▼
Tukar Public Key (X25519 ECDH)
      │
      ▼
Turunkan Shared Secret → Session Key (AES-256-GCM)
      │
      ▼
Simpan sebagai Trusted Device
      │
      ▼
Channel Terenkripsi Siap Dipakai
```

## Send File

```text
Select File
      │
      ▼
Read Metadata
      │
      ▼
Connect TCP (pakai session key jika device sudah paired)
      │
      ▼
Send Metadata
      │
      ▼
Send Chunk
      │
      ▼
Progress
      │
      ▼
Verify Checksum
      │
      ▼
Finish
```

## Receive File

```text
Waiting Connection
        │
        ▼
Receive Metadata
        │
        ▼
Confirmation Dialog (Accept/Reject)
        │
        ▼
Create File (tulis ke temp file dulu, misal .part)
        │
        ▼
Receive Chunk
        │
        ▼
Write File
        │
        ▼
Progress
        │
        ▼
Verify Checksum → Rename dari .part ke nama asli
        │
        ▼
Finish
```

---

# 8. Transfer Protocol

## Handshake / Hello *(baru ditambahkan)*

```json
{
  "type": "hello",
  "protocol_version": 1,
  "device_id": "uuid-v4",
  "device_name": "Ananda-PC",
  "os": "windows"
}
```

> Catatan: sertakan `protocol_version` sejak awal — ini penting supaya versi client/server yang berbeda di masa depan bisa saling menolak koneksi dengan pesan jelas, bukan crash saat parsing.

## Metadata

```json
{
  "type": "metadata",
  "transfer_id": "uuid-v4",
  "file_name": "photo.jpg",
  "file_size": 1520000,
  "mime": "image/jpeg",
  "checksum_sha256": "hex-digest-of-full-file",
  "chunk_size": 65536
}
```

## Chunk

Framing biner (bukan JSON, untuk menghindari overhead encoding pada data besar):

```text
[ chunk_index: u64 ][ chunk_len: u32 ][ chunk_checksum: u32 (CRC32) ][ binary data ]
```

- `chunk_index` — untuk mendeteksi chunk hilang/out-of-order dan mendukung resume
- CRC32 per-chunk untuk deteksi korupsi cepat; SHA256 penuh tetap dipakai untuk verifikasi akhir

## Ack *(baru ditambahkan)*

```json
{
  "type": "ack",
  "transfer_id": "uuid-v4",
  "chunk_index": 42
}
```

> Diperlukan untuk flow control sederhana: pengirim menunggu ack sebelum mengirim terlalu banyak chunk di depan (mencegah membanjiri penerima yang disknya lebih lambat dari network).

## Error *(baru ditambahkan)*

```json
{
  "type": "error",
  "transfer_id": "uuid-v4",
  "code": "CHECKSUM_MISMATCH",
  "message": "Chunk 42 failed checksum verification"
}
```

Kode error minimal yang perlu didefinisikan: `PROTOCOL_VERSION_MISMATCH`, `PAIRING_REJECTED`, `CHECKSUM_MISMATCH`, `DISK_FULL`, `TRANSFER_CANCELLED`, `TIMEOUT`.

## Finish

```json
{
  "type": "finish",
  "transfer_id": "uuid-v4",
  "status": "success"
}
```

---

# 9. UI Pages

## Home

- Device List
- Search Device
- Settings

## Send

- Select File
- Drag & Drop
- Recent Files

## Receive

- Incoming Request
- Accept
- Reject

## Pairing *(baru ditambahkan)*

- Konfirmasi device baru (nama, OS, fingerprint key)
- Trusted device list (kelola/hapus device yang sudah dipasangkan)

## Progress

- File Name
- Speed
- Percentage
- Remaining Time
- Cancel button

## History

- Success
- Failed
- Retry

## Settings

- Download Folder
- Device Name
- Auto Discovery
- Theme
- Port (advanced, opsional)
- Manage Trusted Devices

---

# 10. Future Features

- QR Code Pairing
- Send Clipboard
- Send Text
- Send Image
- Send Video
- Send Folder
- Compression
- Resume Transfer
- Auto Update
- Dark Mode
- Light Mode
- Notifications
- Tray Icon
- Portable Mode
- LAN Chat
- Broadcast File
- Multi Device Transfer
- Android Client
- iOS Client

---

# 11. Development Milestones

| Milestone | Status |
|-----------|--------|
| Project Setup | ⬜ |
| TCP Server | ⬜ |
| TCP Client | ⬜ |
| Text Messaging | ⬜ |
| File Transfer | ⬜ |
| Chunk Transfer | ⬜ |
| Progress Bar | ⬜ |
| Device Discovery | ⬜ |
| Modern UI | ⬜ |
| Drag & Drop | ⬜ |
| Folder Transfer | ⬜ |
| Pairing & Encryption | ⬜ |
| History | ⬜ |
| Packaging & Release | ⬜ |
| Release v1.0 | ⬜ |

---

# 12. MVP Features

- ✅ TCP Socket Communication
- ✅ Automatic Device Discovery
- ✅ Send File
- ✅ Receive File
- ✅ Transfer Progress
- ✅ Transfer Speed
- ✅ ETA
- ✅ Drag & Drop
- ✅ Cross Platform
- ✅ Basic Pairing Confirmation (minimal: dialog terima/tolak, encryption bisa menyusul post-MVP)

---

# 13. Non-Functional Requirements *(baru ditambahkan)*

- **Reliability**: transfer harus tahan terhadap disconnect sementara (retry otomatis, bukan gagal total)
- **Performance target**: mendekati kecepatan maksimum LAN/Wi-Fi (bottleneck disk I/O, bukan protokol)
- **Resource usage**: idle memory footprint rendah karena akan berjalan di tray/background
- **Portability**: build harus reproducible di CI untuk ketiga platform (Windows, Linux, macOS)

---

# 14. Long-Term Vision

Menjadi aplikasi transfer file lokal yang cepat, ringan, aman, dan modern sebagai alternatif open-source untuk SHAREit, Nearby Share, dan LANDrop, dengan dukungan lintas platform serta pengalaman pengguna yang sederhana.