# Release guide

## Hosting

The application is hosted as the Render web service described in
[`render.yaml`](../render.yaml). Keep production environment values in Render;
the repository does not contain them.

## Deploy and rollback

1. Configure the GitHub repository secret `RENDER_DEPLOY_HOOK_URL` with the
   Render service's deploy hook URL. Keep it secret; the URL authorizes a deploy.
2. Merge the intended commit to `main`. The Render deploy workflow calls the
   hook only after the `CI` workflow completes successfully for `main`.
3. Verify the deployment in Render and check the public URL, `/health`, sign-in,
   and the admin console.
4. If the release fails, use Render's dashboard to roll back to the previous
   successful deployment, or revert the change and wait for CI to pass.

This procedure does not claim a zero-downtime deployment, production backup,
or automatic rollback.
