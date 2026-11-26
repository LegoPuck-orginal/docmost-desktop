import 'package:flutter/material.dart';

import '../models/app_config.dart';
import '../theme/docmost_theme.dart';

class SettingsModalResult {
  const SettingsModalResult({
    required this.config,
    required this.serverChanged,
  });

  final AppConfig config;
  final bool serverChanged;
}

class SettingsModal extends StatefulWidget {
  const SettingsModal({
    super.key,
    required this.initialConfig,
    this.initialTabIndex = 0,
    required this.onAddServerRequest,
  });

  final AppConfig initialConfig;
  final int initialTabIndex;
  final Future<AddServerModalData?> Function() onAddServerRequest;

  @override
  State<SettingsModal> createState() => _SettingsModalState();
}

class AddServerModalData {
  const AddServerModalData({required this.name, required this.url});

  final String name;
  final String url;
}

class _SettingsModalState extends State<SettingsModal> {
  late int _tabIndex;
  late AppConfig _draft;
  late String _initialServerUrl;
  late TextEditingController _onlyofficeController;

  @override
  void initState() {
    super.initState();
    _tabIndex = widget.initialTabIndex.clamp(0, 2);
    _draft = widget.initialConfig;
    _initialServerUrl = widget.initialConfig.serverUrl;
    _onlyofficeController =
        TextEditingController(text: widget.initialConfig.onlyofficeUrl);
  }

  @override
  void dispose() {
    _onlyofficeController.dispose();
    super.dispose();
  }

  Future<void> _handleAddServer() async {
    final added = await widget.onAddServerRequest();
    if (added == null || !mounted) return;
    setState(() {
      _draft = _draft.withAddedServer(name: added.name, rawUrl: added.url);
    });
  }

