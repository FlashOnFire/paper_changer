const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

let input;
let wallpapers;
let selectedWallpaper;

let selectedWallpaperImg;
let selectedWallpaperTitle;

let firstLoad = false;

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#filter-btn").addEventListener("click", (e) => {
  });

  document.querySelector("#update-btn").addEventListener("click", (e) => {
    invoke("apply_filter", {search: input.value});
  });

  input = document.querySelector("#search-input");
  input.addEventListener("keyup", (e) => {
    invoke("apply_filter", {search: input.value});
  });

  wallpapers = document.querySelector("#wallpapers");

  wallpapers.addEventListener("click", onWallpaperClick);

  selectedWallpaperImg = document.querySelector(".right_panel .selected-paper-img");
  selectedWallpaperTitle = document.querySelector(".right_panel .selected-paper-title");

  invoke("fetch_wallpapers");
  invoke('loaded');
  getMonitorsList();
});

await listen('updated', (event) => {
  if (!firstLoad) {
    firstLoad = true;
    invoke("apply_filter", {search: input.value});
  }
});

await listen('addWallpapers', (event) => {
  console.log(event.event, event.payload)
  let papers = event.payload;

  papers.forEach((paper) => {
    wallpapers.innerHTML += "<div class=\"wallpaper\" id=\"" + paper.id + "\">" +
        "<img class=\"wallpaper-img\" width=\"200\" src=" + paper.preview_url + " />" +
        "<div class=\"wallpaper-title-container\"><span class=\"wallpaper-title\">" + paper.title + "</span></div>" +
        "</div>";
  });
})

await listen('updateSelectedWallpaperInfo', (event) => {
  let paper = event.payload;

  selectedWallpaperImg.src = paper.preview_url;
  selectedWallpaperTitle.innerHTML = paper.title;
})

await listen('clearWallpapers', (event) => {
  wallpapers.innerHTML = "";
})

async function getMonitorsList() {
  console.log("Getting monitors list");
  let monitors = await invoke('get_monitors');
  console.log(monitors);
}

function onWallpaperClick(e) {
  let target = e.target;

  if (target.classList.contains("wallpaper-img") || target.classList.contains("wallpaper-title-container")) {
    target = target.parentNode;
  } else if (target.classList.contains("wallpaper-title")) {
    target = target.parentNode.parentNode;
  }

  if (target.classList.contains("wallpaper")) {
    console.log("Clicked on wallpaper " + target.id);
    if (selectedWallpaper != null) {
      selectedWallpaper.classList.remove("selected");
    }
    target.classList.add("selected");
    selectedWallpaper = target

    invoke('select_wallpaper', {id: parseInt(target.id)});
  }
}