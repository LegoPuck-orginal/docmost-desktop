pub const CLEAN_DARK_GLOBAL_CSS: &str = r#"
/* Docmost Desktop - Clean Dark Enhancements */
:root {
    --bg-dark-primary: #0d0f17;
    --bg-dark-secondary: #131722;
    --bg-dark-card: #1a1e2e;
    --border-dark: #252b40;
    --text-primary: #f1f5f9;
    --text-muted: #94a3b8;
    --accent-indigo: #6366f1;
    --accent-indigo-hover: #4f46e5;
    --color-success: #10b981;
    --color-warning: #f59e0b;
}

/* Custom Sleek Dark Scrollbars */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}
::-webkit-scrollbar-track {
    background: #0d0f17;
}
::-webkit-scrollbar-thumb {
    background: #23293d;
    border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
    background: #363f5c;
}

/* Injected Minimalist Dark Top Bar */
#docmost-desktop-nav {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 38px;
    background: #111420;
    border-bottom: 1px solid #1f2538;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    z-index: 9999999;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    font-size: 13px;
    color: #e2e8f0;
    user-select: none;
    box-shadow: 0 2px 8px rgba(0,0,0,0.4);
}

body.has-desktop-nav {
    padding-top: 38px !important;
}

.docmost-nav-group {
    display: flex;
    align-items: center;
    gap: 6px;
}

.docmost-nav-btn {
    background: transparent;
    border: 1px solid transparent;
    color: #94a3b8;
    border-radius: 5px;
    padding: 4px 8px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 4px;
    transition: all 0.15s ease;
}

.docmost-nav-btn:hover {
    background: #1b2032;
    color: #f8fafc;
    border-color: #2b334d;
}

.docmost-nav-badge {
    background: #181d2e;
    color: #cbd5e1;
    border: 1px solid #27304a;
    border-radius: 12px;
    padding: 2px 10px;
    font-size: 11px;
    display: flex;
    align-items: center;
    gap: 6px;
}

.docmost-status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #10b981;
    box-shadow: 0 0 6px rgba(16, 185, 129, 0.6);
}

