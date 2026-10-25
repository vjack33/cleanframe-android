/* CleanFrame Android Device Adapter for Android TV / Google TV */
(function () {
  'use strict';

  var ANDROID_BRAND_PACKAGES = {
    netflix: 'com.netflix.ninja',
    youtube: 'com.google.android.youtube.tv',
    primevideo: 'com.amazon.amazonvideo.livingroom',
    disneyplus: 'in.startv.hotstar',
    spotify: 'com.spotify.tv.android',
    stremio: 'com.stremio.one',
    appletv: 'com.apple.atve.androidtv.appletv',
    applemusic: 'com.apple.android.music',
    hbomax: 'com.wbd.stream',
    tubi: 'com.tubitv',
    abciview: 'au.net.abc.iview',
    sbs: 'sbs.ondemand.tv',
    zee5: 'com.graymatrix.did',
    plex: 'com.plexapp.android',
    jellyfin: 'org.jellyfin.androidtv',
    wholphin: 'com.github.damontecres.wholphin',
    aptoide: 'cm.aptoidetv.pt',
    aurora: 'com.aurora.store',
    projectivy: 'com.spocky.projengmenu',
    tvsettings: 'com.android.tv.settings',
    mediaplayer: 'com.tcl.ui_mediaCenter',
    livetv: 'com.tcl.tv',
    tclhome: 'com.tcl.dashboard',
    quickpanel: 'com.tcl.suspension'
  };

  function hasAndroid() {
    return typeof window.AndroidBridge !== 'undefined';
  }

  function launchApp(id, done) {
    if (hasAndroid()) {
      try {
        var target = ANDROID_BRAND_PACKAGES[id] || id;
        var res = window.AndroidBridge.launchApp(target);
        if (res === 'ok') {
          if (done) done(null);
        } else {
          if (done) done({ message: 'The TV could not open this app.', detail: res });
        }
      } catch (err) {
        if (done) done({ message: 'The TV could not open this app.', detail: err && err.message });
      }
      return;
    }
    if (done) done({ message: 'Open CleanFrame on the TV to use this control.', detail: 'The web preview cannot control the TV.' });
  }

  function launch(brand, done) {
    var target = ANDROID_BRAND_PACKAGES[brand] || brand;
    launchApp(target, done);
  }

  function art(done) {
    if (window.openArtMode) {
      window.openArtMode();
      if (done) done(null);
      return;
    }
    if (hasAndroid()) {
      try {
        window.AndroidBridge.launchArt();
        if (done) done(null);
      } catch (err) {
        if (done) done({ message: 'Could not open Art Mode.', detail: err && err.message });
      }
      return;
    }
    if (done) done({ message: 'Open CleanFrame on the TV to use this control.' });
  }

  function home(done) {
    if (hasAndroid()) {
      try {
        window.AndroidBridge.launchHome();
        if (done) done(null);
      } catch (err) {
        if (done) done({ message: 'Could not switch to default TV launcher.', detail: err && err.message });
      }
      return;
    }
    if (done) done({ message: 'Open CleanFrame on the TV to use this control.' });
  }

  function settings(done) {
    if (hasAndroid()) {
      try {
        window.AndroidBridge.launchSettings();
        if (done) done(null);
      } catch (err) {
        if (done) done({ message: 'Could not open TV Settings.', detail: err && err.message });
      }
      return;
    }
    if (done) done({ message: 'Open CleanFrame on the TV to use this control.' });
  }

  function inputs(done) {
    if (!hasAndroid()) { done(null, []); return; }
    try {
      var list = JSON.parse(window.AndroidBridge.getTvInputs());
      if (done) done(null, Array.isArray(list) ? list : []);
    } catch (err) {
      if (done) done({ message: 'Could not read TV inputs.', detail: err && err.message });
    }
  }

  function launchInput(inputId, done) {
    if (!hasAndroid()) {
      if (done) done({ message: 'TV input switching is unavailable in the web preview.' });
      return;
    }
    try {
      var result = window.AndroidBridge.launchInput(inputId);
      if (done) done(result === 'ok' ? null : { message: result || 'The TV could not switch inputs.' });
    } catch (err) {
      if (done) done({ message: 'The TV could not switch inputs.', detail: err && err.message });
    }
  }

  function nowPlaying(done) {
    if (!hasAndroid()) { done(null, { access: false, available: false }); return; }
    try {
      done(null, JSON.parse(window.AndroidBridge.getNowPlaying()));
    } catch (err) {
      done({ message: 'Could not read current media.', detail: err && err.message });
    }
  }

  function controlMedia(action, done) {
    if (!hasAndroid()) {
      if (done) done({ message: 'Media controls are available on the TV only.' });
      return;
    }
    try {
      var result = window.AndroidBridge.controlMedia(action);
      if (done) done(result === 'ok' ? null : { message: result || 'Media control failed.' });
    } catch (err) {
      if (done) done({ message: 'Media control failed.', detail: err && err.message });
    }
  }

  function requestNowPlayingAccess(done) {
    if (!hasAndroid()) {
      if (done) done({ message: 'Notification access can only be enabled on the TV.' });
      return;
    }
    try {
      window.AndroidBridge.requestNowPlayingAccess();
      if (done) done(null);
    } catch (err) {
      if (done) done({ message: 'Could not open notification access settings.', detail: err && err.message });
    }
  }

  function iconPacks(done) {
    if (!hasAndroid()) { done(null, []); return; }
    try {
      var packs = JSON.parse(window.AndroidBridge.getIconPacks());
      if (done) done(null, Array.isArray(packs) ? packs : []);
    } catch (err) {
      if (done) done({ message: 'Could not read installed icon packs.', detail: err && err.message });
    }
  }

  function iconPackIcons(packageName, appIds, done) {
    if (!hasAndroid()) { done(null, {}); return; }
    try {
      var images = JSON.parse(window.AndroidBridge.getIconPackIcons(packageName, JSON.stringify(appIds)));
      if (done) done(null, images && typeof images === 'object' ? images : {});
    } catch (err) {
      if (done) done({ message: 'Could not load icons from this pack.', detail: err && err.message });
    }
  }

  function apps(done) {
    if (hasAndroid()) {
      try {
        var raw = window.AndroidBridge.getInstalledApps();
        var list = JSON.parse(raw);
        if (done) done(null, list);
      } catch (err) {
        if (done) done({ message: 'Failed to read installed apps.', detail: err && err.message });
      }
      return;
    }
    if (done) done(null, [
      { appId: 'com.netflix.ninja', name: 'Netflix', brand: 'netflix', rank: 1 },
      { appId: 'com.google.android.youtube.tv', name: 'YouTube', brand: 'youtube', rank: 2 },
      { appId: 'com.amazon.amazonvideo.livingroom', name: 'Prime Video', brand: 'primevideo', rank: 3 },
      { appId: 'com.spotify.tv.android', name: 'Spotify', brand: 'spotify', rank: 4 },
      { appId: 'com.stremio.one', name: 'Stremio', brand: 'stremio', rank: 5 }
    ]);
  }

  window.FrameDevice = {
    launch: launch,
    launchApp: launchApp,
    art: art,
    home: home,
    settings: settings,
    inputs: inputs,
    launchInput: launchInput,
    nowPlaying: nowPlaying,
    controlMedia: controlMedia,
    requestNowPlayingAccess: requestNowPlayingAccess,
    iconPacks: iconPacks,
    iconPackIcons: iconPackIcons,
    apps: apps
  };
})();