  void _saveAndClose() {
    final finalConfig = _draft.copyWith(
      onlyofficeUrl: _onlyofficeController.text.trim(),
    );
    Navigator.of(context).pop(
      SettingsModalResult(
        config: finalConfig,
        serverChanged: finalConfig.serverUrl != _initialServerUrl,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      backgroundColor: Colors.transparent,
      insetPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 20),
      child: Container(
        constraints: BoxConstraints(
          maxWidth: 540,
          maxHeight: MediaQuery.sizeOf(context).height * 0.86,
        ),
        decoration: BoxDecoration(
          color: DocmostTheme.bgCard,
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: DocmostTheme.border),
          boxShadow: const <BoxShadow>[
            BoxShadow(
              color: Color(0xCC000000),
              blurRadius: 40,
              offset: Offset(0, 18),
            ),
          ],
        ),
        clipBehavior: Clip.antiAlias,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            _buildHeader(),
            _buildTabs(),
            Flexible(
              child: SingleChildScrollView(
                padding: const EdgeInsets.all(18),
                child: _buildActivePane(),
              ),
            ),
            _buildFooter(),
          ],
        ),
      ),
    );
  }

  Widget _buildHeader() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 14),
      decoration: const BoxDecoration(
        color: DocmostTheme.bgElevated,
        border: Border(bottom: BorderSide(color: DocmostTheme.borderSubtle)),
      ),
      child: Row(
        children: <Widget>[
          const Expanded(
            child: Text(
              'Docmost iOS Einstellungen',
              style: TextStyle(
                color: DocmostTheme.textPrimary,
                fontSize: 15,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
          InkWell(
            onTap: () => Navigator.of(context).pop(),
            borderRadius: BorderRadius.circular(4),
            child: const Padding(
              padding: EdgeInsets.all(4),
              child: Icon(Icons.close, size: 18, color: DocmostTheme.textMuted),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTabs() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10),
      decoration: const BoxDecoration(
        color: DocmostTheme.bgFooter,
        border: Border(bottom: BorderSide(color: DocmostTheme.borderSubtle)),
      ),
      child: Row(
        children: <Widget>[
          _TabButton(
            label: '⚙ Server & Zoom',
            active: _tabIndex == 0,
            onTap: () => setState(() => _tabIndex = 0),
          ),
          _TabButton(
            label: '🖊️ Apple Pencil',
            active: _tabIndex == 1,
            onTap: () => setState(() => _tabIndex = 1),
          ),
          _TabButton(
            label: '📄 Office & Medien',
            active: _tabIndex == 2,
            onTap: () => setState(() => _tabIndex = 2),
          ),
        ],
      ),
    );
  }

  Widget _buildActivePane() {
    switch (_tabIndex) {
      case 1:
        return _buildPenPane();
      case 2:
        return _buildOfficeMediaPane();
      case 0:
      default:
        return _buildGeneralPane();
    }
  }

  Widget _buildGeneralPane() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        Row(
          children: <Widget>[
            const Expanded(
              child: Text(
                'GESPEICHERTE SERVER',
                style: TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.w700,
                  letterSpacing: 0.4,
                  color: DocmostTheme.textMuted,
                ),
              ),
            ),
            InkWell(
              onTap: _handleAddServer,
              borderRadius: BorderRadius.circular(4),
              child: Container(
                padding:
                    const EdgeInsets.symmetric(horizontal: 9, vertical: 4),
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
        const SizedBox(height: 10),
        Container(
          decoration: BoxDecoration(
            color: DocmostTheme.bgLowest,
            borderRadius: BorderRadius.circular(6),
            border: Border.all(color: DocmostTheme.borderSubtle),
          ),
          clipBehavior: Clip.antiAlias,
          child: Column(
            children: _draft.servers.map((srv) {
              final isActive = srv.id == _draft.activeServerId;
              return Container(
                padding:
                    const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                decoration: BoxDecoration(
                  color:
                      isActive ? DocmostTheme.blueTint : Colors.transparent,
                  border: const Border(
                    bottom: BorderSide(color: DocmostTheme.borderSubtle),
                  ),
                ),
                child: Row(
                  children: <Widget>[
                    Container(
                      width: 7,
                      height: 7,
                      decoration: BoxDecoration(
                        shape: BoxShape.circle,
                        color: isActive
                            ? DocmostTheme.teal
                            : DocmostTheme.textMuted,
                      ),
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: <Widget>[
                          Text(
                            isActive ? '${srv.name} (Aktiv)' : srv.name,
                            style: const TextStyle(
                              fontSize: 13,
                              fontWeight: FontWeight.w600,
                              color: DocmostTheme.textPrimary,
                            ),
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
                    if (!isActive)
                      TextButton(
                        onPressed: () {
                          setState(() {
                            _draft = _draft.withActiveServer(srv.id);
                          });
                        },
                        style: TextButton.styleFrom(
                          backgroundColor: DocmostTheme.bgHover,
                          foregroundColor: DocmostTheme.textBody,
                          padding: const EdgeInsets.symmetric(
                            horizontal: 10,
                            vertical: 4,
                          ),
                          minimumSize: Size.zero,
                          tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                        ),
                        child: const Text(
                          'Wechseln',
                          style: TextStyle(fontSize: 11),
                        ),
                      ),
                    if (_draft.servers.length > 1) ...<Widget>[
                      const SizedBox(width: 4),
                      InkWell(
                        onTap: () {
                          setState(() {
                            _draft = _draft.withDeletedServer(srv.id);
                          });
                        },
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
                  ],
                ),
              );
            }).toList(),
          ),
        ),
        const SizedBox(height: 16),
        const Text(
          'Oberflächen-Skalierung (Zoom)',
          style: TextStyle(
            fontSize: 12,
            fontWeight: FontWeight.w500,
            color: DocmostTheme.textLabel,
          ),
        ),
        const SizedBox(height: 6),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          decoration: BoxDecoration(
            color: DocmostTheme.bgLowest,
            borderRadius: BorderRadius.circular(6),
            border: Border.all(color: DocmostTheme.border),
          ),
          child: DropdownButtonHideUnderline(
            child: DropdownButton<double>(
              value: const <double>[0.9, 1.0, 1.1, 1.25, 1.5]
                      .contains(_draft.uiZoomLevel)
                  ? _draft.uiZoomLevel
                  : 1.0,
              isExpanded: true,
              dropdownColor: DocmostTheme.bgElevated,
              style: const TextStyle(
                color: DocmostTheme.textPrimary,
                fontSize: 13,
              ),
              items: const <DropdownMenuItem<double>>[
                DropdownMenuItem<double>(value: 0.9, child: Text('90% (Kompakt)')),
                DropdownMenuItem<double>(value: 1.0, child: Text('100% (Standard)')),
                DropdownMenuItem<double>(value: 1.1, child: Text('110% (Etwas größer)')),
                DropdownMenuItem<double>(value: 1.25, child: Text('125% (Ideal für iPad)')),
                DropdownMenuItem<double>(value: 1.5, child: Text('150% (Sehr groß)')),
              ],
              onChanged: (val) {
                if (val == null) return;
                setState(() {
                  _draft = _draft.copyWith(uiZoomLevel: val);
                });
              },
            ),
          ),
        ),
        const SizedBox(height: 14),
        Container(
          padding: const EdgeInsets.all(10),
          decoration: BoxDecoration(
            color: DocmostTheme.bgLowest,
            borderRadius: BorderRadius.circular(6),
            border: Border.all(color: DocmostTheme.borderSubtle),
          ),
          child: const Row(
            children: <Widget>[
              Icon(Icons.info_outline, size: 15, color: DocmostTheme.blueLight),
              SizedBox(width: 8),
              Expanded(
                child: Text(
                  'Docmost iOS v$kAppVersion • Mantine Dark',
                  style: TextStyle(fontSize: 12, color: DocmostTheme.textMuted),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildPenPane() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        const Text(
          'APPLE PENCIL & EXCALIDRAW OPTIMIERUNG',
          style: TextStyle(
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 0.4,
            color: DocmostTheme.textMuted,
          ),
        ),
        const SizedBox(height: 12),
        _SwitchTile(
          title: 'Stift-Modus (Apple Pencil Only)',
          subtitle: 'Nur der Apple Pencil zeichnet; Finger scrollen oder zoomen.',
          value: _draft.excalidrawPenMode,
          onChanged: (v) =>
              setState(() => _draft = _draft.copyWith(excalidrawPenMode: v)),
        ),
        _SwitchTile(
          title: 'Handballen-Schutz (Palm Rejection)',
          subtitle: 'Ignoriert aufliegende Handflächen beim Zeichnen automatisch.',
          value: _draft.excalidrawPalmRejection,
          onChanged: (v) => setState(
            () => _draft = _draft.copyWith(excalidrawPalmRejection: v),
          ),
        ),
        _SwitchTile(
          title: 'Schwebende Schnellzugriffsleiste',
          subtitle: 'Ergonomische Leiste für Stift, Radierer, Farben & Stärken.',
          value: _draft.excalidrawQuickToolbar,
          onChanged: (v) => setState(
            () => _draft = _draft.copyWith(excalidrawQuickToolbar: v),
          ),
        ),
        _SwitchTile(
          title: 'Clean Dark für Excalidraw erzwingen',
          subtitle: 'Dunkler Zeichenhintergrund ohne blendendes Weiß.',
          value: _draft.excalidrawDarkTheme,
          onChanged: (v) => setState(
            () => _draft = _draft.copyWith(excalidrawDarkTheme: v),
          ),
        ),
      ],
    );
  }

  Widget _buildOfficeMediaPane() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        const Text(
          'ONLYOFFICE & INTEGRIERTER MEDIENPLAYER',
          style: TextStyle(
            fontSize: 12,
            fontWeight: FontWeight.w700,
            letterSpacing: 0.4,
            color: DocmostTheme.textMuted,
          ),
        ),
        const SizedBox(height: 12),
        _SwitchTile(
          title: 'Integrierter Video- & Audio-Player',
          subtitle: 'Spielt MP4, MOV, M4V, MP3, M4A & WAV direkt in der App ab.',
          value: _draft.mediaPlayerEnabled,
          onChanged: (v) =>
              setState(() => _draft = _draft.copyWith(mediaPlayerEnabled: v)),
        ),
        _SwitchTile(
          title: 'Office- & PDF-Dokumenten-Viewer',
          subtitle: 'Zeigt PDF, Word, PowerPoint & Excel via iOS QuickLook oder OnlyOffice.',
          value: _draft.officeViewerEnabled,
          onChanged: (v) =>
              setState(() => _draft = _draft.copyWith(officeViewerEnabled: v)),
        ),
        const SizedBox(height: 12),
        const Text(
          'OnlyOffice Document Server URL (optional)',
          style: TextStyle(
            fontSize: 12,
            fontWeight: FontWeight.w500,
            color: DocmostTheme.textLabel,
          ),
        ),
        const SizedBox(height: 6),
        TextField(
          controller: _onlyofficeController,
          keyboardType: TextInputType.url,
          autocorrect: false,
          style: const TextStyle(color: DocmostTheme.textPrimary, fontSize: 13),
          decoration: InputDecoration(
            hintText: 'https://onlyoffice.meinedomain.de',
            hintStyle:
                const TextStyle(color: DocmostTheme.textMuted, fontSize: 13),
            filled: true,
            fillColor: DocmostTheme.bgLowest,
            contentPadding:
                const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
            enabledBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(6),
              borderSide: const BorderSide(color: DocmostTheme.border),
            ),
            focusedBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(6),
              borderSide: const BorderSide(color: DocmostTheme.blue),
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildFooter() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 12),
      decoration: const BoxDecoration(
        color: DocmostTheme.bgFooter,
        border: Border(top: BorderSide(color: DocmostTheme.borderSubtle)),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.end,
        children: <Widget>[
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            style: TextButton.styleFrom(
              backgroundColor: DocmostTheme.bgHover,
              foregroundColor: DocmostTheme.textBody,
              padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(6),
                side: const BorderSide(color: DocmostTheme.border),
              ),
            ),
            child: const Text('Abbrechen'),
          ),
          const SizedBox(width: 8),
          ElevatedButton(
            key: const Key('settings-save-btn'),
            onPressed: _saveAndClose,
            style: ElevatedButton.styleFrom(
              backgroundColor: DocmostTheme.blue,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(6),
              ),
            ),
            child: const Text(
              'Speichern & Anwenden',
              style: TextStyle(fontWeight: FontWeight.w600),
            ),
          ),
        ],
      ),
    );
  }
}

class _TabButton extends StatelessWidget {
  const _TabButton({
    required this.label,
    required this.active,
    required this.onTap,
  });

  final String label;
  final bool active;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: InkWell(
        onTap: onTap,
        child: Container(
          padding: const EdgeInsets.symmetric(vertical: 11, horizontal: 6),
          decoration: BoxDecoration(
            border: Border(
              bottom: BorderSide(
                color: active ? DocmostTheme.blue : Colors.transparent,
                width: 2,
              ),
            ),
          ),
          child: Text(
            label,
            textAlign: TextAlign.center,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontSize: 12,
              fontWeight: active ? FontWeight.w600 : FontWeight.w500,
              color: active ? Colors.white : DocmostTheme.textMuted,
            ),
          ),
        ),
      ),
    );
  }
}

class _SwitchTile extends StatelessWidget {
  const _SwitchTile({
    required this.title,
    required this.subtitle,
    required this.value,
    required this.onChanged,
  });

  final String title;
  final String subtitle;
  final bool value;
  final ValueChanged<bool> onChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: <Widget>[
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: <Widget>[
                Text(
                  title,
                  style: const TextStyle(
                    fontSize: 13,
                    fontWeight: FontWeight.w600,
                    color: DocmostTheme.textPrimary,
                  ),
                ),
                const SizedBox(height: 2),
                Text(
                  subtitle,
                  style: const TextStyle(
                    fontSize: 11,
                    color: DocmostTheme.textMuted,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(width: 8),
          Switch(
            value: value,
            onChanged: onChanged,
            thumbColor: const WidgetStatePropertyAll<Color>(Colors.white),
            trackColor: WidgetStateProperty.resolveWith<Color>((states) {
              return states.contains(WidgetState.selected)
                  ? DocmostTheme.blue
                  : DocmostTheme.bgHover;
            }),
          ),
        ],
      ),
    );
  }
}
