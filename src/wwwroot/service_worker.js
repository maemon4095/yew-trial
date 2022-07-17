var version = 'yew-trial 0.0.0';
var files = [
    //<:replace:>
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