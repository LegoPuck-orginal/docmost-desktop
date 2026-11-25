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

/* Floating 3-Line Hamburger Menu Button (Top Right, Docmost Mantine Style) */
#docmost-app-menu-btn {
    position: fixed;
    top: 8px;
    right: 12px;
    width: 32px;
    height: 32px;
    background: rgba(31, 31, 31, 0.88);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    border: 1px solid #383838;
    border-radius: 6px;
    color: #c9c9c9;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2147483645 !important;
    transition: background-color 0.12s ease, color 0.12s ease, border-color 0.12s ease;
    user-select: none;
    padding: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.35);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}
#docmost-app-menu-btn:hover,
#docmost-app-menu-btn.active {
    background: #2e2e2e;
    border-color: #4a4a4a;
    color: #ffffff;
}
#docmost-menu-update-dot {
    position: absolute;
    top: 4px;
    right: 4px;
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #228be6;
    box-shadow: 0 0 6px rgba(34, 139, 230, 0.9);
    display: none;
}

/* Reserve space on the right of Docmost's header so the 3-line button never overlaps controls */
body.has-desktop-menu header.mantine-AppShell-header,
body.has-desktop-menu header [class*="_header_"] {
    padding-right: 54px !important;
}

.docmost-dropdown-nav-row {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 4px;
    padding: 4px 6px 6px 6px;
}
.docmost-dropdown-nav-btn {
    height: 30px;
    background: #1f1f1f;
    border: 1px solid #333333;
    border-radius: 5px;
    color: #c9c9c9;
    cursor: pointer;
    font-size: 13px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.12s ease, color 0.12s ease, border-color 0.12s ease;
    user-select: none;
}
.docmost-dropdown-nav-btn:hover {
    background: #2e2e2e;
    border-color: #484848;
    color: #ffffff;
}

.docmost-status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #12b886;
    box-shadow: 0 0 6px rgba(18, 184, 134, 0.6);
    flex-shrink: 0;
}

/* Mantine Style Dropdown Menus */
.docmost-dropdown-menu {
    position: fixed;
    top: 46px;
    right: 12px;
    background: #242424;
    border: 1px solid #383838;
    border-radius: 8px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7);
    padding: 6px;
    z-index: 2147483647 !important;
    display: none;
    flex-direction: column;
    min-width: 270px;
    max-width: 340px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    animation: mantineFadeIn 0.12s cubic-bezier(0.16, 1, 0.3, 1);
}
@keyframes mantineFadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
}

.docmost-dropdown-label {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    color: #828282;
    padding: 6px 10px 4px 10px;
    letter-spacing: 0.5px;
    user-select: none;
}

.docmost-dropdown-divider {
    height: 1px;
    background: #333333;
    margin: 4px 0;
}

.docmost-dropdown-item {
    padding: 7px 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    color: #c9c9c9;
    font-size: 13px;
    font-weight: 500;
    border-radius: 4px;
    cursor: pointer;
    text-decoration: none;
    user-select: none;
    transition: background-color 0.12s ease, color 0.12s ease;
}
.docmost-dropdown-item:hover {
    background-color: #2e2e2e;
    color: #ffffff;
}
.docmost-dropdown-item.active {
    background-color: rgba(34, 139, 230, 0.15);
    color: #339af0;
    font-weight: 600;
}

.docmost-dropdown-item-left {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
}
.docmost-dropdown-shortcut {
    font-size: 11px;
    color: #666;
    margin-left: auto;
    font-family: monospace;
}

.docmost-dropdown-list {
    max-height: 200px;
    overflow-y: auto;
}
.docmost-server-del-btn {
    background: transparent;
    border: none;
    color: #666;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 11px;
    transition: all 0.12s ease;
}
.docmost-server-del-btn:hover {
    background: rgba(239, 68, 68, 0.2);
    color: #ff6b6b;
}
.docmost-add-server-btn {
    background: #228be6;
    border: none;
    border-radius: 4px;
    color: #fff;
    font-size: 11px;
    font-weight: 600;
    padding: 3px 8px;
    cursor: pointer;
    transition: background-color 0.12s ease;
}
.docmost-add-server-btn:hover {
    background: #1c7ed6;
}

/* Update Pill inside Menu */
.docmost-update-banner {
    background: rgba(34, 139, 230, 0.18);
    border: 1px solid #228be6;
    color: #74c0fc;
    font-weight: 600;
    border-radius: 5px;
    padding: 7px 10px;
    margin: 4px 0;
    font-size: 12px;
    cursor: pointer;
    display: none;
    align-items: center;
    justify-content: center;
    gap: 6px;
    transition: all 0.15s ease;
}
.docmost-update-banner:hover {
    background: #228be6;
    color: #ffffff;
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
    z-index: 2147483647 !important;
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

/* Touch-Action & Pen Pointer Optimization */
.excalidraw,
.excalidraw canvas,
.excalidraw .interactive,
.excalidraw-container,
[data-testid="excalidraw-container"] {
    touch-action: none !important;
    -webkit-touch-callout: none !important;
    user-select: none !important;
    -webkit-user-select: none !important;
}

/* Settings Tabs */
.docmost-tabs-header {
    display: flex;
    border-bottom: 1px solid #20273c;
    padding: 0 16px;
    background: #0f121d;
    gap: 4px;
}
.docmost-tab-btn {
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: #94a3b8;
    font-size: 13px;
    font-weight: 500;
    padding: 10px 14px;
    cursor: pointer;
    transition: all 0.15s ease;
}
.docmost-tab-btn:hover {
    color: #f1f5f9;
}
.docmost-tab-btn.active {
    color: #818cf8;
    border-bottom-color: #6366f1;
    font-weight: 600;
}
.docmost-tab-pane {
    display: none;
}
.docmost-tab-pane.active {
    display: block;
}

/* Floating Stylus Quick Toolbar */
#docmost-pen-toolbar {
    position: fixed;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(18, 22, 34, 0.95);
    backdrop-filter: blur(18px);
    -webkit-backdrop-filter: blur(18px);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 40px;
    padding: 6px 14px;
    box-shadow: 0 16px 36px rgba(0, 0, 0, 0.65);
    z-index: 9999999;
    display: none;
    align-items: center;
    gap: 8px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    user-select: none;
    animation: fadeInToolbar 0.2s ease-out;
}
@keyframes fadeInToolbar {
    from { opacity: 0; transform: translate(-50%, 10px); }
    to { opacity: 1; transform: translate(-50%, 0); }
}
.docmost-pen-btn {
    background: transparent;
    border: 1px solid transparent;
    color: #cbd5e1;
    border-radius: 20px;
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 5px;
    transition: all 0.15s ease;
}
.docmost-pen-btn:hover {
    background: #1e2538;
    color: #fff;
    border-color: #2f3b58;
}
.docmost-pen-btn.active {
    background: #6366f1;
    color: #fff;
    border-color: #818cf8;
    box-shadow: 0 0 10px rgba(99, 102, 241, 0.4);
}
.docmost-color-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid rgba(255,255,255,0.2);
    cursor: pointer;
    transition: transform 0.1s ease, border-color 0.1s ease;
    padding: 0;
}
.docmost-color-dot:hover {
    transform: scale(1.25);
    border-color: #fff;
}
.docmost-pen-stroke-btn {
    background: transparent;
    border: 1px solid transparent;
    color: #94a3b8;
    border-radius: 6px;
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.15s ease;
}
.docmost-pen-stroke-btn:hover {
    background: #1e2538;
    color: #fff;
}
.docmost-pen-stroke-btn.active {
    background: #252e46;
    color: #818cf8;
    border-color: #6366f1;
}

