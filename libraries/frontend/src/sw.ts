// Install event
self.addEventListener("install", () => {
  // Activate immediately without waiting
  self.skipWaiting();
});

// Activate event
self.addEventListener("activate", (event: ExtendableEvent) => {
  // Take control of the page immediately
  event.waitUntil(self.clients.claim());
});

type NotificationType = {
  body: string;
  table: string;
  title: string;
};

// Push event
self.addEventListener("push", (event: PushEvent) => {
  const notifications = event.data?.json() as NotificationType[];

  notifications.forEach(({ title, body, table }) => {
    const url = `/tables/${table}`;
    const data = {
      body,
      icon: "/favicon.ico",
      badge: "/favicon.ico",
    };
    // Check if the relevant URL is already active and focused
    event.waitUntil(
      self.clients
        .matchAll({ type: "window", includeUncontrolled: true })
        .then((clients) => {
          for (const client of clients) {
            if (client.url.includes(url) && client.focused) {
              // If the relevant URL is already active and focused, do not show the notification
              console.log(
                "Relevant URL is already active and focused, skipping notification"
              );
              return;
            }
          }
          // If no active and focused client matches the relevant URL, show the notification
          console.log("Push notification", title, data);
          event.waitUntil(self.registration.showNotification(title, data));
        })
    );
  });
});

// Notification click event
self.addEventListener("notificationclick", (event: NotificationEvent) => {
  event.notification.close();

  const data = event.notification.data as NotificationType;
  if (data.table === undefined) return;

  const urlToOpen = `${
    !event.target || !("origin" in event.target)
      ? "https://zkpoker.app"
      : event.target.origin
  }/tables/${data.table}`;

  event.waitUntil(
    self.clients
      .matchAll({ type: "window", includeUncontrolled: true })
      .then((clientsArr) => {
        const client = clientsArr.find((c) => c.url.includes(urlToOpen));

        if (client) {
          // Focus the existing client with the desired URL
          return client.focus();
        } else if (clientsArr.length > 0) {
          // Navigate the first available client to the URL and focus it
          return clientsArr[0]
            .navigate(urlToOpen)
            .then(() => clientsArr[0].focus());
        } else if (self.clients.openWindow) {
          // Open a new window with the URL
          return self.clients.openWindow(urlToOpen);
        }
      })
  );
});

// Optional: Notification close event
self.addEventListener("notificationclose", () => {
  // Handle analytics or clean up tasks here
});
