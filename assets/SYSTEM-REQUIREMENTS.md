# Cebu Tours & Adventures System Requirements

## 1. System Overview

The **Cebu Tours & Adventures** system consists of two major components:

1. **Client Website** – For tourists and customers to browse, book, and explore tours, promos, and locations.  
2. **Admin System** – For staff and management to oversee tours, bookings, payments, analytics, fleets, and configurations.

Both systems share a unified **PostgreSQL database** and **Axum + SQLx backend**, but differ in access control, purpose, and interface.

---

## 2. CLIENT WEBSITE

### 2.1 Functional Requirements (FR-C)

| ID | Description | Applies To |
|----|--------------|------------|
| FR-C1 | Visitors can browse available tour packages with images, prices, and inclusions. | Web |
| FR-C2 | Users can filter tours by category (island hopping, city tours, adventures). | Web |
| FR-C3 | Users can view detailed package pages with itinerary, gallery, and booking form. | Web |
| FR-C4 | Visitors can see active promos, discounts, and seasonal packages. | Web |
| FR-C5 | Users can check tour availability by date. | Web |
| FR-C6 | Users can book tours by filling up a booking form (name, email, date, headcount). | Web |
| FR-C7 | Bookings generate confirmation emails with reference number. | Web |
| FR-C8 | Users can pay online via PayMongo, Stripe, or GCash integration. | Web |
| FR-C9 | Visitors can view a map of pickup points and destinations. | Web |
| FR-C10 | Site must include About Us, Contact Us, FAQ, and Terms & Conditions pages. | Web |
| FR-C11 | Contact form submissions must send an email to support inbox. | Web |
| FR-C12 | Site must include multilingual support (English, Cebuano, Japanese, Korean). | Web |
| FR-C13 | Users can view testimonials and reviews from previous customers. | Web |
| FR-C14 | SEO metadata, OpenGraph tags, and structured data must be generated for all public pages. | Web |
| FR-C15 | Site must be mobile-friendly and pass Lighthouse mobile performance > 85. | Web |

---

### 2.2 Non-Functional Requirements (NFR-C)

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-C1 | Page load time must not exceed 2.5s on 3G network. | Web |
| NFR-C2 | Web must support CDN delivery for images (e.g., Cloudflare). | Web |
| NFR-C3 | All public forms must have CAPTCHA (hCaptcha/Cloudflare Turnstile). | Web |
| NFR-C4 | Public routes must be cacheable for 24 hours. | Web |
| NFR-C5 | Booking and payment APIs must use HTTPS only. | Web |
| NFR-C6 | Frontend must be built with Leptos (SSR) for SEO optimization. | Web |
| NFR-C7 | The design must comply with WCAG 2.1 accessibility standards. | Web |
| NFR-C8 | Translations must be stored in versioned JSON/i18n files. | Web |
| NFR-C9 | Deployment must use Docker and Nginx with HTTP/2. | Web |

---

## 3. ADMIN SYSTEM
(*Web Admin + Desktop Admin share core logic and API*)

### 3.1 Functional Requirements (FR-A)

#### Authentication & Authorization

| ID | Description | Applies To |
|----|--------------|------------|
| FR-1 | The system must allow admins to log in using email and password. | Desktop |
| FR-2 | The system must validate credentials against the PostgreSQL database. | Desktop |
| FR-3 | The system must implement role-based access control (RBAC) with roles: `super_admin`, `manager`, `staff`. | Desktop |
| FR-4 | Sessions on the web must use JWT-based cookies with CSRF protection. | Web |
| FR-5 | The desktop app must use secure TOTP or stored access tokens. | Desktop |
| FR-6 | The system must automatically log out users after 30 minutes of inactivity. | Desktop |

#### Dashboard & Analytics

| ID | Description | Applies To |
|----|--------------|------------|
| FR-7 | The dashboard must display real-time KPIs: total bookings, revenue, active tours. | Desktop |
| FR-8 | The system must display charts of booking trends and customer origins. | Desktop |
| FR-9 | The desktop admin must provide Google Analytics integration. | Desktop |
| FR-10 | The desktop app must cache analytics data locally for offline viewing. | Desktop |

#### Tour Management

| ID | Description | Applies To |
|----|--------------|------------|
| FR-11 | Admins can create, edit, and delete tour packages. | Desktop |
| FR-12 | Admins can upload images, set price, availability, and SEO metadata. | Web |
| FR-13 | The system must prevent overlapping schedule conflicts for the same tour ID. | Desktop |
| FR-14 | The system must validate input data (dates, price range, description length). | Desktop |

#### Booking Management

| ID | Description | Applies To |
|----|--------------|------------|
| FR-15 | Admins can view, approve, cancel, or refund bookings. | Desktop |
| FR-16 | The system must support searching bookings by date, tour name, or customer. | Both |
| FR-17 | Booking changes must automatically trigger notification emails to customers. | Desktop |
| FR-18 | Booking updates must sync with the central database via gRPC when online. | Desktop |

#### Customer Management

| ID | Description | Applies To |
|----|--------------|------------|
| FR-19 | Admins can view and edit customer profiles. | Desktop |
| FR-20 | The system must track booking history and loyalty points. | Desktop |
| FR-21 | The system must anonymize deleted customer data for GDPR compliance. | Both |

#### Payments & Invoices

| ID | Description | Applies To |
|----|--------------|------------|
| FR-22 | The system must track payment transactions linked to bookings. | Both |
| FR-23 | Admins can mark payments as paid, pending, or refunded. | Both |
| FR-24 | Integration with Stripe or PayMongo for live payments. | Web |
| FR-25 | Offline desktop mode must store payment logs until reconnected. | Desktop |

