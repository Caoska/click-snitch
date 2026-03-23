export interface ClickSnitchOptions {
  serverUrl?: string;
}

export function initClickSnitch(options?: ClickSnitchOptions) {
  const serverUrl = options?.serverUrl ?? "http://localhost:3000/";

  async function sendEvent(eventData: any) {
    console.log(eventData);
    try {
      await fetch(serverUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(eventData)
      });
    } catch (err) {
      console.error("ClickSnitch error:", err);
    }
  }

  function getSessionId(): string {
    const existing = localStorage.getItem("clicksnitch_session");
    if (existing) return existing;

    const newId = crypto.randomUUID();
    localStorage.setItem("clicksnitch_session", newId);
    return newId;
  }

  const sessionId = getSessionId();
  let lastClick = 0;

  // Capture pageview on load
  sendEvent({
    sessionId,
    event: "pageview",
    text: document.title,
    tag: "page",
    path: window.location.pathname,
    timestamp: Date.now().toString()
  });

  // Auto-detect clicks on buttons and links
  document.addEventListener("click", (e) => {
    const now = Date.now();
    const target = e.target;
    if (!(target instanceof HTMLElement)) return;

    const tag = target.tagName.toLowerCase();
    if (tag === "button" || tag === "a") {
      sendEvent({
        sessionId,
        event: now - lastClick < 300 ? "rapid_click" : "click",
        text: target.innerText || target.getAttribute("aria-label") || "",
        tag: tag,
        id: target.id,
        classes: target.className,
        path: window.location.pathname,
        timestamp: Date.now().toString()
      });
    }
  });

  document.addEventListener("submit", (e) => {
    const form = e.target as HTMLFormElement;

    sendEvent({
      sessionId,
      event: "form_submit",
      tag: "FORM",
      id: form.id,
      classes: form.className,
      path: window.location.pathname,
        timestamp: Date.now().toString()
    });
  });
}
