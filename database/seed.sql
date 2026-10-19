-- Development fixtures for every application table. Safe to run repeatedly.
BEGIN;

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

-- This fixture token's SHA-256 is stored; it has no effect unless used as a cookie.
INSERT INTO auth_session (token_hash, user_id, expires_at)
SELECT 'bb6ac6f48ed9708870b289b67eb2eb174b4f1bfc600abe8d0d326b7a854a616f',
       id, now() + interval '365 days'
FROM app_user
WHERE email = 'user01.seed@neu.edu.vn'
ON CONFLICT (token_hash) DO NOTHING;

INSERT INTO match_request (sender_id, receiver_id, status, responded_at)
SELECT sender.id, receiver.id, 'accepted', now()
FROM app_user sender
JOIN app_user receiver ON receiver.email = 'user04.seed@neu.edu.vn'
WHERE sender.email = 'user03.seed@neu.edu.vn'
ON CONFLICT (sender_id, receiver_id) DO UPDATE
SET status = 'accepted', responded_at = COALESCE(match_request.responded_at, EXCLUDED.responded_at);

INSERT INTO match_request (sender_id, receiver_id, status)
SELECT sender.id, receiver.id, 'pending'
FROM app_user sender
JOIN app_user receiver ON receiver.email = 'user06.seed@neu.edu.vn'
WHERE sender.email = 'user05.seed@neu.edu.vn'
ON CONFLICT (sender_id, receiver_id) DO NOTHING;

INSERT INTO chat_room (user_low, user_high, match_request_id)
SELECT LEAST(sender.id, receiver.id), GREATEST(sender.id, receiver.id), request.id
FROM app_user sender
JOIN app_user receiver ON receiver.email = 'user04.seed@neu.edu.vn'
JOIN match_request request
  ON request.sender_id = sender.id
 AND request.receiver_id = receiver.id
 AND request.status = 'accepted'
WHERE sender.email = 'user03.seed@neu.edu.vn'
ON CONFLICT (user_low, user_high) DO NOTHING;

COMMIT;
