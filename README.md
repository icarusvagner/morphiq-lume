# 🌟 Morphiq Lume

> **Transforming Human Resource Operations with Clarity, Precision, and Intelligence.**

Morphiq Lume is a modern, secure, and efficient desktop-based Human Resource Management Office (HRIS) system built using **Rust** and the **Iced GUI framework**. Designed for both government and private institutions, it offers a modular architecture for handling payroll, employee records, attendance, events, and more.

---

## 📖 Theory Behind Morphiq Lume

> “The Intelligent Evolution of Human Resource Empowerment”

Traditional HR systems are either too rigid or overly bloated. **Morphiq Lume** introduces a new paradigm:

> **“Clarity Through Adaptability”**

Where:
- **Morphiq** = Modular + Adaptive logic that evolves with organizational needs  
- **Lume** = Visual clarity, structured insight, and human-centered workflows  

Together, Morphiq Lume is built to reveal clarity in people operations while staying lean and future-ready.

---

## 🧩 Core Features

| Module               | Description |
|----------------------|-------------|
| 🧑‍💼**Employee Management** | Core of the system; rich profiles, rank/dept mapping, data tagging |
| 💰 **Payroll System**         | Auto-sync with attendance/leaves, rules engine for computation |
| ⏱️ **Attendance Tracking**   | Manual input + biometrics-ready interface |
| 🗓️ **Event Management**      | Plan, invite, and track employee participation |
| 🌴 **Leave Requests**        | Custom leave types, quota tracking, approval workflow |
| 📄 **Document Inbox**        | Intake system for email documents (resumes, letters, etc.) |

---

## 🌐 Optional & Upcoming Modules

- 🔐 Biometric Integration (ZKTeco, Suprema)
- 📊 HR Dashboards (Iced charts and visual reports)
- 🧾 Asset Issuance & Management
- 🎯 Performance Review System
- 🧪 Recruitment / Applicant Tracking System

---

## 🛠️ Project Stack

- 🦀 **Rust** — for speed, safety, and offline-capable architecture
- 🧊 **Iced** — lightweight, native GUI framework for desktop apps
- 🗂️ `confy` — local config management
- 🔄 `serde`, `uuid`, `chrono`, `directories` — data handling and time tracking

---

## 📦 Getting Started

### 🔧 Requirements
- [Rust Toolchain](https://www.rust-lang.org/tools/install)
- Works on Linux 🐧, Windows 🪟, macOS 🍎

### 🏃 Run It

```bash
git clone https://github.com/your-org/morphiq_lume.git
cd morphiq_lume
cargo run --release
```

---

## 📁 Project Structure

Morphiq Lume uses a **feature-driven architecture**, adapted for Rust + Iced.

```
  src
   ├─ core
   │  ├─ error.rs
   │  ├─ mod.rs
   │  ├─ theme
   │  │  ├─ colors.rs
   │  │  ├─ dark.rs
   │  │  ├─ fonts.rs
   │  │  ├─ light.rs
   │  │  └─ mod.rs
   │  ├─ types.rs
   │  └─ utils
   │     ├─ constants
   │     │  ├─ mod.rs
   │     │  └─ widgets.rs
   │     ├─ message.rs
   │     └─ mod.rs
   ├─ data
   │  ├─ mod.rs
   │  ├─ models
   │  │  └─ mod.rs
   │  └─ repositories
   │     └─ mod.rs
   ├─ features
   │  ├─ attendance
   │  │  └─ mod.rs
   │  ├─ auth
   │  │  ├─ auth_msg.rs
   │  │  ├─ lockscreen.rs
   │  │  ├─ login.rs
   │  │  └─ mod.rs
   │  ├─ dashboard
   │  │  ├─ dashboard_msg.rs
   │  │  ├─ mod.rs
   │  │  └─ view.rs
   │  ├─ employees
   │  │  ├─ addition_fields.rs
   │  │  ├─ create.rs
   │  │  ├─ employee_msg.rs
   │  │  ├─ employee_type.rs
   │  │  ├─ mod.rs
   │  │  └─ view.rs
   │  ├─ mod.rs
   │  ├─ payroll
   │  │  └─ mod.rs
   │  └─ settings
   │     └─ mod.rs
   ├─ main.rs
   ├─ morphiq.rs
   ├─ router.rs
   ├─ styles
   │  ├─ bar.rs
   │  ├─ button.rs
   │  ├─ color_remote.rs
   │  ├─ container.rs
   │  ├─ datepicker.rs
   │  ├─ donut.rs
   │  ├─ menu.rs
   │  ├─ mod.rs
   │  ├─ palette.rs
   │  ├─ pick_list.rs
   │  ├─ rule.rs
   │  ├─ scrollable.rs
   │  ├─ style_constant.rs
   │  ├─ style_type.rs
   │  ├─ svg.rs
   │  ├─ table.rs
   │  ├─ text.rs
   │  └─ text_input.rs
   ├─ widgets
   │  ├─ mod.rs
   │  └─ widget_custom_card.rs
   ├─ morphiq.rs
   ├─ router.rs
   └─ main.rs
```

---

## 💻 Screenshots

Coming soon... 🧊

---

## 🧠 System Philosophy

### 📘 Theory: "Clarity Through Adaptability"

Each module in Morphiq Lume acts as a dynamic unit that:

- Evolves with organizational needs (Morphiq)
- Exposes useful insight without complexity (Lume)

### 🧬 Architecture Highlights

| Principle      | Implementation                                            |
|----------------|------------------------------------------------------------|
| Offline-first  | Native app, no browser or Electron dependencies            |
| Modular        | All core modules live independently inside `core/` and `view/` |
| Secure         | No external telemetry, encrypted config, local-first       |
| Extensible     | Easily connect biometric devices, performance review engines |

---

## 📊 Internal Pitch Snapshot

### 🎯 Vision
> “Empowering organizations with a lean, intelligent, and secure HR ecosystem.”

### 🧩 Roadmap

| Phase | Modules                       | Timeline  |
|-------|-------------------------------|-----------|
| Alpha | Employee, Payroll, Leaves     | Month 1-2 |
| Beta  | Attendance, Docs, Events      | Month 3-4 |
| Stable| Biometrics, Dashboard, Installers | Month 5-6 |

---

## 🤝 Contributing

Want to help build the future of HR systems?

We’re open to:

- UI/UX designers (Iced or Tailwind mockups)
- Rust developers
- HR testers

📄 See `CONTRIBUTING.md` (coming soon)

---

## 📜 License

Choose either:

- MIT License (permissive)
- Apache 2.0 (business-friendly and patent-protective)

---

## 👤 Maintainers

- **Akaza Ruthven** – Founder, Engineer  
- **YourTeamName** – Systems + UI

---

## 📬 Contact

📧 morphiq@castlebytesolutions.com  
🌐 [www.morphiqlume.com](http://www.morphiqlume.com) _(to be launched)_

---

> _“Designed for clarity. Built for change.”_
