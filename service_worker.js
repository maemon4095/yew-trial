var version = 'yew-trial 0.0.0';
var files = [
    'index.html','manifest.json','yew-trial-c06082c52bff8413.js','yew-trial-c06082c52bff8413_bg.wasm','assets/images/ycon.svg'
];


self.addEventListener('install', function (e) {
    e.waitUntil(
        caches.open(version).then(function (cache) {
            return cache.addAll(files);
        })
    );
});

self.addEventListener('fetch', function (e) {
    e.respondWith(
        caches.match(e.request).then(function (response) {
            return response || fetch(e.request)
        })
    );
});