.docmost-update-banner {
    background: linear-gradient(135deg, #4f46e5, #7c3aed);
    color: #ffffff;
    font-weight: 600;
    border-radius: 6px;
    padding: 3px 10px;
    font-size: 11px;
    cursor: pointer;
    display: none;
    align-items: center;
    gap: 6px;
    animation: pulseGlow 2s infinite;
}

@keyframes pulseGlow {
    0% { box-shadow: 0 0 0 0 rgba(99, 102, 241, 0.6); }
    70% { box-shadow: 0 0 0 8px rgba(99, 102, 241, 0); }
    100% { box-shadow: 0 0 0 0 rgba(99, 102, 241, 0); }
}

/* Injected Modal Overlay */
.docmost-modal-overlay {
    display: none;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(4px);
    z-index: 10000000;
    align-items: center;
    justify-content: center;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.docmost-modal-box {
    background: #131724;
    border: 1px solid #28314a;
    border-radius: 12px;
    width: 480px;
    max-width: 90vw;
    box-shadow: 0 20px 40px rgba(0,0,0,0.6);
    color: #f1f5f9;
    overflow: hidden;
}

.docmost-modal-header {
    padding: 16px 20px;
    border-bottom: 1px solid #20273c;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.docmost-modal-body {
    padding: 20px;
}

.docmost-modal-footer {
    padding: 14px 20px;
    background: #0f121d;
    border-top: 1px solid #20273c;
    display: flex;
    justify-content: flex-end;
    gap: 10px;
}

.docmost-btn-primary {
    background: #6366f1;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s ease;
}
.docmost-btn-primary:hover {
    background: #4f46e5;
}

.docmost-btn-secondary {
    background: #1d2336;
    color: #cbd5e1;
    border: 1px solid #2f3956;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 13px;
    cursor: pointer;
}
.docmost-btn-secondary:hover {
    background: #283049;
}

/* ===== EXCALIDRAW CLEAN DARK ENHANCEMENTS ===== */
.excalidraw,
.excalidraw-container,
[data-testid="excalidraw-container"] {
    --theme-filter: none !important;
    --color-surface-low: #131725 !important;
    --color-surface-mid: #191f32 !important;
    --color-surface-high: #222a42 !important;
    --color-surface-lowest: #0c0f18 !important;
    --color-on-surface: #f1f5f9 !important;
    --color-muted: #94a3b8 !important;
    --color-border: #28314a !important;
    --color-primary: #6366f1 !important;
    --color-primary-darker: #4f46e5 !important;
    --color-primary-darkest: #4338ca !important;
    --color-primary-light: #818cf8 !important;
    background-color: #0c0f18 !important;
}

/* Excalidraw Canvas Background */
.excalidraw canvas,
.excalidraw .excalidraw__canvas {
    background-color: #0c0f18 !important;
}

/* Floating Toolbars (Island) */
.excalidraw .Island,
.excalidraw .App-toolbar,
.excalidraw .layer-ui__wrapper .Island {
    background: rgba(18, 22, 34, 0.94) !important;
    backdrop-filter: blur(16px) !important;
    -webkit-backdrop-filter: blur(16px) !important;
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    border-radius: 12px !important;
    box-shadow: 0 12px 36px -4px rgba(0, 0, 0, 0.65), 0 0 0 1px rgba(255, 255, 255, 0.05) !important;
    padding: 4px 6px !important;
}

/* Excalidraw Tool Icons (Shapes, Pencil, Text, Eraser, etc.) */
.excalidraw .ToolIcon_type_button,
.excalidraw .ToolIcon_type_radio,
.excalidraw .ToolIcon,
.excalidraw button.ToolIcon {
    border-radius: 8px !important;
    color: #94a3b8 !important;
    background: transparent !important;
    border: 1px solid transparent !important;
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1) !important;
    margin: 2px !important;
}

.excalidraw .ToolIcon:hover,
.excalidraw .ToolIcon_type_button:hover,
.excalidraw .ToolIcon_type_radio:hover {
    background: rgba(99, 102, 241, 0.16) !important;
    color: #c7d2fe !important;
    border-color: rgba(99, 102, 241, 0.3) !important;
    transform: translateY(-1px) !important;
}

.excalidraw .ToolIcon:active,
.excalidraw .ToolIcon_type_button:active {
    transform: translateY(0px) !important;
}

/* Active / Selected Tool */
.excalidraw .ToolIcon_type_radio:checked + label .ToolIcon,
.excalidraw .ToolIcon_type_radio:checked,
.excalidraw .ToolIcon--selected,
.excalidraw .ToolIcon.active,
.excalidraw .ToolIcon[aria-pressed="true"] {
    background: #6366f1 !important;
    color: #ffffff !important;
    border-color: #818cf8 !important;
    box-shadow: 0 0 12px rgba(99, 102, 241, 0.5) !important;
}

.excalidraw .ToolIcon--selected svg,
.excalidraw .ToolIcon.active svg {
    fill: #ffffff !important;
    stroke: #ffffff !important;
}

/* Excalidraw Sidebar & Properties Panel (Stroke, Fill, Color, Opacity) */
.excalidraw .panelColumn,
.excalidraw .sidebar-content,
.excalidraw .layer-ui__wrapper .sidebar,
.excalidraw .context-menu,
.excalidraw .dropdown-menu,
.excalidraw .popover {
    background: #141826 !important;
    border: 1px solid #28314a !important;
    border-radius: 12px !important;
    color: #f1f5f9 !important;
    box-shadow: 0 16px 40px rgba(0, 0, 0, 0.7) !important;
}

.excalidraw .context-menu-item,
.excalidraw .dropdown-menu-item {
    color: #cbd5e1 !important;
    border-radius: 6px !important;
    transition: all 0.12s ease !important;
}

.excalidraw .context-menu-item:hover,
.excalidraw .dropdown-menu-item:hover {
    background: #20273c !important;
    color: #818cf8 !important;
}

/* Excalidraw Color Picker Palette */
.excalidraw .color-picker,
.excalidraw .picker {
    background: #141826 !important;
    border: 1px solid #28314a !important;
    border-radius: 10px !important;
    padding: 8px !important;
}

.excalidraw .color-picker-swatch {
    border-radius: 6px !important;
    border: 1px solid rgba(255, 255, 255, 0.15) !important;
    transition: transform 0.1s ease !important;
}

.excalidraw .color-picker-swatch:hover {
    transform: scale(1.15) !important;
    border-color: #ffffff !important;
}

/* Bottom Controls (Zoom, Undo, Redo, Help, Zen Mode) */
.excalidraw .layer-ui__wrapper .footer-center,
.excalidraw .layer-ui__wrapper .footer-left,
.excalidraw .layer-ui__wrapper .footer-right,
.excalidraw .undo-redo-buttons,
.excalidraw .zoom-actions {
    background: rgba(18, 22, 34, 0.92) !important;
    backdrop-filter: blur(14px) !important;
    border: 1px solid rgba(255, 255, 255, 0.08) !important;
    border-radius: 10px !important;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5) !important;
}

