
INSERT INTO tbl_role (name, description, cid, ctime)
VALUES ('Admin', 'Superuser with full system access', 0, now())
ON CONFLICT (name) DO NOTHING;

INSERT INTO tbl_user_account (username, cid, ctime, mid, mtime)
VALUES ('admin',0,now(),0,now());

INSERT INTO tbl_user_role (user_id, role_id, cid, ctime, mid, mtime)
SELECT u.id, r.id, 0, now(), 0, now()
FROM tbl_user_account u, tbl_role r
WHERE u.username='admin' AND r.name='Admin'
ON CONFLICT DO NOTHING;
