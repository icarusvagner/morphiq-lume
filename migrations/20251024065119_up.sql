-- Add migration script here
-- Enable pgcrypto for gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TYPE "employment_status" AS ENUM (
	'Full-time',
	'Part-time',
	'Contractor',
	'Temporary',
	'Intern'
);

-- Master tables
CREATE TABLE tbl_department (
    department_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE tbl_salary_grade (
    salary_grade_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    level TEXT NOT NULL UNIQUE,
    base_salary NUMERIC(12,2) NOT NULL CHECK (base_salary >= 0)
);

CREATE TABLE tbl_job_position (
    job_position_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL UNIQUE,
    department_id UUID NOT NULL REFERENCES tbl_department(department_id) ON UPDATE CASCADE,
    salary_grade_id UUID NOT NULL REFERENCES tbl_salary_grade(salary_grade_id) ON UPDATE CASCADE,
    description TEXT
);

-- Address (1:1 with employee)
CREATE TABLE tbl_address (
    address_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    building_number TEXT,
    street_name TEXT,
    barangay TEXT,
    city TEXT,
    municipality TEXT,
    province TEXT,
    time_created TIMESTAMP DEFAULT NOW(),
    time_updated TIMESTAMP DEFAULT NOW()
);

-- Employee
CREATE TABLE tbl_employee (
    employee_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    firstname TEXT NOT NULL,
    middlename TEXT,
    lastname TEXT NOT NULL,
    hire_date DATE NOT NULL,
    status TEXT NOT NULL, -- e.g. 'Active'
    supervisor_id UUID REFERENCES tbl_employee(employee_id) ON DELETE SET NULL ON UPDATE CASCADE,
    address_id UUID UNIQUE REFERENCES tbl_address(address_id) ON UPDATE CASCADE,
    department_id UUID REFERENCES tbl_department(department_id) ON UPDATE CASCADE,
    job_position_id UUID REFERENCES tbl_job_position(job_position_id) ON UPDATE CASCADE
);

-- Time & attendance
CREATE TABLE tbl_time_log (
    time_log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES tbl_employee(employee_id) ON DELETE CASCADE,
    check_in TIMESTAMP NOT NULL,
    check_out TIMESTAMP,
    notes TEXT,
    CHECK (check_out IS NULL OR check_out >= check_in)
);

CREATE TABLE tbl_attendance_record (
    attendance_record_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES tbl_employee(employee_id) ON DELETE CASCADE,
    date DATE NOT NULL,
    status TEXT NOT NULL
);

-- Leave management
CREATE TABLE tbl_leave_type (
    leave_type_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    max_days INT CHECK (max_days >= 0),
    is_paid BOOLEAN DEFAULT TRUE
);

CREATE TABLE tbl_leave_balance (
    leave_balance_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES tbl_employee(employee_id) ON DELETE CASCADE,
    leave_type_id UUID NOT NULL REFERENCES tbl_leave_type(leave_type_id) ON DELETE CASCADE,
    balance_days INT NOT NULL CHECK (balance_days >= 0),
    UNIQUE(employee_id, leave_type_id)
);

CREATE TABLE tbl_leave_request (
    leave_request_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES tbl_employee(employee_id),
    leave_type_id UUID NOT NULL REFERENCES tbl_leave_type(leave_type_id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    reason TEXT,
    status TEXT NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    CHECK (end_date >= start_date)
);

-- Payroll domain
CREATE TABLE tbl_earning (
    earning_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    amount NUMERIC(12,2) NOT NULL CHECK (amount >= 0),
    taxable BOOLEAN DEFAULT TRUE
);

CREATE TABLE tbl_deduction (
    deduction_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    amount NUMERIC(12,2) CHECK (amount >= 0),
    percentage NUMERIC(5,2) CHECK (percentage >= 0)
);

CREATE TABLE tbl_kpi (
    kpi_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    weight INT CHECK (weight >= 0)
);

CREATE TABLE tbl_payroll (
    payroll_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES tbl_employee(employee_id) ON DELETE CASCADE,
    start_period DATE NOT NULL,
    end_period DATE NOT NULL,
    gross_pay NUMERIC(12,2) NOT NULL CHECK (gross_pay >= 0),
    total_deductions NUMERIC(12,2) CHECK (total_deductions >= 0) DEFAULT 0,
    net_pay NUMERIC(12,2) NOT NULL CHECK (net_pay >= 0),
    date_processed TIMESTAMP DEFAULT NOW(),
    CHECK (end_period >= start_period)
);

CREATE TABLE tbl_performance_review (
    performance_review_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES tbl_employee(employee_id),
    date_evaluated DATE NOT NULL,
    score INT CHECK (score >= 0 AND score <= 100),
    remarks TEXT,
    status TEXT
);

-- Auth & RBAC
CREATE TABLE tbl_user_account (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID REFERENCES tbl_employee(employee_id),
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    pass_salt TEXT,
    token_salt TEXT,
    status TEXT DEFAULT 'Active',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE tbl_role (
    role_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE tbl_permission (
    permission_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    level TEXT
);

CREATE TABLE tbl_user_role (
    user_role_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES tbl_user_account(user_id) ON DELETE CASCADE,
    role_id UUID REFERENCES tbl_role(role_id) ON DELETE CASCADE,
    UNIQUE(user_id, role_id)
);

CREATE TABLE tbl_role_permission (
    role_permission_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID REFERENCES tbl_role(role_id) ON DELETE CASCADE,
    permission_id UUID REFERENCES tbl_permission(permission_id) ON DELETE CASCADE,
    UNIQUE(role_id, permission_id)
);

-- Audit log (actor references user_account)
CREATE TABLE tbl_audit_log (
    audit_log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES tbl_user_account(user_id),
    action TEXT NOT NULL,
    target_table TEXT NOT NULL,
    target_id UUID,
    changes JSONB,
    time_created TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_employee_department ON tbl_employee(department_id);
CREATE INDEX idx_employee_supervisor ON tbl_employee(supervisor_id);
CREATE INDEX idx_payroll_employee ON tbl_payroll(employee_id);
CREATE INDEX idx_attendance_employee ON tbl_attendance_record(employee_id);
CREATE INDEX idx_leave_balance_employee ON tbl_leave_balance(employee_id);