/* Modal Dialog when editing full diagram */
.excalidraw-modal-container,
.excalidraw .Modal,
.excalidraw .Modal__content {
    background: #10131e !important;
    border: 1px solid #252b40 !important;
    border-radius: 14px !important;
    box-shadow: 0 25px 60px rgba(0, 0, 0, 0.8) !important;
    color: #f1f5f9 !important;
}

.excalidraw .Modal__header {
    border-bottom: 1px solid #20273c !important;
}
"#;

pub fn get_injection_script(server_url: &str, current_version: &str) -> String {
    format!(r#"
(function() {{
    if (window.__docmostDesktopInitialized) return;
    window.__docmostDesktopInitialized = true;

    // Inject CSS
    const style = document.createElement('style');
    style.innerHTML = `{css}`;
    document.head.appendChild(style);

    // Auto-sync Excalidraw Dark Mode
    try {{
        localStorage.setItem('excalidraw-theme', 'dark');
        localStorage.setItem('excalidraw-state', JSON.stringify({{ theme: 'dark' }}));
    }} catch (e) {{}}

    const ensureExcalidrawDark = () => {{
        document.querySelectorAll('.excalidraw').forEach(el => {{
            if (!el.classList.contains('theme--dark')) {{
                el.classList.add('theme--dark');
            }}
        }});
    }};
    const excalidrawObserver = new MutationObserver(ensureExcalidrawDark);
    excalidrawObserver.observe(document.documentElement, {{ childList: true, subtree: true }});
    ensureExcalidrawDark();

    // Create Navigation Bar
    const nav = document.createElement('div');
    nav.id = 'docmost-desktop-nav';
    nav.innerHTML = `
        <div class="docmost-nav-group">
            <span style="font-weight: 700; color: #818cf8; margin-right: 6px; letter-spacing: 0.5px;">DOCMOST</span>
            <button class="docmost-nav-btn" onclick="window.history.back()" title="Zurück">◀</button>
            <button class="docmost-nav-btn" onclick="window.history.forward()" title="Vorwärts">▶</button>
            <button class="docmost-nav-btn" onclick="window.location.reload()" title="Neu laden">↻</button>
            <button class="docmost-nav-btn" onclick="window.location.href='{url}'" title="Startseite">🏠</button>
        </div>
        <div class="docmost-nav-group">
            <span id="docmost-nav-url" class="docmost-nav-badge">
                <span class="docmost-status-dot"></span>
                <span>{url_display}</span>
            </span>
            <div id="docmost-update-btn" class="docmost-update-banner" onclick="openUpdateModal()">
                <span>⬆ Update verfügbar</span>
            </div>
            <button class="docmost-nav-btn" onclick="openSettingsModal()" title="Einstellungen & Updates">⚙</button>
        </div>
    `;

    // Wait for body to append nav
    const appendNav = () => {{
        if (document.body && !document.getElementById('docmost-desktop-nav')) {{
            document.body.classList.add('has-desktop-nav');
            document.body.prepend(nav);
            createModals();
        }}
    }};

    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', appendNav);
    }} else {{
        appendNav();
    }}

    // Modals
    function createModals() {{
        if (document.getElementById('docmost-settings-modal')) return;

        const modalContainer = document.createElement('div');
        modalContainer.innerHTML = `
            <!-- Settings Modal -->
            <div id="docmost-settings-modal" class="docmost-modal-overlay">
                <div class="docmost-modal-box">
                    <div class="docmost-modal-header">
                        <h3 style="margin:0; font-size: 16px;">Docmost Desktop Einstellungen</h3>
                        <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="closeSettingsModal()">✕</span>
                    </div>
                    <div class="docmost-modal-body">
                        <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">Docmost Server URL</label>
                        <input id="input-server-url" type="text" value="{url}" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:16px;" />
                        
                        <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">GitHub Repository (für Updates)</label>
                        <input id="input-github-repo" type="text" value="LegoPuck-orginal/docmost-desktop" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:16px;" />
                        
                        <div style="margin-bottom:16px;">
                            <label style="display:flex; align-items:center; gap:8px; font-size:13px; cursor:pointer;">
                                <input id="check-auto-update" type="checkbox" checked />
                                <span>Automatisch beim Start nach Updates suchen</span>
                            </label>
                        </div>

                        <div style="background:#0e111a; padding:12px; border-radius:6px; border:1px solid #20273c; font-size:12px; color:#94a3b8;">
                            <div>Installierte Version: <strong style="color:#e2e8f0;">v{version}</strong></div>
                            <div id="update-status-text" style="margin-top:6px; color:#10b981;">✓ Status: Bereit</div>
                        </div>
                    </div>
                    <div class="docmost-modal-footer">
                        <button class="docmost-btn-secondary" onclick="testUpdateModalPreview()" style="font-size:11px;" title="Öffnet das Update-Fenster als Vorschau">Fenster-Vorschau testen</button>
                        <button class="docmost-btn-secondary" onclick="checkUpdatesManually()">Jetzt nach Updates suchen</button>
                        <button class="docmost-btn-primary" onclick="saveSettings()">Speichern & Neu laden</button>
                    </div>
                </div>
            </div>

            <!-- Update Available Modal -->
            <div id="docmost-update-modal" class="docmost-modal-overlay">
                <div class="docmost-modal-box">
                    <div class="docmost-modal-header" style="background: linear-gradient(135deg, #1e1b4b, #1e293b);">
                        <h3 style="margin:0; font-size: 16px; color:#a5b4fc;">🚀 Neues Update verfügbar!</h3>
                        <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="closeUpdateModal()">✕</span>
                    </div>
                    <div class="docmost-modal-body">
                        <div style="font-size:15px; font-weight:700; color:#f1f5f9; margin-bottom:4px;" id="update-modal-title">Docmost Desktop Update</div>
                        <div style="font-size:12px; color:#94a3b8; margin-bottom:12px;">Eine neue Version steht auf GitHub bereit und kann mit einem Klick installiert werden.</div>
                        <div id="update-modal-changelog" style="background:#0e111a; padding:12px; border-radius:6px; border:1px solid #20273c; font-size:12px; max-height:160px; overflow-y:auto; white-space:pre-wrap; color:#cbd5e1; font-family: monospace;"></div>
                        <div id="update-progress-box" style="display:none; margin-top:14px;">
                            <div style="font-size:12px; color:#818cf8; margin-bottom:6px; font-weight:600;">Lade Update herunter und starte Installer...</div>
                            <div style="background:#0e111a; height:8px; border-radius:4px; overflow:hidden;">
                                <div id="update-progress-bar" style="background:#6366f1; width:100%; height:100%; animation: pulseGlow 1.5s infinite;"></div>
                            </div>
                        </div>
                    </div>
                    <div class="docmost-modal-footer">
                        <button class="docmost-btn-secondary" onclick="closeUpdateModal()">Später</button>
                        <button id="btn-do-update" class="docmost-btn-primary" onclick="performUpdate()">Jetzt herunterladen & installieren</button>
                    </div>
                </div>
            </div>
        `;
        document.body.appendChild(modalContainer);

        // Falls bereits ein Update empfangen wurde, sofort anzeigen
        if (window.__pendingUpdateInfo) {{
            window.onUpdateAvailable(window.__pendingUpdateInfo);
        }}
        // Backend nach anstehendem Update abfragen
        if (window.ipc) {{
            window.ipc.postMessage(JSON.stringify({{ action: 'check_pending_update' }}));
        }}
    }}

    window.openSettingsModal = function() {{
        const el = document.getElementById('docmost-settings-modal');
        if (el) el.style.display = 'flex';
    }};
    window.closeSettingsModal = function() {{
        const el = document.getElementById('docmost-settings-modal');
        if (el) el.style.display = 'none';
    }};

    window.openUpdateModal = function() {{
        const el = document.getElementById('docmost-update-modal');
        if (el) el.style.display = 'flex';
    }};
    window.closeUpdateModal = function() {{
        const el = document.getElementById('docmost-update-modal');
        if (el) el.style.display = 'none';
    }};

    window.testUpdateModalPreview = function() {{
        window.closeSettingsModal();
        window.onUpdateAvailable({{
            version: 'v0.9.9',
            title: 'Docmost Desktop v0.9.9 (Vorschau)',
            changelog: '• Dies ist eine Vorschau des automatischen Update-Fensters!\n• Clean Dark Design aktiv.\n• Sobald eine neue Version auf GitHub existiert, öffnet sich dieses Fenster vollautomatisch beim Start.',
            download_url: ''
        }});
    }};

    window.saveSettings = function() {{
        const urlInput = document.getElementById('input-server-url');
        const repoInput = document.getElementById('input-github-repo');
        const autoUpInput = document.getElementById('check-auto-update');
        const newUrl = urlInput ? urlInput.value.trim() : '';
        const newRepo = repoInput ? repoInput.value.trim() : '';
        const autoUp = autoUpInput ? autoUpInput.checked : true;

        if (window.ipc) {{
            window.ipc.postMessage(JSON.stringify({{
                action: 'save_settings',
                server_url: newUrl,
                github_repo: newRepo,
                check_updates_on_startup: autoUp
            }}));
        }}
        closeSettingsModal();
        if (newUrl) {{
            window.location.href = newUrl;
        }}
    }};

    window.checkUpdatesManually = function() {{
        const statusText = document.getElementById('update-status-text');
        if (statusText) {{
            statusText.style.color = '#38bdf8';
            statusText.innerText = 'Prüfe GitHub auf Updates...';
        }}
        if (window.ipc) {{
            window.ipc.postMessage(JSON.stringify({{ action: 'check_updates' }}));
        }}
    }};

    window.onUpdateAvailable = function(info) {{
        if (!info) return;
        window.__pendingUpdateInfo = info;
        window.closeSettingsModal();

        const banner = document.getElementById('docmost-update-btn');
        if (banner) {{
            banner.style.display = 'flex';
            banner.innerHTML = '<span>⬆ Update verfügbar (' + (info.version || '') + ')</span>';
        }}
        const titleEl = document.getElementById('update-modal-title');
        if (titleEl) titleEl.innerText = info.title || ('Version ' + info.version);
        const clEl = document.getElementById('update-modal-changelog');
        if (clEl) clEl.innerText = info.changelog || 'Keine Versionshinweise vorhanden.';
        window.openUpdateModal();
    }};

    window.performUpdate = function() {{
        const info = window.__pendingUpdateInfo;
        if (!info || !info.download_url) {{
            alert('Vorschaumodus: Es ist keine Download-URL hinterlegt.');
            return;
        }}
        const pBox = document.getElementById('update-progress-box');
        if (pBox) pBox.style.display = 'block';
        const btn = document.getElementById('btn-do-update');
        if (btn) {{
            btn.disabled = true;
            btn.innerText = 'Wird heruntergeladen & installiert...';
        }}
        if (window.ipc) {{
            window.ipc.postMessage(JSON.stringify({{
                action: 'download_and_install_update',
                download_url: info.download_url
            }}));
        }}
    }};
}})();
    "#,
    css = CLEAN_DARK_GLOBAL_CSS,
    url = server_url,
    url_display = server_url.replace("https://", "").replace("http://", ""),
    version = current_version
    )
}