/* Media Player Modal */
#docmost-media-modal .docmost-modal-box {
    width: 860px;
    max-width: 95vw;
    background: #0d101a;
    border: 1px solid #28314a;
    border-radius: 14px;
    box-shadow: 0 25px 60px rgba(0,0,0,0.9);
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
.docmost-media-content {
    background: #070911;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 260px;
    max-height: 72vh;
    position: relative;
    padding: 8px;
}
#docmost-media-video {
    max-width: 100%;
    max-height: 70vh;
    border-radius: 8px;
    outline: none;
    background: #000;
}
.docmost-audio-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 30px;
    width: 100%;
    gap: 16px;
}
.docmost-audio-visual {
    width: 68px;
    height: 68px;
    border-radius: 50%;
    background: linear-gradient(135deg, #6366f1, #a855f7);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 30px;
    box-shadow: 0 0 24px rgba(99, 102, 241, 0.5);
    animation: pulseGlow 2s infinite;
}
#docmost-media-audio {
    width: 85%;
    max-width: 550px;
}
.docmost-badge-format {
    background: #1e2538;
    color: #818cf8;
    border: 1px solid #2f3956;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
    padding: 2px 8px;
    text-transform: uppercase;
}

/* Office & Document Viewer Modal */
#docmost-office-modal .docmost-modal-box {
    width: 1040px;
    max-width: 96vw;
    height: 88vh;
    background: #0e111c;
    border: 1px solid #28314a;
    border-radius: 14px;
    box-shadow: 0 25px 65px rgba(0,0,0,0.9);
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
.docmost-office-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #080a13;
    position: relative;
    overflow: hidden;
}
#docmost-office-iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: #111422;
}
.docmost-office-fallback-card {
    margin: auto;
    max-width: 560px;
    padding: 36px 30px;
    background: #121626;
    border: 1px solid #28314a;
    border-radius: 12px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    box-shadow: 0 12px 30px rgba(0,0,0,0.5);
}
"#;

