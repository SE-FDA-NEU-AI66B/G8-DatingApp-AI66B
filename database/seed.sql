INSERT INTO app_user (email, email_verified, role)
VALUES
    ('admin.seed@neu.edu.vn', TRUE, 'admin'),
    ('user01.seed@neu.edu.vn', TRUE, 'user'),
    ('user02.seed@neu.edu.vn', TRUE, 'user'),
    ('user03.seed@neu.edu.vn', TRUE, 'user'),
    ('user04.seed@neu.edu.vn', TRUE, 'user'),
    ('user05.seed@neu.edu.vn', TRUE, 'user'),
    ('user06.seed@neu.edu.vn', TRUE, 'user'),
    ('user07.seed@neu.edu.vn', TRUE, 'user'),
    ('user08.seed@neu.edu.vn', TRUE, 'user'),
    ('user09.seed@neu.edu.vn', TRUE, 'user')
ON CONFLICT (email) DO NOTHING;

INSERT INTO study_date_request (creator_id, starts_at, ends_at, status)
SELECT id, now() + interval '2 hours', now() + interval '3 hours', 'active'
FROM app_user
WHERE email = 'user01.seed@neu.edu.vn'
  AND NOT EXISTS (
      SELECT 1 FROM study_date_request
      WHERE creator_id = app_user.id AND status = 'active'
  );

INSERT INTO study_date_request (creator_id, starts_at, ends_at, status)
SELECT id, now() + interval '5 hours', now() + interval '6 hours', 'active'
FROM app_user
WHERE email = 'user02.seed@neu.edu.vn'
  AND NOT EXISTS (
      SELECT 1 FROM study_date_request
      WHERE creator_id = app_user.id AND status = 'active'
  );

INSERT INTO report (reporter_id, reported_user_id, status)
SELECT reporter.id, reported.id, 'pending'
FROM app_user reporter
JOIN app_user reported ON reported.email = 'user02.seed@neu.edu.vn'
WHERE reporter.email = 'user01.seed@neu.edu.vn'
  AND NOT EXISTS (
      SELECT 1 FROM report
      WHERE reporter_id = reporter.id AND reported_user_id = reported.id
  );
