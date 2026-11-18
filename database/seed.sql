-- Seed statements for database/init.py. Keep each statement under its named section.

-- statement: sample_user
INSERT INTO users (email, username, password_hash, email_verified)
VALUES (%s, %s, %s, TRUE)
ON CONFLICT (email) DO UPDATE SET
    username = EXCLUDED.username,
    password_hash = EXCLUDED.password_hash,
    email_verified = TRUE
RETURNING id;

-- statement: admin_user
INSERT INTO users (email, username, password_hash, email_verified, role)
VALUES (%s, %s, %s, TRUE, 'user')
ON CONFLICT (email) DO UPDATE SET
    username = EXCLUDED.username,
    password_hash = EXCLUDED.password_hash,
    email_verified = TRUE,
    role = 'user'
RETURNING id;

-- statement: study_dates
INSERT INTO study_dates
    (creator_id, starts_at, ends_at, duration_min, status)
SELECT user_id, start_at, start_at + interval '60 minutes', 60, 'active'
FROM (VALUES
    (%s::uuid, date_trunc('day', now()) + interval '1 day' + interval '9 hours'),
    (%s::uuid, date_trunc('day', now()) + interval '1 day' + interval '14 hours'),
    (%s::uuid, date_trunc('day', now()) + interval '2 days' + interval '10 hours')
) AS samples(user_id, start_at)
WHERE NOT EXISTS (
    SELECT 1 FROM study_dates existing
    WHERE existing.creator_id = samples.user_id
      AND existing.starts_at = samples.start_at
);

-- statement: pending_report
INSERT INTO user_reports (reporter_id, reported_user_id, status)
SELECT %s, %s, 'pending'
WHERE NOT EXISTS (
    SELECT 1 FROM user_reports
    WHERE reporter_id = %s AND reported_user_id = %s AND status = 'pending'
);

-- statement: accepted_match
INSERT INTO match_requests (sender_id, receiver_id, status, responded_at)
VALUES (%s, %s, 'accepted', now())
ON CONFLICT (sender_id, receiver_id) DO UPDATE
SET status = 'accepted', responded_at = COALESCE(match_requests.responded_at, now())
RETURNING id;

-- statement: pending_match
INSERT INTO match_requests (sender_id, receiver_id, status)
VALUES (%s, %s, 'pending')
ON CONFLICT (sender_id, receiver_id) DO NOTHING;

-- statement: conversation
INSERT INTO conversations (user_low, user_high, match_request_id)
VALUES (%s, %s, %s)
ON CONFLICT (user_low, user_high) DO NOTHING;

-- statement: sample_posts
INSERT INTO threads (author_id, title, body)
SELECT samples.author_id, samples.title, samples.body
FROM (VALUES
    (%s::uuid, 'Best quiet places to study?', 'I am looking for quiet spots on campus with reliable Wi-Fi. Where do you like to study?'),
    (%s::uuid, 'Study group for algorithms', 'Anyone interested in reviewing graph algorithms together this week?'),
    (%s::uuid, 'Sharing exam prep tips', 'What has helped you prepare for midterms? I can share my notes for databases.')
) AS samples(author_id, title, body)
WHERE NOT EXISTS (
    SELECT 1 FROM threads existing
    WHERE existing.author_id = samples.author_id
      AND existing.title = samples.title
);
