const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

let greetInputEl;
let greetMsgEl;
let wallpapers;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function get_papers() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let papers = JSON.parse(await invoke("get_papers_list", {}));
  console.log(papers);
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    get_papers();
  });

  document.querySelector("#update-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  wallpapers = document.querySelector("#wallpapers");
});

const unlisten = await listen('click', (event) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object
  console.log(event.event, event.payload)
})

// emits the `click` event with the object payload
emit('click', {
  theMessage: 'Tauri is awesome!',
})
