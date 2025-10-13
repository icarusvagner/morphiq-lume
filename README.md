# Cebu Tours & Adventures Admin System — Software Planning

## 1. Overview
The **Cebu Tours & Adventures Admin System** is designed to streamline the management of tours, bookings, payments, fleets, customer relations, and analytics.  
It consists of two main platforms:

- **Web Admin Portal:** For online management accessible via browsers.
- **Desktop Admin Application:** For offline and on-site operations with local caching and synchronization.

Both platforms share a unified **PostgreSQL + Axum backend API**, ensuring data consistency and security.

---

## 2. Stakeholders

| Stakeholder | Role | Description |
|--------------|------|-------------|
| System Administrator | Oversees entire platform | Manages user roles, system settings, and security. |
| Tour Manager | Manages tour offerings | Creates, updates, and archives tour packages. |
| Booking Officer | Handles reservations | Manages customer bookings, schedules, and cancellations. |
| Fleet Manager | Manages vehicles and drivers | Monitors availability, assignments, and maintenance schedules. |
| Finance Officer | Handles billing and payments | Tracks invoices, refunds, and generates reports. |
| Marketing Manager | Manages promotions | Oversees discounts, campaigns, and customer engagement. |
| Customer Support | Assists travelers | Handles inquiries, feedback, and complaints. |
| Executive / Owner | Reviews analytics | Views KPIs, reports, and strategic dashboards. |

---

## 3. Features per Stakeholder

### 3.1 System Administrator
**Goals:**
- Ensure system reliability, access control, and data integrity.

**Features:**
- Manage user accounts and roles.
- Configure system-wide settings (payment gateways, APIs, etc.).
- Monitor system logs and audit trails.
- Set up backups, replication, and data synchronization.
- Desktop-only: Manage local database sync and offline mode.

**Applies to:** Web Admin, Desktop Admin

---

### 3.2 Tour Manager
**Goals:**
- Create and maintain tour offerings with detailed itineraries.

**Features:**
- Add/edit/delete tours (name, description, itinerary, images, price).
- Tag tours by categories (island hopping, adventure, cultural).
- Manage tour availability and capacity.
- Upload media galleries.
- Generate dynamic tour URLs with SEO metadata.
- Preview listings before publishing.

**Applies to:** Web Admin

---

### 3.3 Booking Officer
**Goals:**
- Efficiently process customer reservations.

**Features:**
- Search and filter bookings by date, tour, or customer.
- Manage bookings: confirm, cancel, reschedule.
- Track payment status.
- Handle manual bookings and walk-ins (Desktop-specific).
- Send automated email confirmations or reminders.
- Access real-time availability synced with fleet schedules.

**Applies to:** Web Admin, Desktop Admin

---

### 3.4 Fleet Manager
**Goals:**
- Ensure efficient allocation and utilization of transport resources.

**Features:**
- Register and manage vehicles.
- Assign drivers to tours.
- Schedule maintenance and track vehicle status.
- Generate daily trip manifests.
- Track GPS or logs (if supported by API).
- Offline updates synced upon network restoration (Desktop).

**Applies to:** Desktop Admin

---

### 3.5 Finance Officer
**Goals:**
- Maintain accurate financial records and reports.

**Features:**
- View and approve payment transactions.
- Generate invoices and receipts.
- Manage refunds and adjustments.
- Integrate with accounting tools (e.g., QuickBooks API).
- Generate monthly and annual financial summaries.
- Dashboard for revenue and expense analysis.

**Applies to:** Web Admin, Desktop Admin

---

### 3.6 Marketing Manager
**Goals:**
- Increase visibility and sales through promotions.

**Features:**
- Create and manage discount codes.
- Launch seasonal campaigns and promo packages.
- Monitor conversion and engagement metrics.
- Sync tour data to social media and travel platforms.
- Manage content for the landing page and blogs.

**Applies to:** Web Admin

---

### 3.7 Customer Support
**Goals:**
- Provide responsive and quality service to customers.

**Features:**
- Access booking and customer details.
- Respond to inquiries via integrated chat or email.
- Log complaints and resolutions.
- Manage customer reviews and feedback.
- Create issue reports for admin attention.

**Applies to:** Web Admin, Desktop Admin

---

### 3.8 Executive / Owner
**Goals:**
- View overall business performance and make data-driven decisions.

**Features:**
- Access business intelligence dashboard.
- View top tours, customer trends, and financial metrics.
- Export reports to CSV/PDF.
- Schedule automated report deliveries via email.
- View system health summaries (uptime, sales, user growth).

**Applies to:** Web Admin

---

## 4. Cross-Functional Features

| Feature | Description | Applies To |
|----------|--------------|------------|
| Notifications | Real-time alerts for bookings, payments, or errors | Both |
| Offline Mode | Continue operations when disconnected | Desktop |
| Data Sync | Auto-sync between local and remote databases | Desktop |
| Reports Export | Export in CSV, PDF formats | Both |
| Multi-language Support | Support for English, Cebuano, and Japanese | Both |
| Role-based Access Control | Ensure security via scoped permissions | Both |

---

## 5. Technology Stack (Planned)

| Layer | Technology |
|-------|-------------|
| Frontend (Web) | Leptos (Rust), TailwindCSS |
| Frontend (Desktop) | Iced (Rust GUI) / Tauri |
| Backend API | Axum (Rust), gRPC/Tonic for services |
| Database | PostgreSQL 16 with replication and JSONB extensions |
| Caching / Queues | PostgreSQL LISTEN/NOTIFY, background workers |
| Authentication | Argon2 + Blake3 + Sha2 + session store |
| Reports | PDF generation (ReportLab / Rust PDF crate) |
| Packaging | `.deb`, `.rpm`, `.msi` (via cargo-dist / WiX) |

---

## 6. Summary
This Admin System unifies all Cebu Tours & Adventures operations into one cohesive platform.  
It bridges **online management (Web Admin)** and **on-site control (Desktop Admin)** through shared APIs, consistent UI logic, and robust PostgreSQL data replication.

---

**Prepared by:**
**Software Architecture Team**
Cebu Tours & Adventures Digital Division_

