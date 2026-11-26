import 'package:flutter/material.dart';

import '../models/app_config.dart';
import '../theme/docmost_theme.dart';

/// Floating 3-line top-right trigger matching Docmost Mantine Dark UI.
class AppMenuTriggerButton extends StatelessWidget {
  const AppMenuTriggerButton({
    super.key,
    required this.isOpen,
    required this.activeServerName,
    required this.onTap,
  });

  final bool isOpen;
  final String activeServerName;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Tooltip(
      message: 'Menü • $activeServerName',
      child: Material(
        color: isOpen ? DocmostTheme.bgHover : DocmostTheme.bgCard,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(6),
          side: BorderSide(
            color: isOpen ? DocmostTheme.borderHover : DocmostTheme.border,
          ),
        ),
        elevation: 6,
        shadowColor: const Color(0x80000000),
        child: InkWell(
          key: const Key('docmost-app-menu-btn'),
          onTap: onTap,
          borderRadius: BorderRadius.circular(6),
          child: SizedBox(
            width: 34,
            height: 34,
            child: Icon(
              Icons.menu,
              size: 18,
              color: isOpen ? Colors.white : DocmostTheme.textBody,
            ),
          ),
        ),
      ),
    );
  }
}

/// Top-right Mantine Dark dropdown menu overlay with navigation, multi-server switch and quick actions.
class AppMenuDropdownOverlay extends StatelessWidget {
  const AppMenuDropdownOverlay({
    super.key,
    required this.config,
    required this.onClose,
    required this.onGoBack,
    required this.onGoForward,
    required this.onReload,
    required this.onGoHome,
    required this.onSelectServer,
    required this.onDeleteServer,
    required this.onAddServer,
    required this.onOpenSettingsTab,
    required this.onOpenInSafari,
  });

