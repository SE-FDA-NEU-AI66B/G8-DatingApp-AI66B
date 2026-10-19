# Admin dashboard statistics

The **Registered users** metric counts only accounts whose NEU email has been
verified (`app_user.email_verified = TRUE`). An account created during the OTP
registration flow is not included until the OTP is accepted. This keeps the
dashboard aligned with the product definition of a registered NEU member rather
than counting incomplete sign-ups.

The other US14 metrics are:

- **Active study dates**: requests with `status = 'active'` whose time window
  overlaps now and the next 24 hours.
- **Pending reports**: reports with `status = 'pending'`.
