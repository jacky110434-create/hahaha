// sw.js
const CACHE_NAME = "蘇州 上海-guide-v1";

// 把你要離線可用的檔案列在這裡
const ASSETS = [
  "./",
  "./index.html",
  "./manifest.json",
  "./sw.js",
  // 如果你有外接 css/js，就加進來：
  // "./styles.css",
  // "./app.js",

  // 圖片（只加你真的會用到的）
  // "./images/虎丘.jpg",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS))
  );
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(
        keys.map((key) => (key !== CACHE_NAME ? caches.delete(key) : null))
      )
    )
  );
  self.clients.claim();
});

// 先用快取，沒有再抓網路（Cache-first）
self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((cached) => cached || fetch(event.request))
  );
});
