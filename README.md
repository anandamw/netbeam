<div align="center">
  <img src="src/assets/logo.png" alt="Netbeam Logo" width="120" />
  
  # ☄️ Netbeam
  
  **Lightning Fast Local P2P File Transfer**

  <p>
    <a href="#"><img src="https://img.shields.io/badge/Tauri-2.0-FFC131?style=for-the-badge&logo=tauri&logoColor=black" alt="Tauri" /></a>
    <a href="#"><img src="https://img.shields.io/badge/Rust-Black?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" /></a>
    <a href="#"><img src="https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB" alt="React" /></a>
    <a href="#"><img src="https://img.shields.io/badge/Tailwind-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white" alt="Tailwind" /></a>
  </p>
</div>

---

## 📖 Deskripsi
**Netbeam** adalah aplikasi desktop modern dan berkinerja tinggi yang memungkinkan Anda untuk mentransfer file berukuran besar antar komputer dalam satu jaringan lokal (Wi-Fi / LAN) secara instan, tanpa memerlukan koneksi internet maupun server pihak ketiga.

Didesain dengan antarmuka **Dark Mode Glassmorphism** bergaya modern dengan palet warna hitam dan kuning (*Neon Yellow*). 

---

## ✨ Fitur Utama
- **⚡ Super Cepat (Rust Core)**: Transfer data ditangani langsung oleh socket TCP *native* Rust untuk memaksimalkan batas kecepatan router/switch Anda.
- **📡 Network Radar**: Sistem *Auto-Discovery* via UDP Broadcast. Tidak perlu lagi menghafal IP teman Anda!
- **🛡️ Confirmation Dialog**: Benteng perlindungan bawaan. Terima atau tolak file yang masuk.
- **📂 Multi-file Drag & Drop**: Tarik dan lepas belasan file sekaligus ke dalam jendela antrean transfer.
- **🧠 Transfer Memory**: Aplikasi mengingat *Recent Devices* dan menyimpan *Transfer History* Anda secara lokal.
- **💬 Built-in Chat**: Ngobrol secara lokal dengan perangkat tujuan sambil menunggu transfer selesai.

---

## 🛠️ Teknologi yang Digunakan
* **Frontend**: React 19, TypeScript, Tailwind CSS, Lucide Icons.
* **Backend**: Rust, Tauri v2, Tokio (Asynchronous TCP/UDP Networking).
* **State Management**: React Hooks di Frontend & `Arc<Mutex>` di Rust Backend.

---

## 🚀 Cara Menjalankan (Development)

Pastikan Anda telah menginstal [Node.js](https://nodejs.org/), [Rust](https://rustup.rs/), dan prasyarat Tauri v2.

1. **Install dependensi node**
   ```bash
   npm install
   ```
2. **Jalankan dalam mode Development**
   ```bash
   npm run tauri dev
   ```

---

## 📦 Cara Membangun Installer (Release Build)

Untuk menghasilkan file Installer mandiri (seperti `.exe` di Windows) yang bisa dibagikan tanpa instalasi Node/Rust:

```bash
npm run tauri build
```
Setelah proses selesai (sekitar 2-5 menit), file *installer* Anda akan berada di direktori:
`src-tauri/target/release/bundle/nsis/Netbeam_1.0.0_x64-setup.exe`

---

## 📂 Struktur Proyek
```text
netbeam/
 ├── src/                  # (Frontend) React & Tailwind UI
 │    ├── App.tsx          # Antarmuka Utama & Logika Event Tauri
 │    ├── index.css        # Desain Warna dan Styling Dasar
 │    └── assets/          # Logo dan Gambar pendukung
 ├── src-tauri/            # (Backend) Rust Core
 │    ├── src/
 │    │    ├── commands/   # Komunikasi antara React <-> Rust
 │    │    ├── file/       # Logika Stream File (Sender)
 │    │    ├── network/    # TCP Server, Client, & UDP Discovery Radar
 │    │    ├── store.rs    # Manajemen Data Lokal (History)
 │    │    └── state.rs    # State Mutex & Channel Async
 │    ├── tauri.conf.json  # Konfigurasi Installer & Window
 │    └── Cargo.toml       # Dependensi Rust
 ├── PLAN.md               # Dokumen Perencanaan & Arsitektur
 └── PHASES.md             # Peta Jalan Pengembangan (Roadmap)
```

---

<div align="center">
  Dibuat dengan ❤️ untuk pertukaran data lokal tanpa batas.
</div>