  final AppConfig config;
  final VoidCallback onClose;
  final VoidCallback onGoBack;
  final VoidCallback onGoForward;
  final VoidCallback onReload;
  final VoidCallback onGoHome;
  final ValueChanged<String> onSelectServer;
  final ValueChanged<String> onDeleteServer;
  final VoidCallback onAddServer;
  final ValueChanged<int> onOpenSettingsTab;
  final VoidCallback onOpenInSafari;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: <Widget>[
        Positioned.fill(
          child: GestureDetector(
            behavior: HitTestBehavior.translucent,
            onTap: onClose,
          ),
        ),
        Positioned(
          top: 48,
          right: 12,
          child: Material(
            color: Colors.transparent,
            child: Container(
              key: const Key('docmost-app-dropdown-menu'),
              width: 300,
              constraints: BoxConstraints(
                maxWidth: MediaQuery.sizeOf(context).width - 24,
                maxHeight: MediaQuery.sizeOf(context).height * 0.78,
              ),
              padding: const EdgeInsets.all(6),
              decoration: BoxDecoration(
                color: DocmostTheme.bgElevated,
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: DocmostTheme.border),
                boxShadow: const <BoxShadow>[
                  BoxShadow(
                    color: Color(0xB3000000),
                    blurRadius: 28,
                    offset: Offset(0, 12),
                  ),
                ],
              ),
              child: SingleChildScrollView(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: <Widget>[
                    _buildNavRow(),
                    const Divider(height: 9, color: DocmostTheme.borderSubtle),
                    _buildServersHeader(),
                    const SizedBox(height: 4),
                    ConstrainedBox(
                      constraints: const BoxConstraints(maxHeight: 190),
                      child: ListView.builder(
                        shrinkWrap: true,
                        itemCount: config.servers.length,
                        itemBuilder: (context, index) {
                          final srv = config.servers[index];
                          return _buildServerTile(srv);
                        },
                      ),
                    ),
                    const Divider(height: 9, color: DocmostTheme.borderSubtle),
                    _MenuActionItem(
                      icon: Icons.draw_outlined,
                      label: 'Stift & Excalidraw',
                      onTap: () {
                        onClose();
                        onOpenSettingsTab(1);
                      },
                    ),
                    _MenuActionItem(
                      icon: Icons.description_outlined,
                      label: 'Office & Medien-Viewer',
                      onTap: () {
                        onClose();
                        onOpenSettingsTab(2);
                      },
                    ),
                    _MenuActionItem(
                      icon: Icons.settings_outlined,
                      label: 'Alle Einstellungen...',
                      onTap: () {
                        onClose();
                        onOpenSettingsTab(0);
                      },
                    ),
                    const Divider(height: 9, color: DocmostTheme.borderSubtle),
                    _MenuActionItem(
                      icon: Icons.open_in_browser,
                      label: 'In Safari öffnen',
                      onTap: () {
                        onClose();
                        onOpenInSafari();
                      },
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildNavRow() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(4, 2, 4, 4),
      child: Row(
        children: <Widget>[
          _NavIconBtn(
            icon: Icons.arrow_back,
            tooltip: 'Zurück',
            onTap: () {
              onClose();
              onGoBack();
            },
          ),
          const SizedBox(width: 4),
          _NavIconBtn(
            icon: Icons.arrow_forward,
            tooltip: 'Vorwärts',
            onTap: () {
              onClose();
              onGoForward();
            },
          ),
          const SizedBox(width: 4),
          _NavIconBtn(
            icon: Icons.refresh,
            tooltip: 'Neu laden',
            onTap: () {
              onClose();
              onReload();
            },
          ),
          const SizedBox(width: 4),
          _NavIconBtn(
            icon: Icons.home_outlined,
            tooltip: 'Startseite',
            onTap: () {
              onClose();
              onGoHome();
            },
          ),
        ],
      ),
    );
  }

  Widget _buildServersHeader() {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      child: Row(
        children: <Widget>[
          const Expanded(
            child: Text(
              'SERVER',
              style: TextStyle(
                fontSize: 11,
                fontWeight: FontWeight.w700,
                letterSpacing: 0.5,
                color: DocmostTheme.textMuted,
              ),
            ),
          ),
          InkWell(
            key: const Key('menu-add-server-btn'),
            onTap: () {
              onClose();
              onAddServer();
            },
            borderRadius: BorderRadius.circular(4),
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
              decoration: BoxDecoration(
                color: DocmostTheme.blue,
                borderRadius: BorderRadius.circular(4),
              ),
              child: const Text(
                '+ Neu',
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 11,
                  fontWeight: FontWeight.w600,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildServerTile(ServerEntry srv) {
    final isActive = srv.id == config.activeServerId;
    return InkWell(
      onTap: () {
        onClose();
        onSelectServer(srv.id);
      },
      borderRadius: BorderRadius.circular(4),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 7),
        decoration: BoxDecoration(
          color: isActive ? DocmostTheme.blueTint : Colors.transparent,
          borderRadius: BorderRadius.circular(4),
        ),
        child: Row(
          children: <Widget>[
            Container(
              width: 7,
              height: 7,
              decoration: BoxDecoration(
                shape: BoxShape.circle,
                color: isActive ? DocmostTheme.teal : DocmostTheme.textMuted,
              ),
            ),
            const SizedBox(width: 8),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: <Widget>[
                  Row(
                    children: <Widget>[
                      Flexible(
                        child: Text(
                          srv.name,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            fontSize: 13,
                            fontWeight: FontWeight.w600,
                            color: isActive
                                ? Colors.white
                                : DocmostTheme.textBody,
                          ),
                        ),
                      ),
                      if (isActive) ...<Widget>[
                        const SizedBox(width: 5),
                        const Text(
                          '(Aktiv)',
                          style: TextStyle(
                            fontSize: 10,
                            color: DocmostTheme.blueLight,
                          ),
                        ),
                      ],
                    ],
                  ),
                  Text(
                    srv.displayHost,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(
                      fontSize: 11,
                      color: DocmostTheme.textMuted,
                    ),
                  ),
                ],
              ),
            ),
            if (config.servers.length > 1)
              InkWell(
                onTap: () => onDeleteServer(srv.id),
                borderRadius: BorderRadius.circular(4),
                child: const Padding(
                  padding: EdgeInsets.all(4),
                  child: Icon(
                    Icons.close,
                    size: 14,
                    color: DocmostTheme.textMuted,
                  ),
                ),
              ),
          ],
        ),
      ),
    );
  }
}

class _NavIconBtn extends StatelessWidget {
  const _NavIconBtn({
    required this.icon,
    required this.tooltip,
    required this.onTap,
  });

  final IconData icon;
  final String tooltip;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Tooltip(
        message: tooltip,
        child: Material(
          color: DocmostTheme.bgCard,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(5),
            side: const BorderSide(color: DocmostTheme.borderSubtle),
          ),
          child: InkWell(
            onTap: onTap,
            borderRadius: BorderRadius.circular(5),
            child: SizedBox(
              height: 32,
              child: Icon(icon, size: 16, color: DocmostTheme.textBody),
            ),
          ),
        ),
      ),
    );
  }
}

class _MenuActionItem extends StatelessWidget {
  const _MenuActionItem({
    required this.icon,
    required this.label,
    required this.onTap,
  });

  final IconData icon;
  final String label;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(4),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
        child: Row(
          children: <Widget>[
            Icon(icon, size: 16, color: DocmostTheme.textBody),
            const SizedBox(width: 9),
            Expanded(
              child: Text(
                label,
                style: const TextStyle(
                  fontSize: 13,
                  fontWeight: FontWeight.w500,
                  color: DocmostTheme.textBody,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