pub fn get_injection_script(config: &crate::config::AppConfig, current_version: &str) -> String {
    let servers_json = serde_json::to_string(&config.servers).unwrap_or_else(|_| "[]".to_string());
    let active_id = config.active_server_id.as_deref().unwrap_or("default");
    let active_name = config.active_server().map(|s| s.name.as_str()).unwrap_or("Server");

    format!(r#"
(function() {{
    try {{
        if (window.top !== window.self) return;
        if (window.__docmostDesktopInitialized) return;
        window.__docmostDesktopInitialized = true;

        // Diagnostic IPC forwarder
        function logToRust(level, msg) {{
            try {{
                if (window.ipc) {{
                    window.ipc.postMessage(JSON.stringify({{
                        action: 'console_log',
                        level: level,
                        message: String(msg)
                    }}));
                }}
            }} catch (e) {{}}
        }}

        logToRust('info', 'Docmost Desktop Script initialized at: ' + window.location.href);

        window.addEventListener('error', function(e) {{
            logToRust('error', (e.message || 'Error') + ' at ' + (e.filename || '') + ':' + (e.lineno || ''));
        }});

        window.addEventListener('unhandledrejection', function(e) {{
            logToRust('unhandledrejection', e.reason ? (e.reason.stack || e.reason.message || String(e.reason)) : 'Promise rejected');
        }});

        // Config State
        window.__docmostConfig = {{
            serverUrl: "{url}",
            githubRepo: "{repo}",
            autoUpdate: {auto_update},
            penMode: {pen_mode},
            palmRejection: {palm_rejection},
            darkTheme: {dark_theme},
            quickToolbar: {quick_toolbar},
            zoomLevel: {zoom_level},
            version: "{version}",
            servers: {servers_json},
            activeServerId: "{active_server_id}",
            onlyofficeUrl: "{onlyoffice_url}",
            mediaPlayerEnabled: {media_player_enabled},
            officeViewerEnabled: {office_viewer_enabled}
        }};

        // Safe CSS Injection
        const injectStyles = () => {{
            if (document.getElementById('docmost-desktop-style')) return;
            const target = document.head || document.documentElement;
            if (!target) return;
            const style = document.createElement('style');
            style.id = 'docmost-desktop-style';
            style.textContent = `{css}`;
            target.appendChild(style);
        }};

        // Safe Zoom
        const applyZoom = () => {{
            if (document.documentElement && window.__docmostConfig.zoomLevel && window.__docmostConfig.zoomLevel !== 1.0) {{
                document.documentElement.style.zoom = window.__docmostConfig.zoomLevel;
            }}
        }};

        // Auto-sync Excalidraw Dark Mode & Pen Mode (runs only when Excalidraw is present)
        const syncExcalidrawState = () => {{
            const hasExcalidraw = !!document.querySelector('.excalidraw, [data-testid="excalidraw-container"]');
            if (!hasExcalidraw) return;
            try {{
                const rawState = localStorage.getItem('excalidraw-state');
                let state = rawState ? JSON.parse(rawState) : {{}};
                if (window.__docmostConfig.darkTheme) {{
                    state.theme = 'dark';
                    localStorage.setItem('excalidraw-theme', 'dark');
                }}
                if (window.__docmostConfig.penMode) {{
                    state.penMode = true;
                    state.penDetected = true;
                }}
                localStorage.setItem('excalidraw-state', JSON.stringify(state));
            }} catch (e) {{}}
        }};

        const ensureExcalidrawDark = () => {{
            if (window.__docmostConfig.darkTheme) {{
                document.querySelectorAll('.excalidraw').forEach(el => {{
                    if (!el.classList.contains('theme--dark')) {{
                        el.classList.add('theme--dark');
                    }}
                }});
            }}
        }};

        const checkExcalidrawPresence = () => {{
            ensureExcalidrawDark();
            const hasExcalidraw = !!document.querySelector('.excalidraw, [data-testid="excalidraw-container"]');
            const penToolbar = document.getElementById('docmost-pen-toolbar');
            if (penToolbar) {{
                if (hasExcalidraw && window.__docmostConfig.quickToolbar) {{
                    penToolbar.style.display = 'flex';
                }} else {{
                    penToolbar.style.display = 'none';
                }}
            }}
        }};

        // Stylus / Palm Rejection Logic
        let lastPenContactTime = 0;
        window.addEventListener('pointerdown', (e) => {{
            if (e.pointerType === 'pen') {{
                lastPenContactTime = Date.now();
                const badge = document.getElementById('badge-palm-rejection');
                if (badge) badge.style.opacity = '1';
            }} else if (e.pointerType === 'touch' && window.__docmostConfig.palmRejection) {{
                const onCanvas = e.target.closest('.excalidraw canvas') || e.target.closest('.excalidraw');
                if (onCanvas && (Date.now() - lastPenContactTime < 1200 || window.__docmostConfig.penMode)) {{
                    if (e.isPrimary && !e.shiftKey) {{
                        e.stopPropagation();
                    }}
                }}
            }}
        }}, true);

        window.addEventListener('pointerup', (e) => {{
            if (e.pointerType === 'pen') {{
                lastPenContactTime = Date.now();
            }}
        }}, true);

        window.addEventListener('contextmenu', (e) => {{
            if (e.target.closest('.excalidraw canvas') || e.target.closest('.excalidraw')) {{
                e.preventDefault();
            }}
        }}, true);

        // Pen Tools Actions
        window.activatePenTool = function(type) {{
            const btnPen = document.getElementById('tb-btn-pen');
            const btnEraser = document.getElementById('tb-btn-eraser');
            if (type === 'freedraw') {{
                if (btnPen) btnPen.classList.add('active');
                if (btnEraser) btnEraser.classList.remove('active');
                const el = document.querySelector('input[value="freedraw"]') || 
                           document.querySelector('[data-testid="freedraw"]') ||
                           document.querySelector('button[aria-label*="Draw"]') ||
                           document.querySelector('button[aria-label*="Freihand"]');
                if (el) {{ el.click(); }}
                else {{ document.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'p', code: 'KeyP', bubbles: true }})); }}
            }} else if (type === 'eraser') {{
                if (btnEraser) btnEraser.classList.add('active');
                if (btnPen) btnPen.classList.remove('active');
                const el = document.querySelector('input[value="eraser"]') || 
                           document.querySelector('[data-testid="eraser"]') ||
                           document.querySelector('button[aria-label*="Eraser"]') ||
                           document.querySelector('button[aria-label*="Radierer"]');
                if (el) {{ el.click(); }}
                else {{ document.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'e', code: 'KeyE', bubbles: true }})); }}
            }}
        }};

        window.setPenColor = function(hex) {{
            const swatches = document.querySelectorAll('.color-picker-swatch, .color-picker input, [data-testid="color-picker"] button');
            for (const s of swatches) {{
                if (s.title && s.title.toLowerCase().includes(hex.toLowerCase())) {{
                    s.click();
                    return;
                }}
            }}
            try {{
                const raw = localStorage.getItem('excalidraw-state');
                if (raw) {{
                    const st = JSON.parse(raw);
                    st.currentItemStrokeColor = hex;
                    localStorage.setItem('excalidraw-state', JSON.stringify(st));
                }}
            }} catch(e) {{}}
        }};

        window.setPenStroke = function(width) {{
            document.querySelectorAll('.docmost-pen-stroke-btn').forEach(b => b.classList.remove('active'));
            if (window.event && window.event.currentTarget) window.event.currentTarget.classList.add('active');
            try {{
                const raw = localStorage.getItem('excalidraw-state');
                if (raw) {{
                    const st = JSON.parse(raw);
                    st.currentItemStrokeWidth = width;
                    localStorage.setItem('excalidraw-state', JSON.stringify(st));
                }}
            }} catch(e) {{}}
        }};

        window.penUndo = function() {{
            document.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'z', code: 'KeyZ', ctrlKey: true, bubbles: true }}));
        }};
        window.penRedo = function() {{
            document.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'y', code: 'KeyY', ctrlKey: true, bubbles: true }}));
        }};

        // Dropdown Menu Management (Top-Right 3-Line Menu)
        window.closeAllDropdowns = function() {{
            const appMenu = document.getElementById('docmost-app-dropdown-menu');
            if (appMenu) appMenu.style.display = 'none';
            const menuBtn = document.getElementById('docmost-app-menu-btn');
            if (menuBtn) menuBtn.classList.remove('active');
        }};

        window.renderAppMenuServers = function() {{
            const container = document.getElementById('docmost-app-menu-servers-list');
            if (!container) return;
            const servers = window.__docmostConfig.servers || [];
            const activeId = window.__docmostConfig.activeServerId;
            let html = '';
            servers.forEach(s => {{
                const isActive = s.id === activeId;
                const displayUrl = s.url.replace('https://', '').replace('http://', '');
                html += `
                    <div class="docmost-dropdown-item ${{isActive ? 'active' : ''}}" onclick="window.selectServer('${{s.id}}', '${{s.url}}')">
                        <div class="docmost-dropdown-item-left" style="flex:1; min-width:0;">
                            <span class="docmost-status-dot" style="${{isActive ? '' : 'background:#666; box-shadow:none;'}}"></span>
                            <div style="overflow:hidden;">
                                <div style="font-size:13px; font-weight:600; color:${{isActive ? '#ffffff' : '#c9c9c9'}}; overflow:hidden; text-overflow:ellipsis;">${{s.name}} ${{isActive ? '<span style="font-size:10px; color:#339af0; font-weight:normal;">(Aktiv)</span>' : ''}}</div>
                                <div style="font-size:11px; color:#828282; overflow:hidden; text-overflow:ellipsis;">${{displayUrl}}</div>
                            </div>
                        </div>
                        ${{servers.length > 1 ? `<button class="docmost-server-del-btn" onclick="window.deleteServer(event, '${{s.id}}')" title="Server entfernen">✕</button>` : ''}}
                    </div>
                `;
            }});
            container.innerHTML = html;
        }};

        window.toggleAppMenu = function(e) {{
            if (e) e.stopPropagation();
            const menu = document.getElementById('docmost-app-dropdown-menu');
            const menuBtn = document.getElementById('docmost-app-menu-btn');
            if (!menu) return;

            if (menu.style.display === 'flex') {{
                menu.style.display = 'none';
                if (menuBtn) menuBtn.classList.remove('active');
            }} else {{
                window.renderAppMenuServers();
                if (menuBtn) {{
                    const rect = menuBtn.getBoundingClientRect();
                    menu.style.top = (rect.bottom + 6) + 'px';
                    menu.style.right = Math.max(8, window.innerWidth - rect.right) + 'px';
                    menu.style.left = 'auto';
                    menuBtn.classList.add('active');
                }}
                menu.style.display = 'flex';
            }}
        }};

        window.renderSettingsServerList = function() {{
            const container = document.getElementById('settings-server-list');
            if (!container) return;
            const servers = window.__docmostConfig.servers || [];
            const activeId = window.__docmostConfig.activeServerId;

            let html = '';
            servers.forEach(s => {{
                const isActive = s.id === activeId;
                const displayUrl = s.url.replace('https://', '').replace('http://', '');
                html += `
                    <div style="padding:10px 12px; display:flex; align-items:center; justify-content:space-between; border-bottom:1px solid #2e2e2e; ${{isActive ? 'background:rgba(34,139,230,0.15); border-left:3px solid #228be6;' : ''}}">
                        <div style="display:flex; align-items:center; gap:8px;">
                            <span class="docmost-status-dot" style="${{isActive ? '' : 'background:#666; box-shadow:none;'}}"></span>
                            <div>
                                <div style="font-size:13px; font-weight:600; color:#f1f5f9;">${{s.name}} ${{isActive ? '<span style="font-size:11px; color:#12b886; font-weight:normal;">(Aktiv)</span>' : ''}}</div>
                                <div style="font-size:11px; color:#828282;">${{displayUrl}}</div>
                            </div>
                        </div>
                        <div style="display:flex; gap:6px; align-items:center;">
                            ${{!isActive ? `<button class="docmost-btn-secondary" style="padding:4px 10px; font-size:11px;" onclick="window.selectServer('${{s.id}}', '${{s.url}}')">Wechseln</button>` : ''}}
                            ${{servers.length > 1 ? `<button class="docmost-server-del-btn" onclick="window.deleteServer(event, '${{s.id}}')" title="Server entfernen">✕</button>` : ''}}
                        </div>
                    </div>
                `;
            }});
            container.innerHTML = html;
        }};

        window.addEventListener('click', (e) => {{
            if (!e.target.closest('#docmost-app-dropdown-menu') &&
                !e.target.closest('#docmost-app-menu-btn')) {{
                window.closeAllDropdowns();
            }}
        }});

        window.selectServer = function(id, url) {{
            window.closeAllDropdowns();
            if (id === window.__docmostConfig.activeServerId && url === window.location.href) {{
                return;
            }}
            window.__docmostConfig.activeServerId = id;
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'switch_server',
                    server_id: id
                }}));
            }}
            window.closeSettingsModal();
            window.location.href = url;
        }};

        window.openAddServerModal = function(e) {{
            if (e) e.stopPropagation();
            window.closeAllDropdowns();
            const modal = document.getElementById('docmost-add-server-modal');
            if (modal) modal.style.display = 'flex';
        }};

        window.closeAddServerModal = function() {{
            const modal = document.getElementById('docmost-add-server-modal');
            if (modal) modal.style.display = 'none';
        }};

        window.submitAddServer = function() {{
            const nameInput = document.getElementById('input-new-server-name');
            const urlInput = document.getElementById('input-new-server-url');
            const name = nameInput ? nameInput.value.trim() : '';
            let url = urlInput ? urlInput.value.trim() : '';
            if (!url) {{
                alert('Bitte eine gültige Server-URL eingeben.');
                return;
            }}
            if (!url.startsWith('http://') && !url.startsWith('https://')) {{
                url = 'https://' + url;
            }}
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'add_server',
                    server_name: name || url,
                    server_url: url
                }}));
            }}
            window.closeAddServerModal();
            window.location.href = url;
        }};

        window.deleteServer = function(e, id) {{
            if (e) e.stopPropagation();
            if (!confirm('Diesen Server wirklich aus der Liste entfernen?')) return;
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'delete_server',
                    server_id: id
                }}));
            }}
            window.__docmostConfig.servers = (window.__docmostConfig.servers || []).filter(s => s.id !== id);
            window.renderAppMenuServers();
            window.renderSettingsServerList();
        }};

        window.switchSettingsTab = function(tabName) {{
            document.querySelectorAll('.docmost-tab-btn').forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.docmost-tab-pane').forEach(p => p.classList.remove('active'));

            const btn = document.getElementById('tab-btn-' + tabName);
            const pane = document.getElementById('pane-' + tabName);
            if (btn) btn.classList.add('active');
            if (pane) pane.classList.add('active');
            if (tabName === 'general') {{
                window.renderSettingsServerList();
            }}
        }};

        window.openSettingsModal = function() {{
            window.renderSettingsServerList();
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
                changelog: '• Dies ist eine Vorschau des automatischen Update-Fensters!\n• Clean Dark Design & Stift-Unterstützung aktiv.\n• Sobald eine neue Version auf GitHub existiert, öffnet sich dieses Fenster vollautomatisch beim Start.',
                download_url: ''
            }});
        }};

        window.saveSettings = function() {{
            const repoInput = document.getElementById('input-github-repo');
            const autoUpInput = document.getElementById('check-auto-update');
            const penModeInput = document.getElementById('check-pen-mode');
            const palmRejInput = document.getElementById('check-palm-rejection');
            const quickTbInput = document.getElementById('check-quick-toolbar');
            const darkThemeInput = document.getElementById('check-excalidraw-dark');
            const zoomSelect = document.getElementById('select-ui-zoom');
            const ooInput = document.getElementById('input-onlyoffice-url');
            const mpInput = document.getElementById('check-media-player');
            const ovInput = document.getElementById('check-office-viewer');

            const newRepo = repoInput ? repoInput.value.trim() : '';
            const autoUp = autoUpInput ? autoUpInput.checked : true;
            const penMode = penModeInput ? penModeInput.checked : true;
            const palmRejection = palmRejInput ? palmRejInput.checked : true;
            const quickToolbar = quickTbInput ? quickTbInput.checked : true;
            const darkTheme = darkThemeInput ? darkThemeInput.checked : true;
            const zoomLevel = zoomSelect ? parseFloat(zoomSelect.value) : 1.0;
            const onlyofficeUrl = ooInput ? ooInput.value.trim() : '';
            const mediaPlayerEnabled = mpInput ? mpInput.checked : true;
            const officeViewerEnabled = ovInput ? ovInput.checked : true;

            window.__docmostConfig.githubRepo = newRepo;
            window.__docmostConfig.autoUpdate = autoUp;
            window.__docmostConfig.penMode = penMode;
            window.__docmostConfig.palmRejection = palmRejection;
            window.__docmostConfig.quickToolbar = quickToolbar;
            window.__docmostConfig.darkTheme = darkTheme;
            window.__docmostConfig.zoomLevel = zoomLevel;
            window.__docmostConfig.onlyofficeUrl = onlyofficeUrl;
            window.__docmostConfig.mediaPlayerEnabled = mediaPlayerEnabled;
            window.__docmostConfig.officeViewerEnabled = officeViewerEnabled;

            if (document.documentElement) {{
                document.documentElement.style.zoom = zoomLevel;
            }}
            checkExcalidrawPresence();
            syncExcalidrawState();

            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'save_settings',
                    github_repo: newRepo,
                    check_updates_on_startup: autoUp,
                    excalidraw_pen_mode: penMode,
                    excalidraw_palm_rejection: palmRejection,
                    excalidraw_quick_toolbar: quickToolbar,
                    excalidraw_dark_theme: darkTheme,
                    ui_zoom_level: zoomLevel,
                    onlyoffice_url: onlyofficeUrl,
                    media_player_enabled: mediaPlayerEnabled,
                    office_viewer_enabled: officeViewerEnabled
                }}));
            }}
            window.closeSettingsModal();
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
            const dot = document.getElementById('docmost-menu-update-dot');
            if (dot) dot.style.display = 'block';
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

        // Media Player Logic
        let currentMediaUrl = '';
        let currentMediaTitle = '';

        window.openMediaPlayer = function(url, title) {{
            currentMediaUrl = url;
            currentMediaTitle = title || url.split('/').pop().split('?')[0] || 'Mediendatei';

            const modal = document.getElementById('docmost-media-modal');
            const titleEl = document.getElementById('media-modal-title');
            const badgeEl = document.getElementById('media-modal-badge');
            const videoEl = document.getElementById('docmost-media-video');
            const audioEl = document.getElementById('docmost-media-audio');
            const audioView = document.getElementById('docmost-audio-view');
            const videoContainer = document.getElementById('docmost-video-container');

            if (!modal) return;
            if (titleEl) titleEl.innerText = currentMediaTitle;

            const ext = (currentMediaTitle.split('.').pop() || '').toLowerCase();
            if (badgeEl) badgeEl.innerText = ext ? ext.toUpperCase() : 'MEDIA';

            const isAudio = ['mp3', 'wav', 'ogg', 'm4a', 'aac', 'flac'].includes(ext);

            if (isAudio) {{
                if (videoContainer) videoContainer.style.display = 'none';
                if (videoEl) {{ videoEl.pause(); videoEl.src = ''; }}
                if (audioView) audioView.style.display = 'flex';
                if (audioEl) {{
                    audioEl.src = url;
                    audioEl.play().catch(() => {{}});
                }}
            }} else {{
                if (audioView) audioView.style.display = 'none';
                if (audioEl) {{ audioEl.pause(); audioEl.src = ''; }}
                if (videoContainer) videoContainer.style.display = 'flex';
                if (videoEl) {{
                    videoEl.src = url;
                    videoEl.play().catch(() => {{}});
                }}
            }}
            modal.style.display = 'flex';
        }};

        window.closeMediaPlayer = function() {{
            const modal = document.getElementById('docmost-media-modal');
            const videoEl = document.getElementById('docmost-media-video');
            const audioEl = document.getElementById('docmost-media-audio');
            if (videoEl) {{ videoEl.pause(); videoEl.src = ''; }}
            if (audioEl) {{ audioEl.pause(); audioEl.src = ''; }}
            if (modal) modal.style.display = 'none';
        }};

        window.openCurrentMediaExternal = function() {{
            if (currentMediaUrl) {{
                window.downloadAndOpenExternal(currentMediaUrl, currentMediaTitle);
            }}
        }};

        // Office & Document Logic
        let currentOfficeUrl = '';
        let currentOfficeTitle = '';

        window.openOfficeModal = function(url, title) {{
            currentOfficeUrl = url;
            currentOfficeTitle = title || url.split('/').pop().split('?')[0] || 'Dokument';

            const modal = document.getElementById('docmost-office-modal');
            const titleEl = document.getElementById('office-modal-title');
            const badgeEl = document.getElementById('office-modal-badge');
            const iframe = document.getElementById('docmost-office-iframe');
            const fallbackCard = document.getElementById('docmost-office-fallback-card');
            const fallbackIcon = document.getElementById('office-fallback-icon');
            const fallbackName = document.getElementById('office-fallback-filename');

            if (!modal) return;
            if (titleEl) titleEl.innerText = currentOfficeTitle;
            if (fallbackName) fallbackName.innerText = currentOfficeTitle;

            const ext = (currentOfficeTitle.split('.').pop() || '').toLowerCase();
            if (badgeEl) badgeEl.innerText = ext ? ext.toUpperCase() : 'DOC';

            if (fallbackIcon) {{
                if (['docx', 'doc', 'odt'].includes(ext)) {{
                    fallbackIcon.innerText = '📝';
                }} else if (['pptx', 'ppt', 'odp'].includes(ext)) {{
                    fallbackIcon.innerText = '📊';
                }} else if (['xlsx', 'xls', 'ods', 'csv'].includes(ext)) {{
                    fallbackIcon.innerText = '📈';
                }} else if (ext === 'pdf') {{
                    fallbackIcon.innerText = '📄';
                }} else {{
                    fallbackIcon.innerText = '📁';
                }}
            }}

            const onlyofficeConfigUrl = (window.__docmostConfig.onlyofficeUrl || '').trim();

            if (ext === 'pdf') {{
                if (fallbackCard) fallbackCard.style.display = 'none';
                if (iframe) {{
                    iframe.style.display = 'block';
                    iframe.src = url;
                }}
            }} else if (onlyofficeConfigUrl) {{
                let ooEmbed = onlyofficeConfigUrl;
                if (!ooEmbed.endsWith('/')) ooEmbed += '/';
                const fullFileUrl = url.startsWith('http') ? url : (window.location.origin + (url.startsWith('/') ? '' : '/') + url);
                const ooViewerUrl = ooEmbed + 'web-apps/apps/documenteditor/main/index.html?mode=view&url=' + encodeURIComponent(fullFileUrl);
                if (fallbackCard) fallbackCard.style.display = 'none';
                if (iframe) {{
                    iframe.style.display = 'block';
                    iframe.src = ooViewerUrl;
                }}
            }} else {{
                if (iframe) {{
                    iframe.style.display = 'none';
                    iframe.src = 'about:blank';
                }}
                if (fallbackCard) fallbackCard.style.display = 'flex';
            }}

            modal.style.display = 'flex';
        }};

        window.closeOfficeModal = function() {{
            const modal = document.getElementById('docmost-office-modal');
            const iframe = document.getElementById('docmost-office-iframe');
            if (iframe) iframe.src = 'about:blank';
            if (modal) modal.style.display = 'none';
        }};

        window.openCurrentOfficeExternal = function() {{
            if (currentOfficeUrl) {{
                window.downloadAndOpenExternal(currentOfficeUrl, currentOfficeTitle);
            }}
        }};

        window.toggleOfficeOnlineViewer = function() {{
            const iframe = document.getElementById('docmost-office-iframe');
            const fallbackCard = document.getElementById('docmost-office-fallback-card');
            if (!iframe) return;

            const fullFileUrl = currentOfficeUrl.startsWith('http') ? currentOfficeUrl : (window.location.origin + (currentOfficeUrl.startsWith('/') ? '' : '/') + currentOfficeUrl);
            const msOnlineUrl = 'https://view.officeapps.live.com/op/embed.aspx?src=' + encodeURIComponent(fullFileUrl);

            if (fallbackCard) fallbackCard.style.display = 'none';
            iframe.style.display = 'block';
            iframe.src = msOnlineUrl;
        }};

        // Helper to download authenticated file and trigger native local application
        window.downloadAndOpenExternal = async function(url, filename) {{
            const btn = document.activeElement;
            const originalText = btn ? btn.innerText : '';
            if (btn && btn.tagName === 'BUTTON') {{
                btn.disabled = true;
                btn.innerText = '⏳ Öffne in lokaler App...';
            }}
            try {{
                const res = await fetch(url, {{ credentials: 'include' }});
                if (!res.ok) throw new Error('Download-Status: ' + res.status);
                const blob = await res.blob();
                const reader = new FileReader();
                reader.onloadend = function() {{
                    const base64data = reader.result.split(',')[1];
                    if (window.ipc) {{
                        window.ipc.postMessage(JSON.stringify({{
                            action: 'open_temp_file',
                            filename: filename,
                            file_base64: base64data
                        }}));
                    }}
                    setTimeout(() => {{
                        if (btn && btn.tagName === 'BUTTON') {{
                            btn.disabled = false;
                            btn.innerText = originalText;
                        }}
                    }}, 1200);
                }};
                reader.readAsDataURL(blob);
            }} catch (err) {{
                console.warn('[Docmost Desktop] Fetch failed, falling back to direct URL launch:', err);
                if (window.ipc) {{
                    window.ipc.postMessage(JSON.stringify({{
                        action: 'open_file_external',
                        file_url: url.startsWith('http') ? url : (window.location.origin + (url.startsWith('/') ? '' : '/') + url)
                    }}));
                }}
                if (btn && btn.tagName === 'BUTTON') {{
                    btn.disabled = false;
                    btn.innerText = originalText;
                }}
            }}
        }};

        // Global Click Interceptor for Media and Office Attachments
        document.addEventListener('click', (e) => {{
            const link = e.target.closest('a');
            if (!link) return;
            const href = link.getAttribute('href') || '';
            const download = link.getAttribute('download') || '';
            const text = (link.innerText || '').trim();

            if (!href || href.startsWith('javascript:') || href.startsWith('#')) return;

            const cleanHref = href.split('?')[0].split('#')[0].toLowerCase();
            const linkTextLower = text.toLowerCase();
            const downloadLower = download.toLowerCase();

            const mediaExts = ['.mp4', '.webm', '.mov', '.mkv', '.m4v', '.mp3', '.wav', '.ogg', '.m4a', '.aac', '.flac'];
            const officeExts = ['.docx', '.doc', '.pptx', '.ppt', '.xlsx', '.xls', '.odt', '.ods', '.odp', '.pdf'];

            const isMedia = mediaExts.some(ext => cleanHref.endsWith(ext) || linkTextLower.endsWith(ext) || downloadLower.endsWith(ext));
            const isOffice = officeExts.some(ext => cleanHref.endsWith(ext) || linkTextLower.endsWith(ext) || downloadLower.endsWith(ext));

            if (isMedia && window.__docmostConfig.mediaPlayerEnabled) {{
                e.preventDefault();
                e.stopPropagation();
                const displayName = download || text || href.split('/').pop().split('?')[0] || 'Mediendatei';
                window.openMediaPlayer(href, displayName);
            }} else if (isOffice && window.__docmostConfig.officeViewerEnabled) {{
                e.preventDefault();
                e.stopPropagation();
                const displayName = download || text || href.split('/').pop().split('?')[0] || 'Dokument';
                window.openOfficeModal(href, displayName);
            }}
        }}, true);

        // Keyboard shortcuts
        window.addEventListener('keydown', (e) => {{
            if (e.key === 'Escape') {{
                window.closeAllDropdowns();
                const mediaModal = document.getElementById('docmost-media-modal');
                if (mediaModal && mediaModal.style.display === 'flex') {{
                    window.closeMediaPlayer();
                    return;
                }}
                const officeModal = document.getElementById('docmost-office-modal');
                if (officeModal && officeModal.style.display === 'flex') {{
                    window.closeOfficeModal();
                    return;
                }}
            }}
        }});

        // Factory Functions for DOM elements
        function createMenuTriggerButton() {{
            if (document.getElementById('docmost-app-menu-btn')) return;
            const btn = document.createElement('button');
            btn.id = 'docmost-app-menu-btn';
            btn.type = 'button';
            btn.title = 'Menü • {active_server_name}';
            btn.setAttribute('aria-label', 'Docmost Desktop Menü');
            btn.onclick = window.toggleAppMenu;
            btn.innerHTML = `
                <svg width="17" height="17" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                    <line x1="4" y1="6" x2="20" y2="6"></line>
                    <line x1="4" y1="12" x2="20" y2="12"></line>
                    <line x1="4" y1="18" x2="20" y2="18"></line>
                </svg>
                <span id="docmost-menu-update-dot"></span>
            `;
            document.body.appendChild(btn);
        }}

        function createAppMenuDropdown() {{
            if (document.getElementById('docmost-app-dropdown-menu')) return;
            const menu = document.createElement('div');
            menu.id = 'docmost-app-dropdown-menu';
            menu.className = 'docmost-dropdown-menu';
            menu.innerHTML = `
                <div class="docmost-dropdown-label" style="display:flex; justify-content:space-between; align-items:center;">
                    <span>DOCMOST DESKTOP</span>
                    <span style="font-size:10px; color:#12b886; font-weight:600;">v{version}</span>
                </div>
                <div class="docmost-dropdown-nav-row">
                    <button class="docmost-dropdown-nav-btn" onclick="window.history.back(); window.closeAllDropdowns();" title="Zurück (Alt+◀)">◀</button>
                    <button class="docmost-dropdown-nav-btn" onclick="window.history.forward(); window.closeAllDropdowns();" title="Vorwärts (Alt+▶)">▶</button>
                    <button class="docmost-dropdown-nav-btn" onclick="window.location.reload(); window.closeAllDropdowns();" title="Seite neu laden (F5)">↻</button>
                    <button class="docmost-dropdown-nav-btn" onclick="window.location.href='{url}'; window.closeAllDropdowns();" title="Startseite">🏠</button>
                </div>
                <div id="docmost-update-btn" class="docmost-update-banner" onclick="window.openUpdateModal(); window.closeAllDropdowns();">
                    <span>⬆ Update verfügbar</span>
                </div>
                <div class="docmost-dropdown-divider"></div>
                <div class="docmost-dropdown-label" style="display:flex; justify-content:space-between; align-items:center;">
                    <span>SERVER WECHSELN</span>
                    <button class="docmost-add-server-btn" onclick="window.openAddServerModal(event)">+ Neu</button>
                </div>
                <div id="docmost-app-menu-servers-list" class="docmost-dropdown-list"></div>
                <div class="docmost-dropdown-divider"></div>
                <div class="docmost-dropdown-label">EINSTELLUNGEN &amp; TOOLS</div>
                <div class="docmost-dropdown-item" onclick="window.switchSettingsTab('pen'); window.openSettingsModal(); window.closeAllDropdowns();">
                    <div class="docmost-dropdown-item-left"><span>🖊️</span><span>Stift &amp; Excalidraw</span></div>
                </div>
                <div class="docmost-dropdown-item" onclick="window.switchSettingsTab('office'); window.openSettingsModal(); window.closeAllDropdowns();">
                    <div class="docmost-dropdown-item-left"><span>📄</span><span>Office &amp; Medien-Viewer</span></div>
                </div>
                <div class="docmost-dropdown-item" onclick="window.switchSettingsTab('general'); window.openSettingsModal(); window.closeAllDropdowns();">
                    <div class="docmost-dropdown-item-left"><span>⚙</span><span>Alle Einstellungen...</span></div>
                </div>
                <div class="docmost-dropdown-divider"></div>
                <div class="docmost-dropdown-item" onclick="window.checkUpdatesManually(); window.closeAllDropdowns();">
                    <div class="docmost-dropdown-item-left"><span>🔄</span><span>Nach Updates suchen</span></div>
                </div>
            `;
            document.body.appendChild(menu);
        }}

        function createPenToolbar() {{
            if (document.getElementById('docmost-pen-toolbar')) return;
            const tb = document.createElement('div');
            tb.id = 'docmost-pen-toolbar';
            tb.innerHTML = `
                <button id="tb-btn-pen" class="docmost-pen-btn active" onclick="window.activatePenTool('freedraw')" title="Freihand-Stift (P / 2)">
                    <span>🖊️</span><span>Stift</span>
                </button>
                <button id="tb-btn-eraser" class="docmost-pen-btn" onclick="window.activatePenTool('eraser')" title="Radierer (E / 0)">
                    <span>🧹</span><span>Radierer</span>
                </button>
                <div style="width:1px; height:20px; background:#28314a; margin:0 2px;"></div>
                <button class="docmost-color-dot" style="background:#ffffff;" onclick="window.setPenColor('#ffffff')" title="Weiß"></button>
                <button class="docmost-color-dot" style="background:#818cf8;" onclick="window.setPenColor('#818cf8')" title="Indigo"></button>
                <button class="docmost-color-dot" style="background:#34d399;" onclick="window.setPenColor('#34d399')" title="Smaragdgrün"></button>
                <button class="docmost-color-dot" style="background:#fbbf24;" onclick="window.setPenColor('#fbbf24')" title="Sonnengelb"></button>
                <button class="docmost-color-dot" style="background:#f87171;" onclick="window.setPenColor('#f87171')" title="Korallenrot"></button>
                <div style="width:1px; height:20px; background:#28314a; margin:0 2px;"></div>
                <button class="docmost-pen-stroke-btn" onclick="window.setPenStroke(1)" title="Feine Strichstärke">―</button>
                <button class="docmost-pen-stroke-btn active" onclick="window.setPenStroke(2)" title="Mittlere Strichstärke">━</button>
                <button class="docmost-pen-stroke-btn" onclick="window.setPenStroke(4)" title="Breite Strichstärke">█</button>
                <div style="width:1px; height:20px; background:#28314a; margin:0 2px;"></div>
                <div id="badge-palm-rejection" style="font-size:11px; color:#10b981; display:flex; align-items:center; gap:4px; padding:3px 8px; background:rgba(16,185,129,0.12); border-radius:12px;" title="Handballenschutz & Stiftmodus aktiv">
                    <span>🖐️</span><span>Stift aktiv</span>
                </div>
                <button class="docmost-pen-btn" onclick="window.penUndo()" title="Rückgängig (Strg+Z)">↩</button>
                <button class="docmost-pen-btn" onclick="window.penRedo()" title="Wiederholen (Strg+Y)">↪</button>
                <button class="docmost-pen-btn" style="padding:4px 8px; color:#94a3b8;" onclick="document.getElementById('docmost-pen-toolbar').style.display='none'" title="Leiste ausblenden">✕</button>
            `;
            document.body.appendChild(tb);
        }}

        function createModals() {{
            if (document.getElementById('docmost-settings-modal')) return;
            const modalContainer = document.createElement('div');
            modalContainer.id = 'docmost-desktop-modals';
            modalContainer.innerHTML = `
                <!-- Tabbed Settings Modal -->
                <div id="docmost-settings-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box" style="width: 540px;">
                        <div class="docmost-modal-header">
                            <h3 style="margin:0; font-size: 16px;">Docmost Desktop Einstellungen</h3>
                            <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="window.closeSettingsModal()">✕</span>
                        </div>

                        <!-- Tabs Header -->
                        <div class="docmost-tabs-header">
                            <button id="tab-btn-general" class="docmost-tab-btn active" onclick="window.switchSettingsTab('general')">⚙ Allgemein & Server</button>
                            <button id="tab-btn-pen" class="docmost-tab-btn" onclick="window.switchSettingsTab('pen')">🖊️ Stift & Excalidraw</button>
                            <button id="tab-btn-office" class="docmost-tab-btn" onclick="window.switchSettingsTab('office')">📄 Office & Medien</button>
                            <button id="tab-btn-updates" class="docmost-tab-btn" onclick="window.switchSettingsTab('updates')">🔄 Updates</button>
                        </div>

                        <div class="docmost-modal-body" style="padding: 18px 20px;">
                            <!-- Tab 1: Allgemein & Server -->
                            <div id="pane-general" class="docmost-tab-pane active">
                                <div style="font-size: 13px; font-weight:600; color:#818cf8; margin-bottom: 8px; display:flex; justify-content:space-between; align-items:center;">
                                    <span>🌐 Gespeicherte Server</span>
                                    <button class="docmost-add-server-btn" onclick="window.openAddServerModal(event)">+ Neu</button>
                                </div>
                                <div id="settings-server-list" style="background:#0e111a; border:1px solid #28314a; border-radius:8px; margin-bottom:16px; overflow:hidden;"></div>

                                <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">GitHub Repository (für Updates)</label>
                                <input id="input-github-repo" type="text" value="{repo}" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:14px;" />
                                
                                <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">Oberflächen-Skalierung (Zoom)</label>
                                <select id="select-ui-zoom" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:6px;">
                                    <option value="0.9">90% (Kompakt)</option>
                                    <option value="1.0">100% (Standard)</option>
                                    <option value="1.1">110% (Leicht vergrößert)</option>
                                    <option value="1.25">125% (Ideal für Touch / Surface)</option>
                                    <option value="1.5">150% (Sehr groß)</option>
                                </select>
                            </div>

                            <!-- Tab 2: Stift & Excalidraw -->
                            <div id="pane-pen" class="docmost-tab-pane">
                                <div style="font-size: 13px; font-weight:600; color:#818cf8; margin-bottom: 12px; display:flex; align-items:center; gap:6px;">
                                    <span>🖊️</span><span>Optimierung für Stylus, Surface Pen & Touchscreen</span>
                                </div>

                                <div style="display:flex; flex-direction:column; gap:12px;">
                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-pen-mode" type="checkbox" {checked_pen_mode} style="margin-top:2px;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Stift-Modus (Pen-Only Mode)</div>
                                            <div style="font-size:11px; color:#94a3b8;">Nur der digitale Stift zeichnet; Fingergesten scrollen oder zoomen die Leinwand.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-palm-rejection" type="checkbox" {checked_palm_rejection} style="margin-top:2px;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Handballen-Schutz (Palm Rejection)</div>
                                            <div style="font-size:11px; color:#94a3b8;">Ignoriert aufliegende Handflächen während des Zeichnens automatisch.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-quick-toolbar" type="checkbox" {checked_quick_toolbar} style="margin-top:2px;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Schwebende Schnellzugriffsleiste</div>
                                            <div style="font-size:11px; color:#94a3b8;">Zeigt beim Zeichnen eine ergonomische Leiste für Stift, Radierer, Farben und Stärken.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-excalidraw-dark" type="checkbox" {checked_dark_theme} style="margin-top:2px;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Clean Dark für Excalidraw erzwingen</div>
                                            <div style="font-size:11px; color:#94a3b8;">Tiefdunkler Zeichenhintergrund ohne blendendes Weiß.</div>
                                        </div>
                                    </label>
                                </div>
                            </div>

                            <!-- Tab 4: Office & Medien -->
                            <div id="pane-office" class="docmost-tab-pane">
                                <div style="font-size: 13px; font-weight:600; color:#818cf8; margin-bottom: 12px; display:flex; align-items:center; gap:6px;">
                                    <span>📄</span><span>OnlyOffice &amp; Integrierter Medienplayer</span>
                                </div>

                                <div style="display:flex; flex-direction:column; gap:12px; margin-bottom:14px;">
                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-media-player" type="checkbox" {checked_media_player} style="margin-top:2px;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Integrierter Video- &amp; Audio-Player</div>
                                            <div style="font-size:11px; color:#94a3b8;">Spielt Videos (MP4, WebM, MOV) und Audiodateien (MP3, WAV, M4A) direkt in der Desktop-App ab.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-office-viewer" type="checkbox" {checked_office_viewer} style="margin-top:2px;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Office- &amp; Dokumenten-Viewer</div>
                                            <div style="font-size:11px; color:#94a3b8;">Ermöglicht das Öffnen von Word-, PowerPoint- &amp; Excel-Dateien mit 1 Klick in Ihrer lokalen Office-App oder OnlyOffice.</div>
                                        </div>
                                    </label>
                                </div>

                                <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">OnlyOffice Document Server URL (optional)</label>
                                <input id="input-onlyoffice-url" type="text" value="{onlyoffice_url}" placeholder="https://onlyoffice.meinedomain.de" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:6px;" />
                                <div style="font-size:11px; color:#64748b;">Falls Sie einen OnlyOffice Document Server betreiben, können Dokumente direkt im Viewer eingebettet bearbeitet werden.</div>
                            </div>

                            <!-- Tab 3: Updates -->
                            <div id="pane-updates" class="docmost-tab-pane">
                                <div style="margin-bottom:14px;">
                                    <label style="display:flex; align-items:center; gap:8px; font-size:13px; cursor:pointer;">
                                        <input id="check-auto-update" type="checkbox" {checked_auto_up} />
                                        <span>Automatisch beim Start nach Updates suchen</span>
                                    </label>
                                </div>

                                <div style="background:#0e111a; padding:12px; border-radius:6px; border:1px solid #20273c; font-size:12px; color:#94a3b8; margin-bottom:14px;">
                                    <div>Installierte Version: <strong style="color:#e2e8f0;">v{version}</strong></div>
                                    <div id="update-status-text" style="margin-top:6px; color:#10b981;">✓ Status: Bereit</div>
                                </div>

                                <div style="display:flex; gap:8px;">
                                    <button class="docmost-btn-secondary" onclick="window.checkUpdatesManually()" style="flex:1;">Jetzt nach Updates suchen</button>
                                    <button class="docmost-btn-secondary" onclick="window.testUpdateModalPreview()" style="flex:1;" title="Öffnet das Update-Fenster als Vorschau">Fenster-Vorschau testen</button>
                                </div>
                            </div>
                        </div>

                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.closeSettingsModal()">Abbrechen</button>
                            <button class="docmost-btn-primary" onclick="window.saveSettings()">Speichern & Übernehmen</button>
                        </div>
                    </div>
                </div>

                <!-- Add Server Modal -->
                <div id="docmost-add-server-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box" style="width: 420px;">
                        <div class="docmost-modal-header">
                            <h3 style="margin:0; font-size: 15px;">Neuen Server hinzufügen</h3>
                            <span style="cursor:pointer; color:#94a3b8; font-size:18px;" onclick="window.closeAddServerModal()">✕</span>
                        </div>
                        <div class="docmost-modal-body">
                            <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">Name / Bezeichnung</label>
                            <input id="input-new-server-name" type="text" placeholder="z. B. Arbeit, Wiki, Test" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:14px;" />

                            <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">Docmost Server URL</label>
                            <input id="input-new-server-url" type="text" placeholder="https://wiki.firma.de" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px;" />
                        </div>
                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.closeAddServerModal()">Abbrechen</button>
                            <button class="docmost-btn-primary" onclick="window.submitAddServer()">Hinzufügen & Wechseln</button>
                        </div>
                    </div>
                </div>

                <!-- Update Available Modal -->
                <div id="docmost-update-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box">
                        <div class="docmost-modal-header" style="background: linear-gradient(135deg, #1e1b4b, #1e293b);">
                            <h3 style="margin:0; font-size: 16px; color:#a5b4fc;">🚀 Neues Update verfügbar!</h3>
                            <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="window.closeUpdateModal()">✕</span>
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
                            <button class="docmost-btn-secondary" onclick="window.closeUpdateModal()">Später</button>
                            <button id="btn-do-update" class="docmost-btn-primary" onclick="window.performUpdate()">Jetzt herunterladen & installieren</button>
                        </div>
                    </div>
                </div>

                <!-- Media Player Modal -->
                <div id="docmost-media-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box">
                        <div class="docmost-modal-header">
                            <div style="display:flex; align-items:center; gap:8px; overflow:hidden;">
                                <span class="docmost-badge-format" id="media-modal-badge">MEDIA</span>
                                <h3 style="margin:0; font-size: 15px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; max-width:600px;" id="media-modal-title">Mediendatei</h3>
                            </div>
                            <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="window.closeMediaPlayer()">✕</span>
                        </div>
                        <div class="docmost-media-content">
                            <div id="docmost-video-container" style="display:flex; justify-content:center; align-items:center; width:100%;">
                                <video id="docmost-media-video" controls></video>
                            </div>
                            <div id="docmost-audio-view" class="docmost-audio-view" style="display:none;">
                                <div class="docmost-audio-visual">🎵</div>
                                <audio id="docmost-media-audio" controls></audio>
                            </div>
                        </div>
                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.openCurrentMediaExternal()" title="In lokalem Standard-Player (z. B. VLC) öffnen">In Standard-Player öffnen ↗</button>
                            <button class="docmost-btn-secondary" onclick="window.closeMediaPlayer()">Schließen</button>
                        </div>
                    </div>
                </div>

                <!-- Office & Document Viewer Modal -->
                <div id="docmost-office-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box">
                        <div class="docmost-modal-header">
                            <div style="display:flex; align-items:center; gap:8px; overflow:hidden;">
                                <span class="docmost-badge-format" id="office-modal-badge">DOC</span>
                                <h3 style="margin:0; font-size: 15px; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; max-width:680px;" id="office-modal-title">Dokument</h3>
                            </div>
                            <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="window.closeOfficeModal()">✕</span>
                        </div>
                        <div class="docmost-office-body">
                            <iframe id="docmost-office-iframe" style="display:none;"></iframe>
                            <div id="docmost-office-fallback-card" class="docmost-office-fallback-card">
                                <div id="office-fallback-icon" style="font-size:48px;">📄</div>
                                <div>
                                    <div id="office-fallback-filename" style="font-size:16px; font-weight:700; color:#f1f5f9; margin-bottom:6px;">Dokument</div>
                                    <div style="font-size:13px; color:#94a3b8; max-width:440px; line-height:1.4;">
                                        Öffnen Sie diese Office-Datei mit einem Klick in Ihrer lokal installierten Office-Suite (Microsoft Word, PowerPoint, Excel oder OnlyOffice Desktop).
                                    </div>
                                </div>
                                <div style="display:flex; flex-direction:column; gap:10px; width:100%; max-width:320px; margin-top:8px;">
                                    <button class="docmost-btn-primary" style="padding:10px 18px; font-size:14px;" onclick="window.openCurrentOfficeExternal()">
                                        ⚡ In lokaler Office-App öffnen
                                    </button>
                                    <button class="docmost-btn-secondary" style="padding:8px 14px; font-size:12px;" onclick="window.toggleOfficeOnlineViewer()">
                                        🌐 In Office Online Web-Vorschau ansehen
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.openCurrentOfficeExternal()" title="In lokaler Word / PowerPoint / Excel / OnlyOffice App öffnen">In lokaler Office-App öffnen ↗</button>
                            <button class="docmost-btn-secondary" onclick="window.closeOfficeModal()">Schließen</button>
                        </div>
                    </div>
                </div>
            `;
            document.body.appendChild(modalContainer);

            const zoomSelect = document.getElementById('select-ui-zoom');
            if (zoomSelect) {{
                zoomSelect.value = String(window.__docmostConfig.zoomLevel || 1.0);
            }}

            if (window.__pendingUpdateInfo) {{
                window.onUpdateAvailable(window.__pendingUpdateInfo);
            }}
            if (!window.__hasCheckedUpdateOnInit && window.ipc) {{
                window.__hasCheckedUpdateOnInit = true;
                window.ipc.postMessage(JSON.stringify({{ action: 'check_pending_update' }}));
            }}
        }}

        // Master UI Synchronizer
        function ensureDesktopUI() {{
            try {{
                injectStyles();
                applyZoom();
                syncExcalidrawState();

                if (!document.body) return;

                if (!document.body.classList.contains('has-desktop-menu')) {{
                    document.body.classList.add('has-desktop-menu');
                }}

                if (!document.getElementById('docmost-app-menu-btn')) {{
                    createMenuTriggerButton();
                }}

                if (!document.getElementById('docmost-app-dropdown-menu')) {{
                    createAppMenuDropdown();
                }}

                if (!document.getElementById('docmost-settings-modal')) {{
                    createModals();
                }}

                if (!document.getElementById('docmost-pen-toolbar')) {{
                    createPenToolbar();
                }}

                checkExcalidrawPresence();
            }} catch (err) {{
                console.error("[Docmost Desktop] ensureDesktopUI error:", err);
            }}
        }}

        let uiDebounceTimer = null;
        function debouncedEnsureDesktopUI() {{
            if (uiDebounceTimer) clearTimeout(uiDebounceTimer);
            uiDebounceTimer = setTimeout(ensureDesktopUI, 80);
        }}

        // Trigger on all relevant phases
        ensureDesktopUI();

        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', ensureDesktopUI);
        }}
        window.addEventListener('load', ensureDesktopUI);

        // Persistent observer on document tree with intelligent debouncing
        try {{
            const observer = new MutationObserver((mutations) => {{
                let ignore = true;
                for (let i = 0; i < mutations.length; i++) {{
                    const t = mutations[i].target;
                    if (!t || (t.id !== 'docmost-app-menu-btn' && t.id !== 'docmost-desktop-modals' && !t.closest?.('#docmost-desktop-modals, #docmost-app-menu-btn, #docmost-app-dropdown-menu, #docmost-pen-toolbar'))) {{
                        ignore = false;
                        break;
                    }}
                }}
                if (!ignore) {{
                    debouncedEnsureDesktopUI();
                }}
            }});
            if (document.body) {{
                observer.observe(document.body, {{ childList: true, subtree: true }});
            }} else if (document.documentElement) {{
                observer.observe(document.documentElement, {{ childList: true, subtree: true }});
            }}
        }} catch (obsErr) {{
            console.error("[Docmost Desktop] MutationObserver error:", obsErr);
        }}

        // Polling interval safeguard (runs for 6 seconds after start)
        let pollCount = 0;
        const pollTimer = setInterval(() => {{
            ensureDesktopUI();
            pollCount++;
            if (pollCount > 24) {{
                clearInterval(pollTimer);
            }}
        }}, 250);

    }} catch (fatalErr) {{
        console.error("[Docmost Desktop] Global Init Fatal Error:", fatalErr);
    }}
}})();
    "#,
    css = CLEAN_DARK_GLOBAL_CSS,
    url = config.server_url,
    active_server_name = active_name,
    servers_json = servers_json,
    active_server_id = active_id,
    repo = config.github_repo,
    auto_update = config.check_updates_on_startup,
    pen_mode = config.excalidraw_pen_mode,
    palm_rejection = config.excalidraw_palm_rejection,
    dark_theme = config.excalidraw_dark_theme,
    quick_toolbar = config.excalidraw_quick_toolbar,
    zoom_level = config.ui_zoom_level,
    version = current_version,
    checked_pen_mode = if config.excalidraw_pen_mode { "checked" } else { "" },
    checked_palm_rejection = if config.excalidraw_palm_rejection { "checked" } else { "" },
    checked_quick_toolbar = if config.excalidraw_quick_toolbar { "checked" } else { "" },
    checked_dark_theme = if config.excalidraw_dark_theme { "checked" } else { "" },
    checked_auto_up = if config.check_updates_on_startup { "checked" } else { "" },
    onlyoffice_url = config.onlyoffice_url,
    media_player_enabled = config.media_player_enabled,
    office_viewer_enabled = config.office_viewer_enabled,
    checked_media_player = if config.media_player_enabled { "checked" } else { "" },
    checked_office_viewer = if config.office_viewer_enabled { "checked" } else { "" }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injection_script_output() {
        let cfg = crate::config::AppConfig::default();
        let script = get_injection_script(&cfg, "0.1.7");
        let _ = std::fs::write("target/test_injection.js", &script);
        assert!(!script.contains("docmost-desktop-nav"));
        assert!(script.contains("docmost-app-menu-btn"));
        assert!(script.contains("docmost-app-dropdown-menu"));
        assert!(script.contains("docmost-menu-update-dot"));
        assert!(script.contains("toggleAppMenu"));
        assert!(script.contains("renderAppMenuServers"));
        assert!(script.contains("closeAllDropdowns"));
        assert!(script.contains("docmost-media-modal"));
        assert!(script.contains("docmost-office-modal"));
        assert!(script.contains("openMediaPlayer"));
        assert!(script.contains("openOfficeModal"));
    }
}
