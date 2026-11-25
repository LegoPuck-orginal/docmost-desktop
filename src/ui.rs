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
    background: #1f1f1f;
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
body.has-desktop-menu .mantine-AppShell-header {
    padding-right: 44px !important;
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
"#;

pub const MODAL_UI_CSS: &str = r#"
/* Injected Modal Overlay (Docmost Mantine Dark) */
.docmost-modal-overlay {
    display: none;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.78);
    z-index: 2147483647 !important;
    align-items: center;
    justify-content: center;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
}

.docmost-modal-box {
    background: #1f1f1f;
    border: 1px solid #383838;
    border-radius: 8px;
    width: 480px;
    max-width: 90vw;
    box-shadow: 0 20px 48px rgba(0, 0, 0, 0.8);
    color: #c9c9c9;
    overflow: hidden;
}

.docmost-modal-header {
    padding: 14px 18px;
    background: #242424;
    border-bottom: 1px solid #2e2e2e;
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: #f1f5f9;
}

.docmost-modal-close {
    cursor: pointer;
    color: #828282;
    font-size: 16px;
    width: 26px;
    height: 26px;
    border-radius: 4px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.12s ease, color 0.12s ease;
    user-select: none;
}
.docmost-modal-close:hover {
    background: #2e2e2e;
    color: #ffffff;
}

.docmost-modal-body {
    padding: 18px 20px;
}

.docmost-modal-footer {
    padding: 12px 18px;
    background: #1a1a1a;
    border-top: 1px solid #2e2e2e;
    display: flex;
    justify-content: flex-end;
    gap: 8px;
}

.docmost-section-title {
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.4px;
    color: #828282;
    margin-bottom: 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 6px;
}

.docmost-surface-card {
    background: #141414;
    border: 1px solid #2e2e2e;
    border-radius: 6px;
    overflow: hidden;
}

.docmost-field-label {
    font-size: 12px;
    font-weight: 500;
    color: #b8b8b8;
    display: block;
    margin-bottom: 6px;
}

.docmost-input {
    width: 100%;
    box-sizing: border-box;
    background: #141414;
    border: 1px solid #383838;
    border-radius: 6px;
    padding: 8px 12px;
    color: #f1f5f9;
    font-size: 13px;
    outline: none;
    transition: border-color 0.12s ease;
}
.docmost-input:focus {
    border-color: #228be6;
}

.docmost-btn-primary {
    background: #228be6;
    color: #fff;
    border: none;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: background-color 0.12s ease;
}
.docmost-btn-primary:hover {
    background: #1c7ed6;
}

.docmost-btn-secondary {
    background: #2e2e2e;
    color: #c9c9c9;
    border: 1px solid #3e3e3e;
    border-radius: 6px;
    padding: 8px 16px;
    font-size: 13px;
    cursor: pointer;
    transition: background-color 0.12s ease, color 0.12s ease;
}
.docmost-btn-secondary:hover {
    background: #383838;
    color: #ffffff;
}

/* Settings Tabs (Mantine Style) */
.docmost-tabs-header {
    display: flex;
    border-bottom: 1px solid #2e2e2e;
    padding: 0 14px;
    background: #1a1a1a;
    gap: 4px;
}
.docmost-tab-btn {
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: #828282;
    font-size: 13px;
    font-weight: 500;
    padding: 10px 12px;
    cursor: pointer;
    transition: color 0.12s ease, border-color 0.12s ease;
}
.docmost-tab-btn:hover {
    color: #c9c9c9;
}
.docmost-tab-btn.active {
    color: #ffffff;
    border-bottom-color: #228be6;
    font-weight: 600;
}
.docmost-tab-pane {
    display: none;
}
.docmost-tab-pane.active {
    display: block;
}

/* Media Player Modal (Docmost Mantine Style) */
#docmost-media-modal .docmost-modal-box {
    width: 860px;
    max-width: 95vw;
    background: #1f1f1f;
    border: 1px solid #383838;
    border-radius: 8px;
    box-shadow: 0 24px 60px rgba(0,0,0,0.85);
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
.docmost-media-content {
    background: #141414;
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
    border-radius: 6px;
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
    background: linear-gradient(135deg, #228be6, #339af0);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 30px;
    box-shadow: 0 0 20px rgba(34, 139, 230, 0.4);
}
#docmost-media-audio {
    width: 85%;
    max-width: 550px;
}
.docmost-badge-format {
    background: #2e2e2e;
    color: #74c0fc;
    border: 1px solid #3e3e3e;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 700;
    padding: 2px 8px;
    text-transform: uppercase;
}

/* Office & Document Viewer Modal (Docmost Mantine Style) */
#docmost-office-modal .docmost-modal-box {
    width: 1040px;
    max-width: 96vw;
    height: 88vh;
    background: #1f1f1f;
    border: 1px solid #383838;
    border-radius: 8px;
    box-shadow: 0 24px 65px rgba(0,0,0,0.85);
    display: flex;
    flex-direction: column;
    overflow: hidden;
}
.docmost-office-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: #141414;
    position: relative;
    overflow: hidden;
}
#docmost-office-iframe {
    width: 100%;
    height: 100%;
    border: none;
    background: #1a1a1a;
}
.docmost-office-fallback-card {
    margin: auto;
    max-width: 560px;
    padding: 36px 30px;
    background: #242424;
    border: 1px solid #383838;
    border-radius: 8px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
    box-shadow: 0 12px 30px rgba(0,0,0,0.5);
}
"#;

pub const EXCALIDRAW_UI_CSS: &str = r#"
/* ===== EXCALIDRAW DOCMOST MANTINE DARK ENHANCEMENTS ===== */
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
    --color-primary-darkest: #1971c2 !important;
    --color-primary-light: #74c0fc !important;
    background-color: #141414 !important;
}

/* Excalidraw Canvas Background */
.excalidraw canvas,
.excalidraw .excalidraw__canvas {
    background-color: #141414 !important;
}

