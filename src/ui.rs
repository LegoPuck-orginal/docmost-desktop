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
"#;

pub fn get_injection_script(config: &crate::config::AppConfig, current_version: &str) -> String {
    format!(r#"
(function() {{
    if (window.__docmostDesktopInitialized) return;
    window.__docmostDesktopInitialized = true;

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
        version: "{version}"
    }};

    // Inject CSS
    const style = document.createElement('style');
    style.innerHTML = `{css}`;
    document.head.appendChild(style);

    // Apply Zoom
    if (window.__docmostConfig.zoomLevel && window.__docmostConfig.zoomLevel !== 1.0) {{
        document.documentElement.style.zoom = window.__docmostConfig.zoomLevel;
    }}

    // Auto-sync Excalidraw Dark Mode & Pen Mode
    const syncExcalidrawState = () => {{
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
    syncExcalidrawState();

    const ensureExcalidrawDark = () => {{
        if (window.__docmostConfig.darkTheme) {{
            document.querySelectorAll('.excalidraw').forEach(el => {{
                if (!el.classList.contains('theme--dark')) {{
                    el.classList.add('theme--dark');
                }}
            }});
        }}
    }};

    // Observes Excalidraw appearance & updates Pen Toolbar
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

    const excalidrawObserver = new MutationObserver(() => {{
        checkExcalidrawPresence();
    }});
    excalidrawObserver.observe(document.documentElement, {{ childList: true, subtree: true }});
    checkExcalidrawPresence();

    // Stylus / Palm Rejection Logic
    let lastPenContactTime = 0;
    window.addEventListener('pointerdown', (e) => {{
        if (e.pointerType === 'pen') {{
            lastPenContactTime = Date.now();
            const badge = document.getElementById('badge-palm-rejection');
            if (badge) badge.style.opacity = '1';
        }} else if (e.pointerType === 'touch' && window.__docmostConfig.palmRejection) {{
            const onCanvas = e.target.closest('.excalidraw canvas') || e.target.closest('.excalidraw');
            // Wenn in den letzten 1200ms ein Stift aktiv war oder PenMode aktiv ist:
            // Unterdrücke versehentliche Handballen-Touch-Zeichnungen
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

    // Verhindere Windows-Stift Rechtsklick-Menü beim längeren Aufdrücken auf der Leinwand
    window.addEventListener('contextmenu', (e) => {{
        if (e.target.closest('.excalidraw canvas') || e.target.closest('.excalidraw')) {{
            e.preventDefault();
        }}
    }}, true);

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
            <button class="docmost-nav-btn" onclick="openSettingsModal()" title="Einstellungen, Stift & Updates">⚙</button>
        </div>
    `;

    // Wait for body to append nav and UI tools
    const appendNav = () => {{
        if (document.body && !document.getElementById('docmost-desktop-nav')) {{
            document.body.classList.add('has-desktop-nav');
            document.body.prepend(nav);
            createModals();
            createPenToolbar();
        }}
    }};

    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', appendNav);
    }} else {{
        appendNav();
    }}

    // Pen Toolbar
    function createPenToolbar() {{
        if (document.getElementById('docmost-pen-toolbar')) return;

        const tb = document.createElement('div');
        tb.id = 'docmost-pen-toolbar';
        tb.innerHTML = `
            <button id="tb-btn-pen" class="docmost-pen-btn active" onclick="activatePenTool('freedraw')" title="Freihand-Stift (P / 2)">
                <span>🖊️</span><span>Stift</span>
            </button>
            <button id="tb-btn-eraser" class="docmost-pen-btn" onclick="activatePenTool('eraser')" title="Radierer (E / 0)">
                <span>🧹</span><span>Radierer</span>
            </button>
            <div style="width:1px; height:20px; background:#28314a; margin:0 2px;"></div>
            <!-- Quick Colors -->
            <button class="docmost-color-dot" style="background:#ffffff;" onclick="setPenColor('#ffffff')" title="Weiß"></button>
            <button class="docmost-color-dot" style="background:#818cf8;" onclick="setPenColor('#818cf8')" title="Indigo"></button>
            <button class="docmost-color-dot" style="background:#34d399;" onclick="setPenColor('#34d399')" title="Smaragdgrün"></button>
            <button class="docmost-color-dot" style="background:#fbbf24;" onclick="setPenColor('#fbbf24')" title="Sonnengelb"></button>
            <button class="docmost-color-dot" style="background:#f87171;" onclick="setPenColor('#f87171')" title="Korallenrot"></button>
            <div style="width:1px; height:20px; background:#28314a; margin:0 2px;"></div>
            <!-- Stroke Widths -->
            <button class="docmost-pen-stroke-btn" onclick="setPenStroke(1)" title="Feine Strichstärke">―</button>
            <button class="docmost-pen-stroke-btn active" onclick="setPenStroke(2)" title="Mittlere Strichstärke">━</button>
            <button class="docmost-pen-stroke-btn" onclick="setPenStroke(4)" title="Breite Strichstärke">█</button>
            <div style="width:1px; height:20px; background:#28314a; margin:0 2px;"></div>
            <!-- Palm Rejection Badge -->
            <div id="badge-palm-rejection" style="font-size:11px; color:#10b981; display:flex; align-items:center; gap:4px; padding:3px 8px; background:rgba(16,185,129,0.12); border-radius:12px;" title="Handballenschutz & Stiftmodus aktiv">
                <span>🖐️</span><span>Stift aktiv</span>
            </div>
            <!-- Undo / Redo -->
            <button class="docmost-pen-btn" onclick="penUndo()" title="Rückgängig (Strg+Z)">↩</button>
            <button class="docmost-pen-btn" onclick="penRedo()" title="Wiederholen (Strg+Y)">↪</button>
            <button class="docmost-pen-btn" style="padding:4px 8px; color:#94a3b8;" onclick="document.getElementById('docmost-pen-toolbar').style.display='none'" title="Leiste ausblenden">✕</button>
        `;
        document.body.appendChild(tb);
    }}

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
        // Fallback: update excalidraw-state in localStorage
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
        if (event && event.currentTarget) event.currentTarget.classList.add('active');
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

    // Modals
    function createModals() {{
        if (document.getElementById('docmost-settings-modal')) return;

        const modalContainer = document.createElement('div');
        modalContainer.innerHTML = `
            <!-- Tabbed Settings Modal -->
            <div id="docmost-settings-modal" class="docmost-modal-overlay">
                <div class="docmost-modal-box" style="width: 540px;">
                    <div class="docmost-modal-header">
                        <h3 style="margin:0; font-size: 16px;">Docmost Desktop Einstellungen</h3>
                        <span style="cursor:pointer; color:#94a3b8; font-size:18px; padding:0 4px;" onclick="closeSettingsModal()">✕</span>
                    </div>

                    <!-- Tabs Header -->
                    <div class="docmost-tabs-header">
                        <button id="tab-btn-general" class="docmost-tab-btn active" onclick="switchSettingsTab('general')">⚙ Allgemein</button>
                        <button id="tab-btn-pen" class="docmost-tab-btn" onclick="switchSettingsTab('pen')">🖊️ Stift & Excalidraw</button>
                        <button id="tab-btn-updates" class="docmost-tab-btn" onclick="switchSettingsTab('updates')">🔄 Updates</button>
                    </div>

                    <div class="docmost-modal-body" style="padding: 18px 20px;">
                        <!-- Tab 1: Allgemein -->
                        <div id="pane-general" class="docmost-tab-pane active">
                            <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">Docmost Server URL</label>
                            <input id="input-server-url" type="text" value="{url}" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:14px;" />
                            
                            <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">GitHub Repository (für Updates)</label>
                            <input id="input-github-repo" type="text" value="{repo}" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:14px;" />
                            
                            <label style="font-size:12px; color:#94a3b8; display:block; margin-bottom:6px;">Oberflächen-Skalierung (Zoom)</label>
                            <select id="select-ui-zoom" style="width:100%; box-sizing:border-box; background:#0e111a; border:1px solid #28314a; border-radius:6px; padding:8px 12px; color:#fff; font-size:13px; margin-bottom:6px;">
                                <option value="0.9">90% (Kompakt)</option>
                                <option value="1.0">100% (Standard)</option>
                                <option value="1.1">110% (Leicht vergrößert)</option>
                                <option value="1.25">125% (Ideal für Surface / Touch)</option>
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
                                <button class="docmost-btn-secondary" onclick="checkUpdatesManually()" style="flex:1;">Jetzt nach Updates suchen</button>
                                <button class="docmost-btn-secondary" onclick="testUpdateModalPreview()" style="flex:1;" title="Öffnet das Update-Fenster als Vorschau">Fenster-Vorschau testen</button>
                            </div>
                        </div>
                    </div>

                    <div class="docmost-modal-footer">
                        <button class="docmost-btn-secondary" onclick="closeSettingsModal()">Abbrechen</button>
                        <button class="docmost-btn-primary" onclick="saveSettings()">Speichern & Übernehmen</button>
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

        // Pre-select Zoom in dropdown
        const zoomSelect = document.getElementById('select-ui-zoom');
        if (zoomSelect) {{
            zoomSelect.value = String(window.__docmostConfig.zoomLevel || 1.0);
        }}

        // Falls bereits ein Update empfangen wurde, sofort anzeigen
        if (window.__pendingUpdateInfo) {{
            window.onUpdateAvailable(window.__pendingUpdateInfo);
        }}
        // Backend nach anstehendem Update abfragen
        if (window.ipc) {{
            window.ipc.postMessage(JSON.stringify({{ action: 'check_pending_update' }}));
        }}
    }}

    window.switchSettingsTab = function(tabName) {{
        document.querySelectorAll('.docmost-tab-btn').forEach(b => b.classList.remove('active'));
        document.querySelectorAll('.docmost-tab-pane').forEach(p => p.classList.remove('active'));

        const btn = document.getElementById('tab-btn-' + tabName);
        const pane = document.getElementById('pane-' + tabName);
        if (btn) btn.classList.add('active');
        if (pane) pane.classList.add('active');
    }};

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
            changelog: '• Dies ist eine Vorschau des automatischen Update-Fensters!\n• Clean Dark Design & Stift-Unterstützung aktiv.\n• Sobald eine neue Version auf GitHub existiert, öffnet sich dieses Fenster vollautomatisch beim Start.',
            download_url: ''
        }});
    }};

    window.saveSettings = function() {{
        const urlInput = document.getElementById('input-server-url');
        const repoInput = document.getElementById('input-github-repo');
        const autoUpInput = document.getElementById('check-auto-update');
        const penModeInput = document.getElementById('check-pen-mode');
        const palmRejInput = document.getElementById('check-palm-rejection');
        const quickTbInput = document.getElementById('check-quick-toolbar');
        const darkThemeInput = document.getElementById('check-excalidraw-dark');
        const zoomSelect = document.getElementById('select-ui-zoom');

        const newUrl = urlInput ? urlInput.value.trim() : '';
        const newRepo = repoInput ? repoInput.value.trim() : '';
        const autoUp = autoUpInput ? autoUpInput.checked : true;
        const penMode = penModeInput ? penModeInput.checked : true;
        const palmRejection = palmRejInput ? palmRejInput.checked : true;
        const quickToolbar = quickTbInput ? quickTbInput.checked : true;
        const darkTheme = darkThemeInput ? darkThemeInput.checked : true;
        const zoomLevel = zoomSelect ? parseFloat(zoomSelect.value) : 1.0;

        window.__docmostConfig.serverUrl = newUrl;
        window.__docmostConfig.githubRepo = newRepo;
        window.__docmostConfig.autoUpdate = autoUp;
        window.__docmostConfig.penMode = penMode;
        window.__docmostConfig.palmRejection = palmRejection;
        window.__docmostConfig.quickToolbar = quickToolbar;
        window.__docmostConfig.darkTheme = darkTheme;
        window.__docmostConfig.zoomLevel = zoomLevel;

        // Apply zoom immediately
        document.documentElement.style.zoom = zoomLevel;
        checkExcalidrawPresence();
        syncExcalidrawState();

        if (window.ipc) {{
            window.ipc.postMessage(JSON.stringify({{
                action: 'save_settings',
                server_url: newUrl,
                github_repo: newRepo,
                check_updates_on_startup: autoUp,
                excalidraw_pen_mode: penMode,
                excalidraw_palm_rejection: palmRejection,
                excalidraw_quick_toolbar: quickToolbar,
                excalidraw_dark_theme: darkTheme,
                ui_zoom_level: zoomLevel
            }}));
        }}
        closeSettingsModal();
        if (newUrl && newUrl !== window.location.href && newUrl !== window.__docmostConfig.serverUrl) {{
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
    url = config.server_url,
    url_display = config.server_url.replace("https://", "").replace("http://", ""),
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
    checked_auto_up = if config.check_updates_on_startup { "checked" } else { "" }
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
