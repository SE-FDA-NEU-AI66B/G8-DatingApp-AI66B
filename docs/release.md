# Release guide

## Hosting

The application is self-hosted. Cloudflare Tunnel terminates public HTTPS and
forwards to the Actix/Leptos process at `http://localhost:3100`, as configured
in `cert/.cloudflared/config.yml`. PostgreSQL runs on the same host through
`database/compose.yaml`, with port 5432 bound to loopback and a Docker volume.
There is no managed app/database platform, checked-in service unit, or automated
deployment. Keep tunnel credentials and `.env` on the host; set the real public
Microsoft callback URL in the host environment.

## Deploy and rollback

1. Review and merge the intended commit to `main`. Check the Actions workflow,
   but note `eltonjohn.yml` currently only checks out code: its Cargo build and
   test commands are commented out.
2. On the host, update to the reviewed commit, build with
   `cargo leptos build --release`, and restart the application using the host's
   process manager. Preserve the production `.env` and tunnel credentials.
3. Verify the public URL, `/health`, sign-in, and the admin console.
4. If the release fails, restore and rebuild the previous known-good commit,
   or merge a revert and deploy that commit. Verify the same routes afterward.

This procedure describes the current manual setup; it does not claim a
zero-downtime deployment, production backup, or automated rollback.