/* Floating Toolbars (Island) */
.excalidraw .Island,
.excalidraw .App-toolbar,
.excalidraw .layer-ui__wrapper .Island {
    background: #242424 !important;
    border: 1px solid #383838 !important;
    border-radius: 8px !important;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.6) !important;
    padding: 4px 6px !important;
}

/* Excalidraw Tool Icons */
.excalidraw .ToolIcon_type_button,
.excalidraw .ToolIcon_type_radio,
.excalidraw .ToolIcon,
.excalidraw button.ToolIcon {
    border-radius: 6px !important;
    color: #b8b8b8 !important;
    background: transparent !important;
    border: 1px solid transparent !important;
    transition: background-color 0.12s ease, color 0.12s ease !important;
    margin: 2px !important;
}

.excalidraw .ToolIcon:hover,
.excalidraw .ToolIcon_type_button:hover,
.excalidraw .ToolIcon_type_radio:hover {
    background: #2e2e2e !important;
    color: #ffffff !important;
    border-color: #424242 !important;
}

/* Active / Selected Tool */
.excalidraw .ToolIcon_type_radio:checked + label .ToolIcon,
.excalidraw .ToolIcon_type_radio:checked,
.excalidraw .ToolIcon--selected,
.excalidraw .ToolIcon.active,
.excalidraw .ToolIcon[aria-pressed="true"] {
    background: #228be6 !important;
    color: #ffffff !important;
    border-color: #339af0 !important;
}

.excalidraw .ToolIcon--selected svg,
.excalidraw .ToolIcon.active svg {
    fill: #ffffff !important;
    stroke: #ffffff !important;
}

/* Excalidraw Sidebar & Properties Panel */
.excalidraw .panelColumn,
.excalidraw .sidebar-content,
.excalidraw .layer-ui__wrapper .sidebar,
.excalidraw .context-menu,
.excalidraw .dropdown-menu,
.excalidraw .popover {
    background: #242424 !important;
    border: 1px solid #383838 !important;
    border-radius: 8px !important;
    color: #c9c9c9 !important;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7) !important;
}

.excalidraw .context-menu-item,
.excalidraw .dropdown-menu-item {
    color: #c9c9c9 !important;
    border-radius: 4px !important;
    transition: background-color 0.12s ease, color 0.12s ease !important;
}

.excalidraw .context-menu-item:hover,
.excalidraw .dropdown-menu-item:hover {
    background: #2e2e2e !important;
    color: #ffffff !important;
}

/* Excalidraw Color Picker Palette */
.excalidraw .color-picker,
.excalidraw .picker {
    background: #242424 !important;
    border: 1px solid #383838 !important;
    border-radius: 8px !important;
    padding: 8px !important;
}

.excalidraw .color-picker-swatch {
    border-radius: 5px !important;
    border: 1px solid rgba(255, 255, 255, 0.15) !important;
    transition: transform 0.1s ease !important;
}

.excalidraw .color-picker-swatch:hover {
    transform: scale(1.15) !important;
    border-color: #ffffff !important;
}

/* Bottom Controls */
.excalidraw .layer-ui__wrapper .footer-center,
.excalidraw .layer-ui__wrapper .footer-left,
.excalidraw .layer-ui__wrapper .footer-right,
.excalidraw .undo-redo-buttons,
.excalidraw .zoom-actions {
    background: #242424 !important;
    border: 1px solid #383838 !important;
    border-radius: 8px !important;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5) !important;
}

/* Modal Dialog when editing full diagram */
.excalidraw-modal-container,
.excalidraw .Modal,
.excalidraw .Modal__content {
    background: #1f1f1f !important;
    border: 1px solid #383838 !important;
    border-radius: 8px !important;
    box-shadow: 0 20px 48px rgba(0, 0, 0, 0.8) !important;
    color: #c9c9c9 !important;
}

.excalidraw .Modal__header {
    border-bottom: 1px solid #2e2e2e !important;
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

/* Floating Stylus Quick Toolbar (Docmost Mantine Style) */
#docmost-pen-toolbar {
    position: fixed;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    background: #242424;
    border: 1px solid #383838;
    border-radius: 40px;
    padding: 6px 14px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7);
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
    color: #c9c9c9;
    border-radius: 20px;
    padding: 6px 12px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 5px;
    transition: background-color 0.12s ease, color 0.12s ease;
}
.docmost-pen-btn:hover {
    background: #2e2e2e;
    color: #fff;
}
.docmost-pen-btn.active {
    background: #228be6;
    color: #fff;
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
    color: #828282;
    border-radius: 6px;
    width: 28px;
    height: 28px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.12s ease, color 0.12s ease;
}
.docmost-pen-stroke-btn:hover {
    background: #2e2e2e;
    color: #fff;
}
.docmost-pen-stroke-btn.active {
    background: #2e2e2e;
    color: #339af0;
    border-color: #228be6;
}
"#;

/// Strips CSS comments and excessive whitespace to shrink injected script size and parse cost.
pub fn compact_css(css: &str) -> String {
    let mut no_comments = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        no_comments.push_str(&rest[..start]);
        if let Some(end) = rest[start + 2..].find("*/") {
            rest = &rest[start + 2 + end + 2..];
        } else {
            rest = "";
            break;
        }
    }
    no_comments.push_str(rest);

    let mut out = String::with_capacity(no_comments.len());
    for line in no_comments.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(trimmed);
    }
    out
}

