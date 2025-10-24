-- Add migration script here

-- Departments
INSERT INTO tbl_department (department_id, name, description) VALUES
  (gen_random_uuid(), 'Engineering','Engineering & Development'),
  (gen_random_uuid(), 'HR','Human Resources');

-- Salary grades
INSERT INTO tbl_salary_grade (salary_grade_id, level, base_salary) VALUES
  (gen_random_uuid(), 'SG1', 20000.00),
  (gen_random_uuid(), 'SG2', 35000.00);

-- Job positions (you may want to query back the IDs if not using fixed UUIDs)
INSERT INTO tbl_job_position (job_position_id, title, department_id, salary_grade_id, description)
SELECT gen_random_uuid(), 'Software Engineer', d.department_id, s.salary_grade_id, 'Backend/Fullstack dev'
FROM tbl_department d, salary_grade s
WHERE d.name = 'Engineering' AND s.level = 'SG2'
LIMIT 1;

-- Leave types
INSERT INTO tbl_leave_type (leave_type_id, name, max_days, is_paid) VALUES
  (gen_random_uuid(), 'Annual Leave', 15, TRUE),
  (gen_random_uuid(), 'Sick Leave', 10, TRUE),
  (gen_random_uuid(), 'Unpaid Leave', 0, FALSE);

-- Roles & permissions
INSERT INTO tbl_role (role_id, name, description) VALUES
  (gen_random_uuid(), 'admin', 'System administrator'),
  (gen_random_uuid(), 'hr', 'HR manager'),
  (gen_random_uuid(), 'employee', 'Regular employee');

INSERT INTO tbl_permission (permission_id, module, action, level) VALUES
  (gen_random_uuid(), 'users', 'create', 'high'),
  (gen_random_uuid(), 'users', 'update', 'high'),
  (gen_random_uuid(), 'leave', 'apply', 'low'),
  (gen_random_uuid(), 'leave', 'approve', 'medium');

-- Create a demo address and employee and an admin user
WITH addr AS (
  INSERT INTO tbl_address (address_id, building_number, street_name, barangay, city, municipality, province)
  VALUES (gen_random_uuid(), '100', 'Main St', 'Centro', 'Metrocity', 'Metro', 'Province')
  RETURNING address_id
),
emp AS (
  INSERT INTO tbl_employee (employee_id, firstname, middlename, lastname, hire_date, status, address_id)
  VALUES (gen_random_uuid(), 'Admin', '', 'User', '2020-01-01', 'Active', (SELECT address_id FROM addr))
  RETURNING employee_id
)
INSERT INTO tbl_user_account (user_id, employee_id, username, password_hash, pass_salt, token_salt)
VALUES (gen_random_uuid(), (SELECT employee_id FROM emp), 'admin', '<PASSWORD_HASH_HERE>', gen_random_uuid()::text, gen_random_uuid()::text);

-- Assign admin role to admin user
INSERT INTO tbl_user_role (user_role_id, user_id, role_id)
SELECT gen_random_uuid(), u.user_id, r.role_id
FROM tbl_user_account u, tbl_role r
WHERE u.username = 'admin' AND r.name = 'admin';

