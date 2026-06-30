# 🏥 DICOM Web Viewer & Medical Image Compressor 🦀🚀

Welcome to my High-Performance DICOM Web Viewer built from scratch using **Rust** and **Dioxus**! This project is designed for doctors and hospitals to view, analyze, and compress heavy medical files directly in the browser with maximum speed. 🔥

---

## ✨ Core Features (What is done!)

### 📂 1. Smart DICOM Input & Processing
* **Parallel Processing:** Uses `Rayon` (`into_par_iter`) to load and decode multiple heavy `.dcm` files at the same time using all CPU cores. 💻⚡
* **Sidebar Preview:** All uploaded files are rendered inside a beautiful thumbnail preview sidebar. Click on any preview image, and it immediately loads into the main viewport! 🎯🖼️

### 🛠️ 2. Advanced Multi-Tool Viewport (Interactive Engine)
We built a completely modular and component-based mouse event system (`tool_set_down`, `tool_set_move`, `tool_set_up`) for live interactions:
* **🔍 Smooth Zoom:** Scale images in and out perfectly using the mouse wheel.
* **✋ Pan Tool:** Drag and move large images easily around the screen.
* **🌓 Live Window/Level (GPU-Accelerated):** Changes contrast and brightness instantly on mouse move. Instead of heavy CPU re-rendering, it uses smart math converted to GPU-driven CSS filters (`contrast()` and `brightness()`) for 0-lag performance! 🪟💡

### 🗜️ 3. Industrial-Grade Medical Compression Engine
Medical files are huge, so we integrated a dedicated compression backend:
* Supports standard **PNG** and cutting-edge **JPEG XL (Lossless)** formats.
* **✨ 70% Size Reduction:** Compresses heavy DICOM data by up to 70% without losing a single pixel of medical information (Lossless)! 📉📦

---

## 🏗️ Technical Stack

* **Frontend Framework:** `Dioxus` (Modern, Rust-native Virtual DOM) 🦀
* **Concurrency:** `Rayon` (Data parallelism) 🧵
* **DICOM Parser:** `dicom-rs` & `dicom_pixeldata` 🔬
* **Image Processing:** `image` crate 🎨

---

## 🗺️ Project Roadmap (Next Phases)

We are moving fast! Here is the plan for future versions of this software:

### 🚀 Phase 2: Ultimate Medical Tooling (Coming Soon)
* **📏 Length & Measurement Tool:** Calculate real-world dimensions (in millimeters) directly from DICOM `PixelSpacing` metadata using Pythagoras theorem.
* **🔄 Rotate & Flip:** Mirror and rotate images instantly for different anatomical views.
* **🎞️ Cine Loop (Multi-Frame Support):** Scroll through CT-Scan or MRI slices smoothly like a video player.

### 🧠 Phase 3: AI-Powered Medical Reporting (Future Vision)
* Integrating a localized AI/LLM model to analyze the image, detect anomalies, and auto-generate draft clinical reports for radiologists. 🤖📝
