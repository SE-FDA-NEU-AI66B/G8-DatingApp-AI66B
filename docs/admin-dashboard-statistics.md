# Admin dashboard statistics

The **Registered users** metric counts only accounts whose NEU email has been
verified (`users.email_verified = TRUE`) by a valid Microsoft identity token
with an `@neu.edu.vn` email claim.

The other US14 metrics are:

- **Active study dates**: requests with `status = 'active'` whose time window
  overlaps now and the next 24 hours.
- **Pending reports**: reports with `status = 'pending'`.