#### Fleet & Staff Management

| ID | Description | Applies To |
|----|--------------|------------|
| FR-26 | Admins can schedule vehicles and assign drivers to tours. | Both |
| FR-27 | Staff accounts can be created, updated, or deactivated. | Both |
| FR-28 | The system must maintain audit logs of all staff actions. | Both |
| FR-29 | The desktop app must provide a calendar-based fleet view. | Desktop |

#### Reports & Data Export

| ID | Description | Applies To |
|----|--------------|------------|
| FR-30 | Admins can generate and export reports (PDF, CSV, XLSX). | Both |
| FR-31 | Reports include revenue summaries, booking volumes, and top tours. | Both |
| FR-32 | Scheduled reports can be emailed automatically. | Web |

#### Settings & Notifications

| ID | Description | Applies To |
|----|--------------|------------|
| FR-33 | Admins can configure payment gateways and email templates. | Both |
| FR-34 | Notification settings (email, SMS, app) must be adjustable. | Both |
| FR-35 | The system must allow brand customization (logo, color scheme). | Both |
| FR-36 | The desktop app must persist settings locally via `confy`. | Desktop |

---

### 3.2 Non-Functional Requirements (NFR-A)

#### Performance

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-1 | Web API must respond within 500ms for 95% of requests. | Web |
| NFR-2 | Desktop app must render all modules in under 150ms after switching views. | Desktop |
| NFR-3 | Database queries must use indexes to support 10,000+ bookings. | Both |

#### Scalability

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-4 | The backend must support horizontal scaling via load-balanced Axum instances. | Web |
| NFR-5 | The desktop app must handle up to 50 concurrent threads safely (Tokio). | Desktop |

#### Reliability & Availability

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-6 | The web backend uptime must be 99.9% monthly. | Web |
| NFR-7 | The desktop app must auto-reconnect to backend when online. | Desktop |
| NFR-8 | All transactions must be ACID-compliant via PostgreSQL. | Both |

#### Security

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-9 | All communication must be encrypted using TLS 1.3. | Both |
| NFR-10 | Passwords must be hashed with Argon2 or bcrypt. | Both |
| NFR-11 | Input validation and sanitization must prevent SQL injection and XSS. | Both |
| NFR-12 | Admin actions must be logged and auditable. | Both |

#### Usability

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-13 | The UI must be intuitive and responsive (mobile, tablet, desktop). | Web |
| NFR-14 | Desktop app must support keyboard shortcuts and theme switching. | Desktop |
| NFR-15 | Both apps must provide user feedback on every action (loading, success, error). | Both |

#### Maintainability

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-16 | Code must follow Rust Clippy and cargo fmt standards. | Both |
| NFR-17 | Shared models must reside in a common crate (`cta-shared`). | Both |
| NFR-18 | Automated CI/CD testing for lint, build, and integration must be implemented. | Both |

#### Portability

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-19 | Web app must be deployable via Docker and Nginx. | Web |
| NFR-20 | Desktop app must build for Windows, Linux, and macOS (.deb/.rpm/.msi). | Desktop |

#### Data Integrity & Backup

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-21 | Database must have daily incremental backups. | Both |
| NFR-22 | Desktop app must locally cache unsynced changes safely on crash. | Desktop |

#### Additional Admin NFRs

| ID | Description | Applies To |
|----|--------------|------------|
| NFR-A23 | Desktop app must sync new data using background gRPC workers. | Desktop |
| NFR-A24 | Admin backend must support audit data retention of 12 months minimum. | Both |
| NFR-A25 | Web admin interface must support dark/light theme toggle. | Web |

---

## 4. Integration Overview

| Component | Description |
|------------|-------------|
| **Backend (Axum)** | REST + gRPC API for both client and admin apps. |
| **Database (PostgreSQL)** | Shared relational data store for tours, bookings, payments, users. |
| **CDN (Cloudflare)** | Static assets (images, videos) for client site. |
| **Email Service** | SMTP or SendGrid for notifications and contact forms. |
| **Payment Gateway** | PayMongo/Stripe for live payments. |
| **Desktop Admin** | Built using Iced (Rust GUI). Uses gRPC for data sync and local caching with `sled` or `sqlite`. |
| **Web Admin** | Built using Leptos SSR + TailwindCSS. Accessible via browser. |

---

## 5. Security Summary

| Concern | Enforcement |
|----------|-------------|
| Authentication | Argon2, Blake3 and Sha2 (Web), Token/TOTP (Desktop) |
| Authorization | Role-based (Super Admin, Manager, Staff) |
| Data Encryption | TLS 1.3, AES-256 at rest |
| Password Policy | Argon2 hashed, min 8 chars, 2FA optional |
| Audit Logs | Stored per admin action (create/update/delete) |
| Input Sanitization | SQLx bind params + input sanitization middleware |

---

## 6. Deployment Targets

| Platform | Stack | Output |
|-----------|--------|--------|
| Client Website | Leptos SSR + Tailwind + Axum | Deployed via Docker + Nginx |
| Web Admin | Leptos SSR | Docker + Reverse Proxy |
| Desktop Admin | Iced (Rust GUI) | MSI (Windows), DEB/RPM (Linux), DMG (macOS) |

---

## Summary

| System | Primary Users | Access | Purpose |
|---------|----------------|--------|----------|
| **Client Website** | Tourists / Visitors | Public | Browse & book tours |
| **Web Admin** | Managers / Staff | Private (Browser) | Manage tours, bookings, payments |
| **Desktop Admin** | Internal Staff | Private (Desktop App) | Offline-capable administration & analytics |

---