pub fn get_injection_script(config: &crate::config::AppConfig, current_version: &str) -> String {
    let servers_json = serde_json::to_string(&config.servers).unwrap_or_else(|_| "[]".to_string());
    let active_id = config.active_server_id.as_deref().unwrap_or("default");
    let active_name = config.active_server().map(|s| s.name.as_str()).unwrap_or("Server");
    let css_global_js = serde_json::to_string(&compact_css(CLEAN_DARK_GLOBAL_CSS)).unwrap_or_else(|_| "\"\"".to_string());
    let css_modals_js = serde_json::to_string(&compact_css(MODAL_UI_CSS)).unwrap_or_else(|_| "\"\"".to_string());
    let css_excalidraw_js = serde_json::to_string(&compact_css(EXCALIDRAW_UI_CSS)).unwrap_or_else(|_| "\"\"".to_string());

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

        let styleInjected = false;
        let modalStyleInjected = false;
        let excalidrawStyleInjected = false;
        let appMenuOpen = false;
        let modalsCreated = false;
        let lastZoomApplied = null;
        let excalidrawWasPresent = false;

        const injectStyleOnce = (id, cssText) => {{
            if (document.getElementById(id)) return true;
            const target = document.head || document.documentElement;
            if (!target) return false;
            const style = document.createElement('style');
            style.id = id;
            style.textContent = cssText;
            target.appendChild(style);
            return true;
        }};

        // Safe CSS Injection (cached & split for fast startup)
        const injectStyles = () => {{
            if (styleInjected && document.getElementById('docmost-desktop-style')) return;
            styleInjected = injectStyleOnce('docmost-desktop-style', {css});
        }};

        const injectModalStyles = () => {{
            if (modalStyleInjected) return;
            modalStyleInjected = injectStyleOnce('docmost-desktop-modal-style', {modal_css});
        }};

        const injectExcalidrawStyles = () => {{
            if (excalidrawStyleInjected) return;
            excalidrawStyleInjected = injectStyleOnce('docmost-desktop-excalidraw-style', {excalidraw_css});
        }};

        // Safe Zoom (cached to avoid redundant style recalc)
        const applyZoom = () => {{
            const z = window.__docmostConfig.zoomLevel || 1.0;
            if (!document.documentElement || z === lastZoomApplied) return;
            if (z !== 1.0 || lastZoomApplied !== null) {{
                document.documentElement.style.zoom = z;
            }}
            lastZoomApplied = z;
        }};

        // Auto-sync Excalidraw Dark Mode & Pen Mode (only writes localStorage when changed)
        const syncExcalidrawState = () => {{
            if (!excalidrawWasPresent && !document.querySelector('.excalidraw, [data-testid="excalidraw-container"]')) return;
            try {{
                const rawState = localStorage.getItem('excalidraw-state');
                let state = rawState ? JSON.parse(rawState) : {{}};
                let dirty = false;
                if (window.__docmostConfig.darkTheme) {{
                    if (state.theme !== 'dark') {{ state.theme = 'dark'; dirty = true; }}
                    if (localStorage.getItem('excalidraw-theme') !== 'dark') {{
                        localStorage.setItem('excalidraw-theme', 'dark');
                    }}
                }}
                if (window.__docmostConfig.penMode) {{
                    if (!state.penMode || !state.penDetected) {{
                        state.penMode = true;
                        state.penDetected = true;
                        dirty = true;
                    }}
                }}
                if (dirty) {{
                    localStorage.setItem('excalidraw-state', JSON.stringify(state));
                }}
            }} catch (e) {{}}
        }};

        const ensureExcalidrawDark = () => {{
            if (window.__docmostConfig.darkTheme) {{
                document.querySelectorAll('.excalidraw:not(.theme--dark)').forEach(el => {{
                    el.classList.add('theme--dark');
                }});
            }}
        }};

        const checkExcalidrawPresence = () => {{
            const hasExcalidraw = !!document.querySelector('.excalidraw, [data-testid="excalidraw-container"]');
            if (hasExcalidraw) {{
                injectExcalidrawStyles();
                ensureExcalidrawDark();
                if (!excalidrawWasPresent) {{
                    excalidrawWasPresent = true;
                    syncExcalidrawState();
                }}
                if (window.__docmostConfig.quickToolbar) {{
                    if (!document.getElementById('docmost-pen-toolbar')) {{
                        createPenToolbar();
                    }}
                }}
            }} else {{
                excalidrawWasPresent = false;
            }}
            const penToolbar = document.getElementById('docmost-pen-toolbar');
            if (penToolbar) {{
                const wantDisplay = (hasExcalidraw && window.__docmostConfig.quickToolbar) ? 'flex' : 'none';
                if (penToolbar.style.display !== wantDisplay) {{
                    penToolbar.style.display = wantDisplay;
                }}
            }}
        }};

        // Stylus / Palm Rejection Logic (zero DOM traversal when Excalidraw is inactive)
        let lastPenContactTime = 0;
        window.addEventListener('pointerdown', (e) => {{
            if (!excalidrawWasPresent) return;
            if (e.pointerType === 'pen') {{
                lastPenContactTime = Date.now();
                const badge = document.getElementById('badge-palm-rejection');
                if (badge) badge.style.opacity = '1';
            }} else if (e.pointerType === 'touch' && window.__docmostConfig.palmRejection) {{
                const onCanvas = e.target.closest('.excalidraw canvas, .excalidraw');
                if (onCanvas && (Date.now() - lastPenContactTime < 1200 || window.__docmostConfig.penMode)) {{
                    if (e.isPrimary && !e.shiftKey) {{
                        e.stopPropagation();
                    }}
                }}
            }}
        }}, true);

        window.addEventListener('pointerup', (e) => {{
            if (!excalidrawWasPresent) return;
            if (e.pointerType === 'pen') {{
                lastPenContactTime = Date.now();
            }}
        }}, true);

        window.addEventListener('contextmenu', (e) => {{
            if (!excalidrawWasPresent) return;
            if (e.target.closest('.excalidraw canvas, .excalidraw')) {{
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

        function escapeHtml(str) {{
            return String(str || '')
                .replace(/&/g, '&amp;')
                .replace(/</g, '&lt;')
                .replace(/>/g, '&gt;')
                .replace(/"/g, '&quot;')
                .replace(/'/g, '&#39;');
        }}

        function getUrlOrigin(u) {{
            try {{ return new URL(u).origin.toLowerCase(); }} catch (e) {{ return String(u || '').replace(/\/+$/, '').toLowerCase(); }}
        }}

        window.goHome = function() {{
            const target = (window.__docmostConfig && window.__docmostConfig.serverUrl) || '{url}';
            window.closeAllDropdowns();
            if (target) window.location.href = target;
        }};

        window.onDesktopConfigSync = function(newCfg) {{
            if (!newCfg) return;
            window.__docmostConfig = Object.assign({{}}, window.__docmostConfig || {{}}, newCfg);
            applyZoom();

            const menuBtn = document.getElementById('docmost-app-menu-btn');
            if (menuBtn) {{
                const activeSrv = (window.__docmostConfig.servers || []).find(s => s.id === window.__docmostConfig.activeServerId);
                if (activeSrv) menuBtn.title = 'Menü • ' + activeSrv.name;
            }}

            if (document.getElementById('docmost-app-menu-servers-list')) {{
                window.renderAppMenuServers();
            }}
            if (document.getElementById('settings-server-list')) {{
                window.renderSettingsServerList();
            }}
        }};

        // Dropdown Menu Management (Top-Right 3-Line Menu)
        window.closeAllDropdowns = function() {{
            appMenuOpen = false;
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
                const displayUrl = escapeHtml(s.url.replace(/^https?:\/\//i, ''));
                const safeName = escapeHtml(s.name);
                const safeId = escapeHtml(s.id);
                html += `
                    <div class="docmost-dropdown-item ${{isActive ? 'active' : ''}}" data-server-id="${{safeId}}">
                        <div class="docmost-dropdown-item-left" style="flex:1; min-width:0;">
                            <span class="docmost-status-dot" style="${{isActive ? '' : 'background:#666; box-shadow:none;'}}"></span>
                            <div style="overflow:hidden;">
                                <div style="font-size:13px; font-weight:600; color:${{isActive ? '#ffffff' : '#c9c9c9'}}; overflow:hidden; text-overflow:ellipsis;">${{safeName}} ${{isActive ? '<span style="font-size:10px; color:#339af0; font-weight:normal;">(Aktiv)</span>' : ''}}</div>
                                <div style="font-size:11px; color:#828282; overflow:hidden; text-overflow:ellipsis;">${{displayUrl}}</div>
                            </div>
                        </div>
                        ${{servers.length > 1 ? `<button type="button" class="docmost-server-del-btn" data-delete-server-id="${{safeId}}" title="Server entfernen">✕</button>` : ''}}
                    </div>
                `;
            }});
            container.innerHTML = html;
        }};

        window.toggleAppMenu = function(e) {{
            if (e) e.stopPropagation();
            createAppMenuDropdown();
            const menu = document.getElementById('docmost-app-dropdown-menu');
            const menuBtn = document.getElementById('docmost-app-menu-btn');
            if (!menu) return;

            if (menu.style.display === 'flex') {{
                menu.style.display = 'none';
                appMenuOpen = false;
                if (menuBtn) menuBtn.classList.remove('active');
            }} else {{
                window.renderAppMenuServers();
                if (window.__pendingUpdateInfo) {{
                    const banner = document.getElementById('docmost-update-btn');
                    if (banner) {{
                        banner.style.display = 'flex';
                        banner.innerHTML = '<span>⬆ Update verfügbar (' + (window.__pendingUpdateInfo.version || '') + ')</span>';
                    }}
                }}
                if (menuBtn) {{
                    const rect = menuBtn.getBoundingClientRect();
                    menu.style.top = (rect.bottom + 6) + 'px';
                    menu.style.right = Math.max(8, window.innerWidth - rect.right) + 'px';
                    menu.style.left = 'auto';
                    menuBtn.classList.add('active');
                }}
                menu.style.display = 'flex';
                appMenuOpen = true;
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
                const displayUrl = escapeHtml(s.url.replace(/^https?:\/\//i, ''));
                const safeName = escapeHtml(s.name);
                const safeId = escapeHtml(s.id);
                html += `
                    <div style="padding:10px 12px; display:flex; align-items:center; justify-content:space-between; border-bottom:1px solid #2e2e2e; ${{isActive ? 'background:rgba(34,139,230,0.12); border-left:3px solid #228be6;' : ''}}">
                        <div style="display:flex; align-items:center; gap:8px; min-width:0; flex:1;">
                            <span class="docmost-status-dot" style="${{isActive ? '' : 'background:#666; box-shadow:none;'}}"></span>
                            <div style="overflow:hidden;">
                                <div style="font-size:13px; font-weight:600; color:#f1f5f9; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;">${{safeName}} ${{isActive ? '<span style="font-size:11px; color:#12b886; font-weight:normal;">(Aktiv)</span>' : ''}}</div>
                                <div style="font-size:11px; color:#828282; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;">${{displayUrl}}</div>
                            </div>
                        </div>
                        <div style="display:flex; gap:6px; align-items:center; flex-shrink:0; margin-left:8px;">
                            ${{!isActive ? `<button type="button" class="docmost-btn-secondary" style="padding:4px 10px; font-size:11px;" data-select-server-id="${{safeId}}">Wechseln</button>` : ''}}
                            ${{servers.length > 1 ? `<button type="button" class="docmost-server-del-btn" data-delete-server-id="${{safeId}}" title="Server entfernen">✕</button>` : ''}}
                        </div>
                    </div>
                `;
            }});
            container.innerHTML = html;
        }};

        window.addEventListener('click', (e) => {{
            if (!appMenuOpen) return;
            if (!e.target.closest('#docmost-app-dropdown-menu') &&
                !e.target.closest('#docmost-app-menu-btn')) {{
                window.closeAllDropdowns();
            }}
        }});

        window.selectServer = function(id) {{
            window.closeAllDropdowns();
            const servers = window.__docmostConfig.servers || [];
            const target = servers.find(s => s.id === id);
            if (!target) return;

            const sameOrigin = getUrlOrigin(target.url) === getUrlOrigin(window.location.href);
            if (id === window.__docmostConfig.activeServerId && sameOrigin) {{
                window.closeSettingsModal();
                return;
            }}
            window.__docmostConfig.activeServerId = id;
            window.__docmostConfig.serverUrl = target.url;
            window.closeSettingsModal();
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'switch_server',
                    server_id: id
                }}));
            }} else {{
                window.location.href = target.url;
            }}
        }};

        window.openAddServerModal = function(e) {{
            if (e) e.stopPropagation();
            window.closeAllDropdowns();
            createModals();
            const nameInput = document.getElementById('input-new-server-name');
            const urlInput = document.getElementById('input-new-server-url');
            if (nameInput) nameInput.value = '';
            if (urlInput) urlInput.value = '';
            const modal = document.getElementById('docmost-add-server-modal');
            if (modal) modal.style.display = 'flex';
            if (nameInput) setTimeout(() => nameInput.focus(), 30);
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
            if (!/^https?:\/\//i.test(url)) {{
                url = 'https://' + url;
            }}
            window.closeAddServerModal();
            window.closeSettingsModal();
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'add_server',
                    server_name: name || url,
                    server_url: url
                }}));
            }} else {{
                window.location.href = url;
            }}
        }};

        window.deleteServer = function(e, id) {{
            if (e) e.stopPropagation();
            if (!confirm('Diesen Server wirklich aus der Liste entfernen?')) return;
            window.__docmostConfig.servers = (window.__docmostConfig.servers || []).filter(s => s.id !== id);
            if (window.ipc) {{
                window.ipc.postMessage(JSON.stringify({{
                    action: 'delete_server',
                    server_id: id
                }}));
            }}
            window.renderAppMenuServers();
            window.renderSettingsServerList();
        }};

        window.switchSettingsTab = function(tabName) {{
            createModals();
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
            createModals();
            window.renderSettingsServerList();
            const el = document.getElementById('docmost-settings-modal');
            if (el) el.style.display = 'flex';
        }};
        window.closeSettingsModal = function() {{
            const el = document.getElementById('docmost-settings-modal');
            if (el) el.style.display = 'none';
        }};

        window.openUpdateModal = function() {{
            createModals();
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

            applyZoom();
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
            createModals();

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
            createModals();
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
            createModals();
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
            const cfg = window.__docmostConfig;
            if (!cfg || (!cfg.mediaPlayerEnabled && !cfg.officeViewerEnabled)) return;
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
            if (e.key !== 'Escape') return;
            if (!appMenuOpen && !modalsCreated) return;
            if (appMenuOpen) window.closeAllDropdowns();
            if (!modalsCreated) return;
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

        function bindServerContainerEvents(container) {{
            if (!container || container.__boundServerEvents) return;
            container.__boundServerEvents = true;
            container.addEventListener('click', (e) => {{
                const delBtn = e.target.closest('[data-delete-server-id]');
                if (delBtn) {{
                    window.deleteServer(e, delBtn.getAttribute('data-delete-server-id'));
                    return;
                }}
                const selEl = e.target.closest('[data-select-server-id], [data-server-id]');
                if (selEl) {{
                    const id = selEl.getAttribute('data-select-server-id') || selEl.getAttribute('data-server-id');
                    if (id) window.selectServer(id);
                }}
            }});
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
                    <button class="docmost-dropdown-nav-btn" onclick="window.goHome();" title="Startseite">🏠</button>
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
            bindServerContainerEvents(document.getElementById('docmost-app-menu-servers-list'));
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
                <div style="width:1px; height:20px; background:#383838; margin:0 2px;"></div>
                <button class="docmost-color-dot" style="background:#ffffff;" onclick="window.setPenColor('#ffffff')" title="Weiß"></button>
                <button class="docmost-color-dot" style="background:#339af0;" onclick="window.setPenColor('#339af0')" title="Blau"></button>
                <button class="docmost-color-dot" style="background:#12b886;" onclick="window.setPenColor('#12b886')" title="Grün"></button>
                <button class="docmost-color-dot" style="background:#fab005;" onclick="window.setPenColor('#fab005')" title="Gelb"></button>
                <button class="docmost-color-dot" style="background:#fa5252;" onclick="window.setPenColor('#fa5252')" title="Rot"></button>
                <div style="width:1px; height:20px; background:#383838; margin:0 2px;"></div>
                <button class="docmost-pen-stroke-btn" onclick="window.setPenStroke(1)" title="Feine Strichstärke">―</button>
                <button class="docmost-pen-stroke-btn active" onclick="window.setPenStroke(2)" title="Mittlere Strichstärke">━</button>
                <button class="docmost-pen-stroke-btn" onclick="window.setPenStroke(4)" title="Breite Strichstärke">█</button>
                <div style="width:1px; height:20px; background:#383838; margin:0 2px;"></div>
                <div id="badge-palm-rejection" style="font-size:11px; color:#12b886; display:flex; align-items:center; gap:4px; padding:3px 8px; background:rgba(18,184,134,0.12); border-radius:12px;" title="Handballenschutz &amp; Stiftmodus aktiv">
                    <span>🖐️</span><span>Stift aktiv</span>
                </div>
                <button class="docmost-pen-btn" onclick="window.penUndo()" title="Rückgängig (Strg+Z)">↩</button>
                <button class="docmost-pen-btn" onclick="window.penRedo()" title="Wiederholen (Strg+Y)">↪</button>
                <button class="docmost-pen-btn" style="padding:4px 8px; color:#828282;" onclick="document.getElementById('docmost-pen-toolbar').style.display='none'" title="Leiste ausblenden">✕</button>
            `;
            document.body.appendChild(tb);
        }}

        function createModals() {{
            if (document.getElementById('docmost-settings-modal')) {{
                modalsCreated = true;
                return;
            }}
            injectModalStyles();
            const modalContainer = document.createElement('div');
            modalContainer.id = 'docmost-desktop-modals';
            modalContainer.innerHTML = `
                <!-- Tabbed Settings Modal -->
                <div id="docmost-settings-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box" style="width: 540px;">
                        <div class="docmost-modal-header">
                            <h3 style="margin:0; font-size: 15px; font-weight: 600;">Docmost Desktop Einstellungen</h3>
                            <span class="docmost-modal-close" onclick="window.closeSettingsModal()">✕</span>
                        </div>

                        <!-- Tabs Header -->
                        <div class="docmost-tabs-header">
                            <button id="tab-btn-general" class="docmost-tab-btn active" onclick="window.switchSettingsTab('general')">⚙ Allgemein &amp; Server</button>
                            <button id="tab-btn-pen" class="docmost-tab-btn" onclick="window.switchSettingsTab('pen')">🖊️ Stift &amp; Excalidraw</button>
                            <button id="tab-btn-office" class="docmost-tab-btn" onclick="window.switchSettingsTab('office')">📄 Office &amp; Medien</button>
                            <button id="tab-btn-updates" class="docmost-tab-btn" onclick="window.switchSettingsTab('updates')">🔄 Updates</button>
                        </div>

                        <div class="docmost-modal-body">
                            <!-- Tab 1: Allgemein & Server -->
                            <div id="pane-general" class="docmost-tab-pane active">
                                <div class="docmost-section-title">
                                    <span>🌐 Gespeicherte Server</span>
                                    <button class="docmost-add-server-btn" onclick="window.openAddServerModal(event)">+ Neu</button>
                                </div>
                                <div id="settings-server-list" class="docmost-surface-card" style="margin-bottom:16px;"></div>

                                <label class="docmost-field-label">GitHub Repository (für Updates)</label>
                                <input id="input-github-repo" class="docmost-input" type="text" value="{repo}" style="margin-bottom:14px;" />
                                
                                <label class="docmost-field-label">Oberflächen-Skalierung (Zoom)</label>
                                <select id="select-ui-zoom" class="docmost-input" style="margin-bottom:6px;">
                                    <option value="0.9">90% (Kompakt)</option>
                                    <option value="1.0">100% (Standard)</option>
                                    <option value="1.1">110% (Leicht vergrößert)</option>
                                    <option value="1.25">125% (Ideal für Touch / Surface)</option>
                                    <option value="1.5">150% (Sehr groß)</option>
                                </select>
                            </div>

                            <!-- Tab 2: Stift & Excalidraw -->
                            <div id="pane-pen" class="docmost-tab-pane">
                                <div class="docmost-section-title" style="justify-content:flex-start;">
                                    <span>🖊️</span><span>Optimierung für Stylus, Surface Pen &amp; Touchscreen</span>
                                </div>

                                <div style="display:flex; flex-direction:column; gap:12px;">
                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-pen-mode" type="checkbox" {checked_pen_mode} style="margin-top:2px; accent-color:#228be6;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Stift-Modus (Pen-Only Mode)</div>
                                            <div style="font-size:11px; color:#828282;">Nur der digitale Stift zeichnet; Fingergesten scrollen oder zoomen die Leinwand.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-palm-rejection" type="checkbox" {checked_palm_rejection} style="margin-top:2px; accent-color:#228be6;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Handballen-Schutz (Palm Rejection)</div>
                                            <div style="font-size:11px; color:#828282;">Ignoriert aufliegende Handflächen während des Zeichnens automatisch.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-quick-toolbar" type="checkbox" {checked_quick_toolbar} style="margin-top:2px; accent-color:#228be6;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Schwebende Schnellzugriffsleiste</div>
                                            <div style="font-size:11px; color:#828282;">Zeigt beim Zeichnen eine ergonomische Leiste für Stift, Radierer, Farben und Stärken.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-excalidraw-dark" type="checkbox" {checked_dark_theme} style="margin-top:2px; accent-color:#228be6;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Clean Dark für Excalidraw erzwingen</div>
                                            <div style="font-size:11px; color:#828282;">Tiefdunkler Zeichenhintergrund ohne blendendes Weiß.</div>
                                        </div>
                                    </label>
                                </div>
                            </div>

                            <!-- Tab 4: Office & Medien -->
                            <div id="pane-office" class="docmost-tab-pane">
                                <div class="docmost-section-title" style="justify-content:flex-start;">
                                    <span>📄</span><span>OnlyOffice &amp; Integrierter Medienplayer</span>
                                </div>

                                <div style="display:flex; flex-direction:column; gap:12px; margin-bottom:14px;">
                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-media-player" type="checkbox" {checked_media_player} style="margin-top:2px; accent-color:#228be6;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Integrierter Video- &amp; Audio-Player</div>
                                            <div style="font-size:11px; color:#828282;">Spielt Videos (MP4, WebM, MOV) und Audiodateien (MP3, WAV, M4A) direkt in der Desktop-App ab.</div>
                                        </div>
                                    </label>

                                    <label style="display:flex; align-items:flex-start; gap:10px; font-size:13px; cursor:pointer;">
                                        <input id="check-office-viewer" type="checkbox" {checked_office_viewer} style="margin-top:2px; accent-color:#228be6;" />
                                        <div>
                                            <div style="font-weight:600; color:#f1f5f9;">Office- &amp; Dokumenten-Viewer</div>
                                            <div style="font-size:11px; color:#828282;">Ermöglicht das Öffnen von Word-, PowerPoint- &amp; Excel-Dateien mit 1 Klick in Ihrer lokalen Office-App oder OnlyOffice.</div>
                                        </div>
                                    </label>
                                </div>

                                <label class="docmost-field-label">OnlyOffice Document Server URL (optional)</label>
                                <input id="input-onlyoffice-url" class="docmost-input" type="text" value="{onlyoffice_url}" placeholder="https://onlyoffice.meinedomain.de" style="margin-bottom:6px;" />
                                <div style="font-size:11px; color:#828282;">Falls Sie einen OnlyOffice Document Server betreiben, können Dokumente direkt im Viewer eingebettet bearbeitet werden.</div>
                            </div>

                            <!-- Tab 3: Updates -->
                            <div id="pane-updates" class="docmost-tab-pane">
                                <div style="margin-bottom:14px;">
                                    <label style="display:flex; align-items:center; gap:8px; font-size:13px; cursor:pointer;">
                                        <input id="check-auto-update" type="checkbox" {checked_auto_up} style="accent-color:#228be6;" />
                                        <span>Automatisch beim Start nach Updates suchen</span>
                                    </label>
                                </div>

                                <div class="docmost-surface-card" style="padding:12px; font-size:12px; color:#828282; margin-bottom:14px;">
                                    <div>Installierte Version: <strong style="color:#f1f5f9;">v{version}</strong></div>
                                    <div id="update-status-text" style="margin-top:6px; color:#12b886;">✓ Status: Bereit</div>
                                </div>

                                <div style="display:flex; gap:8px;">
                                    <button class="docmost-btn-secondary" onclick="window.checkUpdatesManually()" style="flex:1;">Jetzt nach Updates suchen</button>
                                    <button class="docmost-btn-secondary" onclick="window.testUpdateModalPreview()" style="flex:1;" title="Öffnet das Update-Fenster als Vorschau">Fenster-Vorschau testen</button>
                                </div>
                            </div>
                        </div>

                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.closeSettingsModal()">Abbrechen</button>
                            <button class="docmost-btn-primary" onclick="window.saveSettings()">Speichern &amp; Übernehmen</button>
                        </div>
                    </div>
                </div>

                <!-- Add Server Modal -->
                <div id="docmost-add-server-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box" style="width: 420px;">
                        <div class="docmost-modal-header">
                            <h3 style="margin:0; font-size: 15px; font-weight: 600;">Neuen Server hinzufügen</h3>
                            <span class="docmost-modal-close" onclick="window.closeAddServerModal()">✕</span>
                        </div>
                        <div class="docmost-modal-body">
                            <label class="docmost-field-label">Name / Bezeichnung</label>
                            <input id="input-new-server-name" class="docmost-input" type="text" placeholder="z. B. Arbeit, Wiki, Test" style="margin-bottom:14px;" />

                            <label class="docmost-field-label">Docmost Server URL</label>
                            <input id="input-new-server-url" class="docmost-input" type="text" placeholder="https://wiki.firma.de" />
                        </div>
                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.closeAddServerModal()">Abbrechen</button>
                            <button class="docmost-btn-primary" onclick="window.submitAddServer()">Hinzufügen &amp; Wechseln</button>
                        </div>
                    </div>
                </div>

                <!-- Update Available Modal -->
                <div id="docmost-update-modal" class="docmost-modal-overlay">
                    <div class="docmost-modal-box">
                        <div class="docmost-modal-header">
                            <h3 style="margin:0; font-size: 15px; font-weight: 600; color:#f1f5f9;">🚀 Neues Update verfügbar</h3>
                            <span class="docmost-modal-close" onclick="window.closeUpdateModal()">✕</span>
                        </div>
                        <div class="docmost-modal-body">
                            <div style="font-size:15px; font-weight:700; color:#f1f5f9; margin-bottom:4px;" id="update-modal-title">Docmost Desktop Update</div>
                            <div style="font-size:12px; color:#828282; margin-bottom:12px;">Eine neue Version steht auf GitHub bereit und kann mit einem Klick installiert werden.</div>
                            <div id="update-modal-changelog" class="docmost-surface-card" style="padding:12px; font-size:12px; max-height:160px; overflow-y:auto; white-space:pre-wrap; color:#c9c9c9; font-family: monospace;"></div>
                            <div id="update-progress-box" style="display:none; margin-top:14px;">
                                <div style="font-size:12px; color:#74c0fc; margin-bottom:6px; font-weight:600;">Lade Update herunter und starte Installer...</div>
                                <div style="background:#141414; height:8px; border-radius:4px; overflow:hidden; border:1px solid #2e2e2e;">
                                    <div id="update-progress-bar" style="background:#228be6; width:100%; height:100%;"></div>
                                </div>
                            </div>
                        </div>
                        <div class="docmost-modal-footer">
                            <button class="docmost-btn-secondary" onclick="window.closeUpdateModal()">Später</button>
                            <button id="btn-do-update" class="docmost-btn-primary" onclick="window.performUpdate()">Jetzt herunterladen &amp; installieren</button>
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
                            <span class="docmost-modal-close" onclick="window.closeMediaPlayer()">✕</span>
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
                            <span class="docmost-modal-close" onclick="window.closeOfficeModal()">✕</span>
                        </div>
                        <div class="docmost-office-body">
                            <iframe id="docmost-office-iframe" style="display:none;"></iframe>
                            <div id="docmost-office-fallback-card" class="docmost-office-fallback-card">
                                <div id="office-fallback-icon" style="font-size:48px;">📄</div>
                                <div>
                                    <div id="office-fallback-filename" style="font-size:16px; font-weight:700; color:#f1f5f9; margin-bottom:6px;">Dokument</div>
                                    <div style="font-size:13px; color:#828282; max-width:440px; line-height:1.4;">
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
            bindServerContainerEvents(document.getElementById('settings-server-list'));

            const urlInput = document.getElementById('input-new-server-url');
            if (urlInput) {{
                urlInput.addEventListener('keydown', (e) => {{
                    if (e.key === 'Enter') window.submitAddServer();
                }});
            }}

            const zoomSelect = document.getElementById('select-ui-zoom');
            if (zoomSelect) {{
                zoomSelect.value = String(window.__docmostConfig.zoomLevel || 1.0);
            }}
            modalsCreated = true;
        }}

        let observer = null;
        let observedTarget = null;
        let uiScanScheduled = false;

        // Master UI Synchronizer (lightweight, lazy-first)
        function ensureDesktopUI() {{
            uiScanScheduled = false;
            try {{
                injectStyles();
                applyZoom();

                if (!document.body) return;

                if (!document.body.classList.contains('has-desktop-menu')) {{
                    document.body.classList.add('has-desktop-menu');
                }}

                if (!document.getElementById('docmost-app-menu-btn')) {{
                    createMenuTriggerButton();
                }}

                if (observer && observedTarget !== document.body) {{
                    observer.disconnect();
                    observer.observe(document.body, {{ childList: true, subtree: true }});
                    observedTarget = document.body;
                }}

                if (window.__pendingUpdateInfo) {{
                    const dot = document.getElementById('docmost-menu-update-dot');
                    if (dot) dot.style.display = 'block';
                }}
                if (!window.__hasCheckedUpdateOnInit && window.ipc) {{
                    window.__hasCheckedUpdateOnInit = true;
                    window.ipc.postMessage(JSON.stringify({{ action: 'request_config' }}));
                    window.ipc.postMessage(JSON.stringify({{ action: 'check_pending_update' }}));
                }}

                checkExcalidrawPresence();
            }} catch (err) {{
                console.error("[Docmost Desktop] ensureDesktopUI error:", err);
            }}
        }}

        function scheduleDesktopCheck() {{
            if (uiScanScheduled) return;
            uiScanScheduled = true;
            if (typeof window.requestIdleCallback === 'function') {{
                window.requestIdleCallback(() => ensureDesktopUI(), {{ timeout: 300 }});
            }} else {{
                setTimeout(ensureDesktopUI, 250);
            }}
        }}

        // Trigger on initial phases
        ensureDesktopUI();

        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', ensureDesktopUI, {{ once: true }});
        }}
        window.addEventListener('load', ensureDesktopUI, {{ once: true }});

        // Coalesced observer that skips irrelevant scans during rapid typing
        try {{
            observer = new MutationObserver((mutations) => {{
                if (document.body && !document.getElementById('docmost-app-menu-btn')) {{
                    ensureDesktopUI();
                    return;
                }}
                if (uiScanScheduled) return;
                for (let i = 0; i < mutations.length; i++) {{
                    const t = mutations[i].target;
                    if (!t || (t.id !== 'docmost-app-menu-btn' && t.id !== 'docmost-desktop-modals' && !t.closest?.('#docmost-desktop-modals, #docmost-app-menu-btn, #docmost-app-dropdown-menu, #docmost-pen-toolbar'))) {{
                        scheduleDesktopCheck();
                        return;
                    }}
                }}
            }});
            const target = document.body || document.documentElement;
            if (target) {{
                observer.observe(target, {{ childList: true, subtree: true }});
                observedTarget = target;
            }}
        }} catch (obsErr) {{
            console.error("[Docmost Desktop] MutationObserver error:", obsErr);
        }}

    }} catch (fatalErr) {{
        console.error("[Docmost Desktop] Global Init Fatal Error:", fatalErr);
    }}
}})();
    "#,
    css = css_global_js,
    modal_css = css_modals_js,
    excalidraw_css = css_excalidraw_js,
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
            background: #141414;
            color: #c9c9c9;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
        }}
        .card {{
            background: #1f1f1f;
            border: 1px solid #383838;
            border-radius: 10px;
            padding: 36px;
            width: 480px;
            max-width: 90vw;
            box-shadow: 0 20px 48px rgba(0, 0, 0, 0.75);
            text-align: center;
        }}
        .logo {{
            width: 56px;
            height: 56px;
            background: linear-gradient(135deg, #228be6, #7950f2);
            border-radius: 12px;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            font-size: 28px;
            font-weight: 800;
            color: #fff;
            margin-bottom: 18px;
            box-shadow: 0 8px 20px rgba(34, 139, 230, 0.35);
        }}
        h1 {{
            font-size: 22px;
            font-weight: 700;
            margin: 0 0 6px 0;
            color: #f1f5f9;
        }}
        p.subtitle {{
            color: #828282;
            font-size: 13px;
            margin: 0 0 24px 0;
        }}
        .input-group {{
            text-align: left;
            margin-bottom: 18px;
        }}
        label {{
            font-size: 12px;
            font-weight: 500;
            color: #b8b8b8;
            margin-bottom: 6px;
            display: block;
        }}
        input[type="text"] {{
            width: 100%;
            box-sizing: border-box;
            background: #141414;
            border: 1px solid #383838;
            border-radius: 6px;
            padding: 10px 12px;
            color: #f1f5f9;
            font-size: 13px;
            transition: border-color 0.15s;
        }}
        input[type="text"]:focus {{
            outline: none;
            border-color: #228be6;
        }}
        .btn-connect {{
            width: 100%;
            background: #228be6;
            color: #fff;
            border: none;
            border-radius: 6px;
            padding: 11px;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
            transition: background-color 0.15s, transform 0.1s;
        }}
        .btn-connect:hover {{
            background: #1c7ed6;
        }}
        .btn-connect:active {{
            transform: scale(0.99);
        }}
        .footer-info {{
            margin-top: 22px;
            font-size: 12px;
            color: #828282;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .btn-link {{
            background: none;
            border: none;
            color: #339af0;
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
    css = compact_css(CLEAN_DARK_GLOBAL_CSS),
    error_html = err_block,
    url = server_url,
    version = current_version
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_css_strips_comments_and_whitespace() {
        let raw = "/* Header */\nbody {\n    color: #fff;\n    background: #141414;\n}\n/* End */";
        let compacted = compact_css(raw);
        assert_eq!(compacted, "body { color: #fff; background: #141414; }");
        assert!(!compacted.contains("/*"));
    }

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
        assert!(script.contains("onDesktopConfigSync"));
        assert!(script.contains("bindServerContainerEvents"));
        assert!(script.contains("goHome"));
        assert!(script.contains("docmost-media-modal"));
        assert!(script.contains("docmost-office-modal"));
        assert!(script.contains("openMediaPlayer"));
        assert!(script.contains("openOfficeModal"));
        assert!(script.contains("docmost-desktop-modal-style"));
        assert!(script.contains("docmost-desktop-excalidraw-style"));
        assert!(script.contains("injectModalStyles"));
        assert!(script.contains("injectExcalidrawStyles"));
    }
}
