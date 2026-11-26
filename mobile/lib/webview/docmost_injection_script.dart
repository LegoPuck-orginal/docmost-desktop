import 'dart:convert';

import '../models/app_config.dart';

/// Builds the lightweight JavaScript & CSS bridge injected into Docmost's WKWebView.
class DocmostInjectionScript {
  DocmostInjectionScript._();

  static const String globalCss = '''
::-webkit-scrollbar { width: 6px; height: 6px; }
::-webkit-scrollbar-track { background: #141414; }
::-webkit-scrollbar-thumb { background: #2e2e2e; border-radius: 3px; }
body.has-desktop-menu .mantine-AppShell-header { padding-right: 52px !important; }
* { -webkit-tap-highlight-color: transparent; }
''';

  static const String excalidrawCss = '''
.excalidraw,
.excalidraw-container,
[data-testid="excalidraw-container"] {
  --theme-filter: none !important;
  --color-surface-low: #1f1f1f !important;
  --color-surface-mid: #242424 !important;
  --color-surface-high: #2e2e2e !important;
  --color-surface-lowest: #141414 !important;
  --color-on-surface: #c9c9c9 !important;
  --color-muted: #828282 !important;
  --color-border: #383838 !important;
  --color-primary: #228be6 !important;
  --color-primary-darker: #1c7ed6 !important;
  --color-primary-light: #74c0fc !important;
  background-color: #141414 !important;
  touch-action: none !important;
  -webkit-touch-callout: none !important;
  user-select: none !important;
  -webkit-user-select: none !important;
}
.excalidraw canvas,
.excalidraw .excalidraw__canvas,
.excalidraw .interactive {
  background-color: #141414 !important;
  touch-action: none !important;
  -webkit-touch-callout: none !important;
  user-select: none !important;
  -webkit-user-select: none !important;
}
.excalidraw .Island,
.excalidraw .App-toolbar,
.excalidraw .layer-ui__wrapper .Island,
.excalidraw .panelColumn,
.excalidraw .context-menu,
.excalidraw .dropdown-menu,
.excalidraw .popover,
.excalidraw .color-picker {
  background: #242424 !important;
  border: 1px solid #383838 !important;
  border-radius: 8px !important;
  color: #c9c9c9 !important;
}
.excalidraw .ToolIcon--selected,
.excalidraw .ToolIcon.active,
.excalidraw .ToolIcon[aria-pressed="true"] {
  background: #228be6 !important;
  color: #ffffff !important;
  border-color: #339af0 !important;
}
#docmost-pen-toolbar {
  position: fixed;
  bottom: max(18px, env(safe-area-inset-bottom));
  left: 50%;
  transform: translateX(-50%);
  background: #242424;
  border: 1px solid #383838;
  border-radius: 36px;
  padding: 6px 12px;
  box-shadow: 0 12px 32px rgba(0,0,0,0.75);
  z-index: 999999;
  display: none;
  align-items: center;
  gap: 6px;
  max-width: 94vw;
  overflow-x: auto;
  font-family: -apple-system, BlinkMacSystemFont, sans-serif;
  user-select: none;
  -webkit-user-select: none;
}
.docmost-pen-btn {
  background: transparent;
  border: 1px solid transparent;
  color: #c9c9c9;
  border-radius: 18px;
  padding: 6px 10px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  white-space: nowrap;
}
.docmost-pen-btn.active {
  background: #228be6;
  color: #ffffff;
}
.docmost-color-dot {
  width: 22px;
  height: 22px;
  border-radius: 50%;
  border: 2px solid rgba(255,255,255,0.25);
  cursor: pointer;
  flex-shrink: 0;
  padding: 0;
}
.docmost-pen-stroke-btn {
  background: transparent;
  border: 1px solid transparent;
  color: #828282;
  border-radius: 6px;
  width: 28px;
  height: 28px;
  font-size: 13px;
  cursor: pointer;
  flex-shrink: 0;
}
.docmost-pen-stroke-btn.active {
  background: #2e2e2e;
  color: #339af0;
  border-color: #228be6;
}
''';

