const serviceName = "firefox-extension-logger";
const manifest = browser.runtime.getManifest();
function post_event(event) {
  const payload = {
    service_name: serviceName,
    app: {
      instance_id: browser.runtime.id,
    },
    service: {
      version: manifest.version,
      environment: "development",
    },
    user: {
      id: "user-01",
      name: "Alice",
      email: "alice@example.com",
      session_id: "sess-123",
    },
    ...event,
  };
  //console.log("Event:", payload);
  fetch("http://zg-firefox-logger.dot:3432/api/v1/e/i/basic", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify([payload]),
  })
    .then((response) => {
      if (!response.ok) {
        console.error(
          `Event: Server error logging event: ${response.status} ${response.statusText}`,
        );
      } else {
        //console.log(`Event:  event logged : ${response.status}`);
      }
    })
    .catch((error) => {
      console.error("Failed to send log to endpoint:", error);
    });
}
function logRequest(req) {
  if (req.url.indexOf("zg-firefox-logger.dot") != -1) return;
  const payload = {
    timestamp: new Date().toISOString(),
    severity: "INFO",
    message: `${req.method} ${req.url}`,
    event_name: "http_request",
    http: {
      method: req.method,
      path: req.url,
      url: req.url,
      origin: req.originUrl,
    },
    tags: ["client", "web-request"],
    labels: { browser: "firefox" },
    meta: {
      url: req.url,
      method: req.method,
      type: req.type,
      timestamp: req.timeStamp,
      id: req.requestId,
      document_url: req.documentUrl,
      incognito: req.incognito,
      origin_url: req.originUrl,
    },
  };

  post_event(payload);
}
const logNewTab = (tab) => {
  const payload = {
    timestamp: new Date().toISOString(),
    severity: "INFO",
    message: `Tab ${tab.url || "about:blank"}`,
    event_name: "tab_created",
    meta: {
      tabId: tab.id,
      url: tab.url || "about:blank",
      windowId: tab.windowId,
    },
    tags: ["client", "tab-event"],
    labels: { browser: "firefox" },
  };

  post_event(payload);
};
browser.tabs.onCreated.addListener(logNewTab);
browser.webRequest.onBeforeRequest.addListener(logRequest, {
  urls: ["<all_urls>"],
});
