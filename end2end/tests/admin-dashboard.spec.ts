import { expect, test } from "@playwright/test";

const statistics = {
  total_registered_users: 42,
  active_study_date_requests: 7,
  pending_reports: 3,
};

test.describe("admin dashboard statistics", () => {
  test("displays all statistics after the dashboard loads", async ({ page }) => {
    await page.route("**/api/admin/statistics", async (route) => {
      await route.fulfill({ json: statistics });
    });

    await page.goto("/admin_console");

    await expect(page.getByRole("heading", { name: "Admin dashboard" })).toBeVisible();
    await expect(page.getByText("Registered users").locator("..")).toContainText("42");
    await expect(page.getByText("Active study dates").locator("..")).toContainText("7");
    await expect(page.getByText("Pending reports").locator("..")).toContainText("3");
  });

  test("refreshes the displayed statistics", async ({ page }) => {
    let requestCount = 0;
    await page.route("**/api/admin/statistics", async (route) => {
      requestCount += 1;
      await route.fulfill({
        json:
          requestCount === 1
            ? statistics
            : {
                total_registered_users: 43,
                active_study_date_requests: 8,
                pending_reports: 2,
              },
      });
    });

    await page.goto("/admin_console");
    await expect(page.getByText("Registered users").locator("..")).toContainText("42");

    await page.getByRole("button", { name: "Refresh" }).click();

    await expect(page.getByText("Registered users").locator("..")).toContainText("43");
    await expect(page.getByText("Active study dates").locator("..")).toContainText("8");
    await expect(page.getByText("Pending reports").locator("..")).toContainText("2");
  });

  test("shows an error instead of zero-valued statistics when loading fails", async ({
    page,
  }) => {
    await page.route("**/api/admin/statistics", async (route) => {
      await route.fulfill({
        status: 500,
        contentType: "application/json",
        body: JSON.stringify({
          message: "Dashboard statistics are temporarily unavailable.",
        }),
      });
    });

    await page.goto("/admin_console");

    await expect(page.getByRole("alert")).toContainText(
      "Dashboard statistics are temporarily unavailable.",
    );
    await expect(page.getByText("Registered users").locator("..")).not.toBeVisible();
    await expect(page.getByText("0")).not.toBeVisible();
  });
});