pub fn get_offline_or_setup_html(server_url: &str, current_version: &str, error_msg: Option<&str>) -> String {
    let err_block = if let Some(err) = error_msg {
        format!(r#"<div style="background: rgba(239,68,68,0.15); border: 1px solid #ef4444; color: #fca5a5; padding: 12px; border-radius: 8px; margin-bottom: 20px; font-size: 13px;"><strong>Verbindungsfehler:</strong> {}</div>"#, err)
    } else {
        String::new()
    };

    format!(r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Docmost Desktop</title>
    <style>
        {css}
        body {{
            margin: 0;
            padding: 0;
            background: #0d0f17;
            color: #f1f5f9;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
        }}
        .card {{
            background: #141824;
            border: 1px solid #232b3e;
            border-radius: 16px;
            padding: 40px;
            width: 500px;
            max-width: 90vw;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
            text-align: center;
        }}
        .logo {{
            width: 64px;
            height: 64px;
            background: linear-gradient(135deg, #6366f1, #a855f7);
            border-radius: 16px;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 32px;
            font-weight: 800;
            color: #fff;
            margin-bottom: 20px;
            box-shadow: 0 10px 25px rgba(99, 102, 241, 0.4);
        }}
        h1 {{
            font-size: 24px;
            font-weight: 700;
            margin: 0 0 8px 0;
            color: #fff;
        }}
        p.subtitle {{
            color: #94a3b8;
            font-size: 14px;
            margin: 0 0 28px 0;
        }}
        .input-group {{
            text-align: left;
            margin-bottom: 20px;
        }}
        label {{
            font-size: 12px;
            font-weight: 600;
            color: #cbd5e1;
            margin-bottom: 6px;
            display: block;
        }}
        input[type="text"] {{
            width: 100%;
            box-sizing: border-box;
            background: #0a0d14;
            border: 1px solid #28314a;
            border-radius: 8px;
            padding: 12px 14px;
            color: #fff;
            font-size: 14px;
            transition: border-color 0.2s;
        }}
        input[type="text"]:focus {{
            outline: none;
            border-color: #6366f1;
        }}
        .btn-connect {{
            width: 100%;
            background: #6366f1;
            color: #fff;
            border: none;
            border-radius: 8px;
            padding: 12px;
            font-size: 15px;
            font-weight: 600;
            cursor: pointer;
            transition: background 0.2s, transform 0.1s;
        }}
        .btn-connect:hover {{
            background: #4f46e5;
        }}
        .btn-connect:active {{
            transform: scale(0.99);
        }}
        .footer-info {{
            margin-top: 24px;
            font-size: 12px;
            color: #64748b;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .btn-link {{
            background: none;
            border: none;
            color: #818cf8;
            cursor: pointer;
            font-size: 12px;
            padding: 0;
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="card">
        <div class="logo">D</div>
        <h1>Docmost Desktop</h1>
        <p class="subtitle">Clean Dark Edition &bull; Rust &amp; WebView2</p>

        {error_html}

        <div class="input-group">
            <label for="server-url">Docmost Instanz URL</label>
            <input id="server-url" type="text" value="{url}" placeholder="https://doc.home.klehausen.de" />
        </div>

        <button class="btn-connect" onclick="connect()">Verbinden &amp; Öffnen</button>

        <div class="footer-info">
            <span>Version: v{version}</span>
            <button class="btn-link" onclick="checkUpdates()">Nach Updates suchen</button>
        </div>
    </div>

    <script>
        function connect() {{
            const url = document.getElementById('server-url').value.trim();
            if (url) {{
                if (window.ipc) {{
                    window.ipc.postMessage(JSON.stringify({{
                        action: 'save_settings',
                        server_url: url
                    }}));
                }}
                window.location.href = url;
            }}
        }}

        function checkUpdates() {{
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{ action: 'check_updates' }}));
            }}
        }}
    </script>
</body>
</html>"#,
    css = CLEAN_DARK_GLOBAL_CSS,
    error_html = err_block,
    url = server_url,
    version = current_version
    )
}
