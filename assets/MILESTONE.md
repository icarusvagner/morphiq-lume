# Cebu Tours & Adventures Admin System — Agile Sprint Milestones

## 1. Agile Development Overview
The project will follow an **Agile Scrum methodology**, with iterative development in 2-week sprints.  
Each sprint delivers incremental, testable features for both the **Web Admin (Leptos)** and **Desktop Admin (Iced)** systems, built upon a shared **Axum + PostgreSQL** backend.

---

## 2. Sprint Roadmap Overview

| Sprint | Duration | Theme / Milestone | Key Deliverables |
|---------|-----------|------------------|------------------|
| Sprint 1 | Week 1–2 | Core Infrastructure Setup | Project scaffolding, CI/CD setup, PostgreSQL schema design |
| Sprint 2 | Week 3–4 | Authentication & User Roles | Login, registration, role-based access, audit logs |
| Sprint 3 | Week 5–6 | Tours & Fleet Management | CRUD for tours, fleet assignment, image upload, offline caching |
| Sprint 4 | Week 7–8 | Booking & Payment System | Booking flow, payments, invoicing, notifications |
| Sprint 5 | Week 9–10 | Analytics, Reports & Final QA | Dashboard analytics, export tools, QA & deployment readiness |

---

## 3. Sprint Breakdown

### Sprint 1 — Core Infrastructure Setup
**Goal:** Establish the foundation for backend, frontend, and desktop applications.

**Deliverables:**
- Initialize Rust workspace with shared crates (`api`, `frontend-web`, `frontend-desktop`, `common`).
- Set up Axum + PostgreSQL backend with `sqlx` and migration scaffolding.
- Configure Docker + Docker Compose for local development.
- CI/CD pipelines for GitHub Actions (build + test + deploy).
- Define database schema for users, roles, tours, and bookings.
- Implement replication setup for PostgreSQL (primary/replica).
- Configure TailwindCSS for Web Admin UI.

**Responsible:** Backend Dev, DevOps Engineer, Database Architect

---

### Sprint 2 — Authentication & User Roles
**Goal:** Enable secure access and user identity management.

**Deliverables:**
- Implement JWT-based authentication (login/logout).
- Admin login flow for both Web and Desktop.
- Role-based access control (System Admin, Booking Officer, etc.).
- Password reset and email verification flow.
- Audit trail for login and critical actions.
- Session management table for multi-device tracking.

**Responsible:** Backend Dev, Web Dev, Desktop Dev

---

### Sprint 3 — Tours & Fleet Management
**Goal:** Build modules for managing tours, vehicles, and drivers.

**Deliverables:**
- CRUD for Tours (name, itinerary, price, images, availability).
- Fleet management: add vehicles, assign drivers, track maintenance.
- Sync tour schedules with fleet availability.
- Local offline caching for the Desktop app using SQLite + background sync.
- Admin dashboard to view active tours and fleet status.

**Responsible:** Backend Dev, Web Dev, Desktop Dev, UI/UX Designer

---

### Sprint 4 — Booking & Payment System
**Goal:** Handle booking flows, payments, and notifications.

**Deliverables:**
- Booking creation (manual and online).
- Payment handling (manual entry + API integration with payment gateway).
- Refund and cancellation management.
- Invoice and receipt generation (PDF).
- Email/SMS notifications for bookings and payments.
- Notification center in Web and Desktop apps.

**Responsible:** Backend Dev, Web Dev, QA Engineer, Finance Officer (Stakeholder input)

---

### Sprint 5 — Analytics, Reports & Final QA
**Goal:** Deliver insights and prepare for production release.

**Deliverables:**
- Analytics dashboard (revenue, most booked tours, customer trends).
- Export reports (PDF, CSV) for finance and operations.
- Real-time sync monitoring between Desktop and Cloud.
- Multi-language support (English, Cebuano, Japanese).
- Full QA testing: functional, integration, and performance tests.
- Packaging and release builds: `.deb`, `.rpm`, `.msi`, and web deployment.
- Handover documentation and deployment checklist.

**Responsible:** Full Team, QA Lead, Project Manager

---

## 4. Optional Extended Milestones (Post-Launch)

| Phase | Timeline | Description |
|--------|-----------|-------------|
| Phase 2 | Week 11–14 | Add marketing campaign manager, blog CMS, and SEO tools |
| Phase 3 | Week 15–18 | Integrate GPS tracking for fleets, driver mobile companion app |
| Phase 4 | Week 19–22 | Implement customer mobile portal (for booking and feedback) |

---

## 5. Definition of Done (DoD)
Each sprint’s deliverables are considered **Done** when:
- Code passes all automated tests and code review.
- UI is responsive and accessible.
- Database migrations applied successfully in staging.
- Features demonstrated in sprint review.
- Documentation updated (README, API docs, release notes).

---

**Prepared by:**  
**Cebu Tours & Adventures Product & Engineering Team**  
_Aligned with Agile Sprint Cycle v1.0_

