# 🎨 UI Design Plan
## Web-Based Inventory Management & Repair Service
### Dinas Komunikasi dan Informatika Kabupaten Sumenep

---

# Design Concept

**Theme**
> Modern Dark Dashboard

**Design Style**
- Minimalist
- Clean Interface
- Government Professional
- Material Design Inspired
- Soft Rounded Corner (10–16px)
- Soft Shadow
- High Readability

---

# Color Palette

## Primary

| Color | Hex | Usage |
|---------|------|----------------|
| Primary Yellow | #FACC15 | Button, Active Menu, Icon |
| Primary Hover | #FFD54A | Hover Button |
| Primary Press | #EAB308 | Active State |

---

## Background

| Color | Hex |
|---------|------|
| Main Background | #121212 |
| Secondary Background | #1A1A1A |
| Card | #1E1E1E |
| Sidebar | #181818 |
| Navbar | #1D1D1D |

---

## Text

| Color | Hex |
|---------|------|
| Primary | #FFFFFF |
| Secondary | #BDBDBD |
| Muted | #8A8A8A |
| Disabled | #666666 |

---

## Border

| Color | Hex |
|---------|------|
| Border | #2F2F2F |
| Divider | #343434 |

---

## Status Color

| Status | Hex |
|---------|------|
| Success | #22C55E |
| Warning | #FACC15 |
| Error | #EF4444 |
| Info | #3B82F6 |

---

# Typography

## Font

Poppins

Alternative:
- Inter
- Nunito Sans

---

## Font Size

| Element | Size |
|-----------|--------|
| H1 | 32px |
| H2 | 28px |
| H3 | 24px |
| H4 | 20px |
| H5 | 18px |
| Body | 16px |
| Small | 14px |
| Caption | 12px |

---

# Border Radius

| Component | Radius |
|------------|-----------|
| Button | 10px |
| Card | 16px |
| Input | 10px |
| Modal | 18px |

---

# Shadow

Small

```css
0 2px 6px rgba(0,0,0,.2)
```

Medium

```css
0 8px 20px rgba(0,0,0,.3)
```

Large

```css
0 20px 40px rgba(0,0,0,.35)
```

---

# Layout Structure

```
----------------------------------------------------
 Navbar
----------------------------------------------------

 Sidebar |            Main Content
         |
         |
         |
         |

----------------------------------------------------
 Footer
----------------------------------------------------
```

---

# Sidebar

Background

```
#181818
```

Menu Active

```
Background : #FACC15
Text : #121212
Icon : #121212
```

Menu Hover

```
Background : #2A2A2A
```

Menu Text

```
#BDBDBD
```

---

# Navbar

Height

```
70px
```

Contains

- Logo
- Search
- Notification
- User Profile
- Theme Toggle (optional)

Background

```
#1D1D1D
```

---

# Dashboard

Widget

```
Inventory

Repair Queue

Available Items

Borrowed Items

Completed Repairs

Pending Repairs
```

Card

```
Background
#1E1E1E

Radius
16px

Padding
24px
```

---

# Table Design

Header

```
Background
#252525
```

Row

```
Odd
#1E1E1E

Even
#222222
```

Hover

```
#2D2D2D
```

Selected

```
#FACC15
```

---

# Form Design

Input

Background

```
#252525
```

Border

```
#3A3A3A
```

Focus

```
Border
#FACC15
```

Text

```
#FFFFFF
```

Placeholder

```
#777777
```

---

# Button

## Primary

Background

```
#FACC15
```

Text

```
#121212
```

Hover

```
#FFD54A
```

---

## Secondary

Background

```
Transparent
```

Border

```
#FACC15
```

Text

```
#FACC15
```

---

## Danger

```
#EF4444
```

---

## Success

```
#22C55E
```

---

# Badge

Available

```
Green
```

Borrowed

```
Yellow
```

Broken

```
Red
```

Maintenance

```
Blue
```

---

# Modal

Background

```
#1E1E1E
```

Overlay

```
rgba(0,0,0,.6)
```

Radius

```
18px
```

---

# Notification

Success

```
Green Left Border
```

Warning

```
Yellow Left Border
```

Error

```
Red Left Border
```

Info

```
Blue Left Border
```

---

# Icon Style

- Rounded
- Outline
- 2px Stroke
- Material Symbols Rounded / Lucide Icons
- Active Icon → Yellow

---

# Animation

Duration

```
200ms
```

Transition

```
ease-in-out
```

Hover Scale

```
1.02
```

Button Press

```
0.98
```

---

# Dashboard Pages

## Authentication
- Login
- Forgot Password

---

## Dashboard
- Summary Cards
- Activity Timeline
- Charts
- Recent Repairs

---

## Inventory

- Data Barang
- Tambah Barang
- Detail Barang
- Edit Barang
- Mutasi Barang
- Kategori
- Lokasi
- Supplier

---

## Repair Service

- Pengajuan Perbaikan
- Diagnosis
- Proses Perbaikan
- Sparepart
- Riwayat Servis
- Penyelesaian

---

## Borrowing

- Peminjaman
- Pengembalian
- Approval
- History

---

## Users

- Administrator
- Teknisi
- Pegawai
- Kepala Bidang

---

## Reports

- Inventory Report
- Repair Report
- Borrowing Report
- Asset Report
- Printable PDF
- Export Excel

---

## Settings

- Profile
- Roles & Permission
- System Configuration
- Backup Database
- Log Activity

---

# UX Principles

- Maksimal 3 klik untuk mencapai fitur utama.
- Konsistensi warna dan ikon di seluruh aplikasi.
- Kontras tinggi untuk kenyamanan penggunaan jangka panjang.
- Responsif untuk desktop, tablet, dan perangkat mobile.
- Feedback visual pada setiap aksi (hover, loading, success, error).
- Form sederhana dengan validasi yang jelas dan pesan kesalahan yang informatif.
- Dashboard menyajikan ringkasan informasi penting secara cepat agar pengguna dapat memantau kondisi inventaris dan layanan perbaikan tanpa harus membuka banyak halaman.