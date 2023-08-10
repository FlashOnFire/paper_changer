const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

let wallpapers;
let selectedWallpaper;

let selectedWallpaperImg;
let selectedWallpaperTitle;

async function fetch_wallpapers() {
  await emit("fetch_wallpapers");
}

async function get_papers() {
  let papers = await invoke("get_papers_list", {});
  console.log(papers);

  wallpapers.innerHTML = "";

  papers.forEach((paper) => {
    wallpapers.innerHTML += "<img width=\"200\" src=" + paper + " />";
  });
}

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#filter-btn").addEventListener("click", (e) => {
    get_papers();
  });

  document.querySelector("#update-btn").addEventListener("click", (e) => {
    fetch_wallpapers();
  });

  wallpapers = document.querySelector("#wallpapers");

  wallpapers.addEventListener("click", onWallpaperClick);

  selectedWallpaperImg = document.querySelector(".right_panel .selected-paper-img");
  selectedWallpaperTitle = document.querySelector(".right_panel .selected-paper-title");

  emit('loaded');
});

await listen('addWallpaper', (event) => {
  //console.log(event.event, event.payload)

  let paper = event.payload;

  wallpapers.innerHTML += "<div class=\"wallpaper\" id=\"" + paper.id + "\">" +
                            "<img class=\"wallpaper-img\" width=\"200\" src=" + paper.preview_url + " />" + 
                            "<div class=\"wallpaper-title-container\"><span class=\"wallpaper-title\">" + paper.title + "</span></div>" +
                          "</div>";
})

await listen('updateSelectedWallpaperInfo', (event) => {
  //console.log(event.event, event.payload)
  let paper = event.payload;

  selectedWallpaperImg.src = paper.preview_url;
  selectedWallpaperTitle.innerHTML = paper.title;
})

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

    emit('wallpaperSelected', {id: target.id});
  }
}