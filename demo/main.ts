import { initClickSnitch } from "clicksnitch";

initClickSnitch({ serverUrl: "http://localhost:3000/collect" });

async function fetchEvents() {
  try {
    const res = await fetch("http://localhost:3000/events");
    const sessions = await res.json();

    const container = document.getElementById("events")!;
    container.innerHTML = "";

    Object.entries(sessions).forEach(([sessionId, events]: any) => {
      // Create a section per session
      const section = document.createElement("div");
      section.style.marginBottom = "1.5rem";

      const title = document.createElement("h3");
      title.textContent = `Session: ${sessionId}`;

      section.appendChild(title);

      const list = document.createElement("ul");

      events.forEach((e: any) => {
        const li = document.createElement("li");
        li.textContent = `${new Date(+e.timestamp).toLocaleString()}: Object [${e.tag}] named [${e.text}] received action [${e.event}] on path [${e.path}]`;
        list.appendChild(li);
      });

      section.appendChild(list);
      container.appendChild(section);
    });

  } catch (err) {
    console.error("Failed to fetch events:", err);
  }
}

setInterval(fetchEvents, 1000);
fetchEvents();