  static String compactCss(String css) {
    return css
        .split('\n')
        .map((line) => line.trim())
        .where((line) => line.isNotEmpty)
        .join(' ');
  }

  static Map<String, dynamic> configPayload(AppConfig config) {
    return <String, dynamic>{
      'penMode': config.excalidrawPenMode,
      'palmRejection': config.excalidrawPalmRejection,
      'darkTheme': config.excalidrawDarkTheme,
      'quickToolbar': config.excalidrawQuickToolbar,
      'zoomLevel': config.uiZoomLevel,
      'onlyofficeUrl': config.onlyofficeUrl,
      'mediaPlayerEnabled': config.mediaPlayerEnabled,
      'officeViewerEnabled': config.officeViewerEnabled,
    };
  }

  static String buildSyncScript(AppConfig config) {
    final cfgJson = jsonEncode(configPayload(config));
    return 'if (window.__docmostSyncConfig) { window.__docmostSyncConfig($cfgJson); }';
  }

  static String build(AppConfig config) {
    final globalCssJs = jsonEncode(compactCss(globalCss));
    final excalidrawCssJs = jsonEncode(compactCss(excalidrawCss));
    final cfgJson = jsonEncode(configPayload(config));

    return '''
(function() {
  try {
    if (window.top !== window.self) return;
    if (window.__docmostMobileInitialized) {
      if (window.__docmostSyncConfig) window.__docmostSyncConfig($cfgJson);
      return;
    }
    window.__docmostMobileInitialized = true;
    window.__docmostConfig = $cfgJson;

    var globalInjected = false;
    var excalidrawCssInjected = false;
    var excalidrawPresent = false;
    var lastZoomApplied = null;
    var lastPenTime = 0;

    function injectStyle(id, cssText) {
      if (document.getElementById(id)) return true;
      var target = document.head || document.documentElement;
      if (!target) return false;
      var style = document.createElement('style');
      style.id = id;
      style.textContent = cssText;
      target.appendChild(style);
      return true;
    }

    function applyHeaderReservation() {
      if (document.body && !document.body.classList.contains('has-desktop-menu')) {
        document.body.classList.add('has-desktop-menu');
      }
    }

    function applyZoom() {
      var z = (window.__docmostConfig && window.__docmostConfig.zoomLevel) || 1.0;
      if (!document.documentElement || z === lastZoomApplied) return;
      if (z !== 1.0 || lastZoomApplied !== null) {
        document.documentElement.style.zoom = String(z);
      }
      lastZoomApplied = z;
    }

    function syncExcalidrawState() {
      if (!excalidrawPresent) return;
      try {
        var raw = localStorage.getItem('excalidraw-state');
        var st = raw ? JSON.parse(raw) : {};
        var dirty = false;
        if (window.__docmostConfig.darkTheme) {
          if (st.theme !== 'dark') { st.theme = 'dark'; dirty = true; }
          if (localStorage.getItem('excalidraw-theme') !== 'dark') {
            localStorage.setItem('excalidraw-theme', 'dark');
          }
        }
        if (window.__docmostConfig.penMode) {
          if (!st.penMode || !st.penDetected) {
            st.penMode = true;
            st.penDetected = true;
            dirty = true;
          }
        }
        if (dirty) localStorage.setItem('excalidraw-state', JSON.stringify(st));
      } catch (e) {}
    }

    function createPenToolbar() {
      if (document.getElementById('docmost-pen-toolbar') || !document.body) return;
      var tb = document.createElement('div');
      tb.id = 'docmost-pen-toolbar';
      tb.innerHTML =
        '<button id="tb-btn-pen" class="docmost-pen-btn active" data-tool="freedraw">🖊️ Stift</button>' +
        '<button id="tb-btn-eraser" class="docmost-pen-btn" data-tool="eraser">🧹 Radierer</button>' +
        '<button class="docmost-color-dot" style="background:#ffffff" data-color="#ffffff"></button>' +
        '<button class="docmost-color-dot" style="background:#339af0" data-color="#339af0"></button>' +
        '<button class="docmost-color-dot" style="background:#12b886" data-color="#12b886"></button>' +
        '<button class="docmost-color-dot" style="background:#fab005" data-color="#fab005"></button>' +
        '<button class="docmost-color-dot" style="background:#fa5252" data-color="#fa5252"></button>' +
        '<button class="docmost-pen-stroke-btn" data-stroke="1">―</button>' +
        '<button class="docmost-pen-stroke-btn active" data-stroke="2">━</button>' +
        '<button class="docmost-pen-stroke-btn" data-stroke="4">█</button>' +
        '<button class="docmost-pen-btn" data-action="undo">↩</button>' +
        '<button class="docmost-pen-btn" data-action="redo">↪</button>' +
        '<button class="docmost-pen-btn" data-action="hide" style="color:#828282">✕</button>';

      tb.addEventListener('click', function(ev) {
        var btn = ev.target.closest('button');
        if (!btn) return;
        var tool = btn.getAttribute('data-tool');
        var color = btn.getAttribute('data-color');
        var stroke = btn.getAttribute('data-stroke');
        var action = btn.getAttribute('data-action');
        if (tool === 'freedraw' || tool === 'eraser') {
          var pBtn = document.getElementById('tb-btn-pen');
          var eBtn = document.getElementById('tb-btn-eraser');
          if (pBtn) pBtn.classList.toggle('active', tool === 'freedraw');
          if (eBtn) eBtn.classList.toggle('active', tool === 'eraser');
          var sel = document.querySelector('input[value="' + tool + '"]') ||
                    document.querySelector('[data-testid="' + tool + '"]');
          if (sel) {
            sel.click();
          } else {
            document.dispatchEvent(new KeyboardEvent('keydown', {
              key: tool === 'freedraw' ? 'p' : 'e',
              code: tool === 'freedraw' ? 'KeyP' : 'KeyE',
              bubbles: true
            }));
          }
        } else if (color) {
          try {
            var r = localStorage.getItem('excalidraw-state');
            if (r) {
              var s = JSON.parse(r);
              s.currentItemStrokeColor = color;
              localStorage.setItem('excalidraw-state', JSON.stringify(s));
            }
          } catch (e) {}
        } else if (stroke) {
          tb.querySelectorAll('.docmost-pen-stroke-btn').forEach(function(b) {
            b.classList.toggle('active', b === btn);
          });
          try {
            var r2 = localStorage.getItem('excalidraw-state');
            if (r2) {
              var s2 = JSON.parse(r2);
              s2.currentItemStrokeWidth = Number(stroke);
              localStorage.setItem('excalidraw-state', JSON.stringify(s2));
            }
          } catch (e) {}
        } else if (action === 'undo') {
          document.dispatchEvent(new KeyboardEvent('keydown', { key: 'z', code: 'KeyZ', metaKey: true, ctrlKey: true, bubbles: true }));
        } else if (action === 'redo') {
          document.dispatchEvent(new KeyboardEvent('keydown', { key: 'z', code: 'KeyZ', shiftKey: true, metaKey: true, ctrlKey: true, bubbles: true }));
        } else if (action === 'hide') {
          tb.style.display = 'none';
        }
      });
      document.body.appendChild(tb);
    }

    function checkExcalidrawPresence() {
      var found = !!document.querySelector('.excalidraw, [data-testid="excalidraw-container"]');
      if (found) {
        if (!excalidrawCssInjected) {
          excalidrawCssInjected = injectStyle('docmost-mobile-excalidraw-style', $excalidrawCssJs);
        }
        if (window.__docmostConfig.darkTheme) {
          document.querySelectorAll('.excalidraw:not(.theme--dark)').forEach(function(el) {
            el.classList.add('theme--dark');
          });
        }
        if (!excalidrawPresent) {
          excalidrawPresent = true;
          syncExcalidrawState();
        }
        if (window.__docmostConfig.quickToolbar) {
          createPenToolbar();
        }
      } else {
        excalidrawPresent = false;
      }
      var tb = document.getElementById('docmost-pen-toolbar');
      if (tb) {
        tb.style.display = (found && window.__docmostConfig.quickToolbar) ? 'flex' : 'none';
      }
    }

    function runCheck() {
      if (!globalInjected) {
        globalInjected = injectStyle('docmost-mobile-style', $globalCssJs);
      }
      applyHeaderReservation();
      applyZoom();
      checkExcalidrawPresence();
    }

    window.__docmostSyncConfig = function(newCfg) {
      if (!newCfg) return;
      window.__docmostConfig = Object.assign({}, window.__docmostConfig || {}, newCfg);
      applyZoom();
      checkExcalidrawPresence();
      syncExcalidrawState();
    };

    window.addEventListener('pointerdown', function(e) {
      if (!excalidrawPresent) return;
      if (e.pointerType === 'pen') {
        lastPenTime = Date.now();
      } else if (e.pointerType === 'touch' && window.__docmostConfig.palmRejection) {
        var onCanvas = e.target.closest('.excalidraw canvas, .excalidraw');
        if (onCanvas && (Date.now() - lastPenTime < 1200 || window.__docmostConfig.penMode)) {
          if (e.isPrimary) e.stopPropagation();
        }
      }
    }, true);

    window.addEventListener('pointerup', function(e) {
      if (excalidrawPresent && e.pointerType === 'pen') {
        lastPenTime = Date.now();
      }
    }, true);

    document.addEventListener('click', function(e) {
      var cfg = window.__docmostConfig;
      if (!cfg || (!cfg.mediaPlayerEnabled && !cfg.officeViewerEnabled)) return;
      var link = e.target.closest('a');
      if (!link) return;
      var href = link.getAttribute('href') || '';
      if (!href || href.startsWith('javascript:') || href.startsWith('#')) return;
      var download = link.getAttribute('download') || '';
      var text = (link.innerText || '').trim();
      var cleanHref = href.split('?')[0].split('#')[0].toLowerCase();
      var tLower = text.toLowerCase();
      var dLower = download.toLowerCase();

      var mediaExts = ['.mp4','.webm','.mov','.mkv','.m4v','.mp3','.wav','.ogg','.m4a','.aac','.flac'];
      var officeExts = ['.docx','.doc','.pptx','.ppt','.xlsx','.xls','.odt','.ods','.odp','.pdf'];
      var isMedia = mediaExts.some(function(ext) {
        return cleanHref.endsWith(ext) || tLower.endsWith(ext) || dLower.endsWith(ext);
      });
      var isOffice = officeExts.some(function(ext) {
        return cleanHref.endsWith(ext) || tLower.endsWith(ext) || dLower.endsWith(ext);
      });

      if ((isMedia && cfg.mediaPlayerEnabled) || (isOffice && cfg.officeViewerEnabled)) {
        e.preventDefault();
        e.stopPropagation();
        var resolvedUrl = href;
        try { resolvedUrl = new URL(href, window.location.href).href; } catch (err) {}
        if (window.DocmostBridge && window.DocmostBridge.postMessage) {
          window.DocmostBridge.postMessage(JSON.stringify({
            action: 'open_attachment',
            url: resolvedUrl,
            title: text,
            download: download
          }));
        }
      }
    }, true);

    var scheduled = false;
    function scheduleCheck() {
      if (scheduled) return;
      scheduled = true;
      requestAnimationFrame(function() {
        scheduled = false;
        runCheck();
      });
    }

    runCheck();
    var target = document.body || document.documentElement;
    if (target) {
      var obs = new MutationObserver(scheduleCheck);
      obs.observe(target, { childList: true, subtree: true });
    }
  } catch (err) {}
})();
''';
  }
}
