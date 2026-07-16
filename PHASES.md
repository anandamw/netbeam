# 🚀 LAN Share Development Phases

Dokumen ini berisi daftar centang (checklist) untuk setiap fase pengembangan aplikasi LAN Share. Kita dapat memantau progres dengan memberikan tanda centang pada setiap tugas yang telah selesai.

---

## Phase 0 - Project Setup
*Objective: Menyiapkan fondasi proyek sebelum masuk ke networking.*

- [x] Init Tauri v2 + React 19 + TS + Vite
- [x] Setup TailwindCSS + shadcn/ui
- [x] Setup struktur folder `src-tauri`
- [x] Setup `tracing` logging
- [x] Setup konfigurasi default (port, download folder)
- [x] Setup CI dasar (build check tiap push)

## Phase 1 - Basic Networking
*Objective: Membuat koneksi TCP antar perangkat.*

- [x] Membuat TCP Server
- [x] Membuat TCP Client
- [x] Menjalankan server
- [x] Melakukan koneksi client ke server
- [x] Mengirim pesan teks
- [x] Menerima pesan teks
- [x] Handle port conflict (auto-pilih port lain jika default terpakai)

## Phase 2 - File Transfer
*Objective: Mengirim file sederhana.*

- [x] Memilih file
- [x] Membaca metadata file
- [x] Mengirim metadata
- [x] Mengirim isi file
- [x] Menyimpan file
- [x] Verifikasi ukuran file
- [x] Verifikasi checksum SHA256 penuh (bukan hanya ukuran)

## Phase 3 - Chunk Transfer
*Objective: Mengirim file besar tanpa membebani RAM.*

- [x] Chunk size 64 KB (adaptif)
- [x] Streaming file (tanpa memuat utuh ke RAM)
- [x] Progress transfer (persentase)
- [x] Speed calculation (MB/s)
- [x] Sisa waktu (ETA)
- [x] Backpressure handling (jeda kirim jika receiver lambat via ack)
- [x] Retry per-chunk jika checksum chunk gagal (CRC32 fallback)

## Phase 4 - Device Discovery
*Objective: Mendeteksi perangkat secara otomatis.*

- [x] mDNS Discovery
- [x] Implementasi UDP Broadcast
- [x] Broadcasting kehadiran (nama host, IP, port)
- [x] Mendengarkan broadcast dari perangkat lain
- [x] UI untuk daftar perangkat yang ditemukan (Radar/Daftar)

## Phase 5 - Modern UI
*Objective: Antarmuka yang modern dan responsif.*

- [x] Home Screen
- [x] Device List
- [x] File Picker
- [x] Progress Dialog
- [x] Success Dialog
- [x] Error Dialog
- [x] Cancel/Pause control di Progress Dialog

## Phase 6 - Advanced Features
*Objective: Fitur tambahan untuk meningkatkan pengalaman pengguna.*

- [x] Multiple File Transfer
- [ ] Folder Transfer
- [x] Drag & Drop
- [x] Pause & Resume
- [x] Retry
- [x] Transfer queue

## Phase 7 - Security
*Objective: Mengamankan koneksi dan transfer file.*

- [ ] Pairing Device (PIN/QR code confirmation)
- [ ] Key exchange (X25519) saat pairing
- [x] Confirmation Dialog
- [ ] AES-256-GCM Encryption pada seluruh channel setelah pairing
- [x] SHA256 Verification (per-file dan per-chunk)
- [ ] Trusted device list

## Phase 8 - History
*Objective: Mencatat riwayat transfer.*

- [ ] Transfer History (disimpan lokal)
- [ ] Recent Device
- [ ] Search
- [ ] Filter

## Phase 9 - Packaging & Release
*Objective: Mempersiapkan aplikasi untuk rilis.*

- [ ] Tauri bundler config untuk OS (Windows, macOS, Linux)
- [ ] Code signing
- [ ] Auto-update via Tauri updater plugin
- [ ] Versioning & changelog (semver)
