const CACHE = "dol-save-server-pwa-cache";

importScripts('https://storage.googleapis.com/workbox-cdn/releases/6.4.1/workbox-sw.js');

workbox.routing.registerRoute(
  new RegExp('/api/.*'),
  new workbox.strategies.NetworkOnly()
);

workbox.routing.registerRoute(
  new RegExp('/.*'),
  new workbox.strategies.NetworkFirst({
    cacheName: CACHE
  })
);
