import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_wkwebview/webview_flutter_wkwebview.dart';

import '../models/app_config.dart';
import '../services/config_service.dart';
import '../theme/docmost_theme.dart';
import '../utils/attachment_classifier.dart';
import '../webview/docmost_injection_script.dart';
import '../widgets/add_server_dialog.dart';
import '../widgets/app_menu_dropdown.dart';
import '../widgets/media_player_modal.dart';
import '../widgets/office_viewer_modal.dart';
import '../widgets/settings_modal.dart';

class DocmostHomeScreen extends StatefulWidget {
  const DocmostHomeScreen({super.key, ConfigService? configService})
      : _configService = configService;

  final ConfigService? _configService;

  @override
  State<DocmostHomeScreen> createState() => _DocmostHomeScreenState();
}

class _DocmostHomeScreenState extends State<DocmostHomeScreen> {
  late final ConfigService _configService;
  WebViewController? _webController;
  AppConfig _config = AppConfig.defaults();

  bool _isBootstrapping = true;
  bool _isMenuOpen = false;
  int _loadingProgress = 0;
  String? _connectionError;
  final TextEditingController _offlineUrlController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _configService = widget._configService ?? ConfigService();
    _initApp();
  }

  @override
  void dispose() {
    _offlineUrlController.dispose();
    super.dispose();
  }

  Future<void> _initApp() async {
    final loaded = await _configService.load();
    if (!mounted) return;
    _config = loaded;
    _offlineUrlController.text = loaded.serverUrl;

    final controller = _createWebController(loaded);
    setState(() {
      _webController = controller;
      _isBootstrapping = false;
    });

    final uri = Uri.tryParse(loaded.serverUrl);
    if (uri != null && loaded.serverUrl.isNotEmpty) {
      await controller.loadRequest(uri);
    }
  }

  WebViewController _createWebController(AppConfig cfg) {
    late final PlatformWebViewControllerCreationParams params;
    if (WebViewPlatform.instance is WebKitWebViewPlatform) {
      params = WebKitWebViewControllerCreationParams(
        allowsInlineMediaPlayback: true,
        mediaTypesRequiringUserAction: const <PlaybackMediaTypes>{},
      );
    } else {
      params = const PlatformWebViewControllerCreationParams();
    }

    final controller = WebViewController.fromPlatformCreationParams(params)
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setBackgroundColor(DocmostTheme.bgLowest)
      ..addJavaScriptChannel(
        'DocmostBridge',
        onMessageReceived: (JavaScriptMessage msg) {
          _handleBridgeMessage(msg.message);
        },
      )
      ..setNavigationDelegate(
        NavigationDelegate(
          onProgress: (int progress) {
            if (!mounted) return;
            setState(() => _loadingProgress = progress);
          },
          onPageStarted: (_) {
            if (!mounted) return;
            setState(() => _connectionError = null);
          },
          onPageFinished: (_) async {
            await _injectDesktopEnhancements();
          },
          onWebResourceError: (WebResourceError error) {
            if (error.isForMainFrame == true && mounted) {
              setState(() {
                _connectionError = error.description.isNotEmpty
                    ? error.description
                    : 'Server konnte nicht geladen werden.';
              });
            }
          },
        ),
      );
    return controller;
  }

  Future<void> _injectDesktopEnhancements() async {
    final controller = _webController;
    if (controller == null) return;
    try {
      await controller.runJavaScript(DocmostInjectionScript.build(_config));
    } catch (_) {
      // Page might still be transitioning
    }
  }

  Future<void> _syncConfigToWebView() async {
    final controller = _webController;
    if (controller == null) return;
    try {
      await controller.runJavaScript(
        DocmostInjectionScript.buildSyncScript(_config),
      );
    } catch (_) {}
  }

  void _handleBridgeMessage(String raw) {
    try {
      final decoded = jsonDecode(raw);
      if (decoded is! Map<String, dynamic>) return;
      if (decoded['action'] != 'open_attachment') return;

      final url = (decoded['url'] as String?) ?? '';
      if (url.isEmpty) return;

      final info = AttachmentClassifier.classify(
        url: url,
        title: decoded['title'] as String?,
        downloadName: decoded['download'] as String?,
      );

      if (info.isMedia && _config.mediaPlayerEnabled) {
        _openMediaModal(info);
      } else if (info.isOffice && _config.officeViewerEnabled) {
        _openOfficeModal(info);
      } else {
        _launchExternal(url);
      }
    } catch (_) {}
  }

  Future<void> _openMediaModal(AttachmentInfo info) async {
    if (!mounted) return;
    await showDialog<void>(
      context: context,
      builder: (_) => MediaPlayerModal(
        attachment: info,
        serverBaseUrl: _config.serverUrl,
        onOpenExternal: _launchExternal,
      ),
    );
  }

  Future<void> _openOfficeModal(AttachmentInfo info) async {
    if (!mounted) return;
    await showDialog<void>(
      context: context,
      builder: (_) => OfficeViewerModal(
        attachment: info,
        onlyofficeUrl: _config.onlyofficeUrl,
        onOpenExternal: _launchExternal,
      ),
    );
  }

  Future<void> _launchExternal(String url) async {
    final uri = Uri.tryParse(url);
    if (uri == null) return;
    await launchUrl(uri, mode: LaunchMode.externalApplication);
  }

  Future<void> _loadUrl(String url) async {
    final uri = Uri.tryParse(url);
    final controller = _webController;
    if (uri == null || controller == null) return;
    setState(() {
      _connectionError = null;
      _offlineUrlController.text = url;
    });
    await controller.loadRequest(uri);
  }

  Future<void> _selectServer(String serverId) async {
    final next = _config.withActiveServer(serverId);
    await _configService.save(next);
    if (!mounted) return;
    setState(() => _config = next);
    await _loadUrl(next.serverUrl);
  }

  Future<void> _deleteServer(String serverId) async {
    final prevUrl = _config.serverUrl;
    final next = _config.withDeletedServer(serverId);
    await _configService.save(next);
    if (!mounted) return;
    setState(() => _config = next);
    if (next.serverUrl != prevUrl) {
      await _loadUrl(next.serverUrl);
    }
  }

  Future<AddServerModalData?> _promptAddServer() async {
    final res = await showDialog<AddServerResult>(
      context: context,
      builder: (_) => const AddServerDialog(),
    );
    if (res == null) return null;
    return AddServerModalData(name: res.name, url: res.url);
  }

  Future<void> _addServerFromMenu() async {
    final res = await showDialog<AddServerResult>(
      context: context,
      builder: (_) => const AddServerDialog(),
    );
    if (res == null || !mounted) return;
    final next = _config.withAddedServer(name: res.name, rawUrl: res.url);
    await _configService.save(next);
    if (!mounted) return;
    setState(() => _config = next);
    await _loadUrl(next.serverUrl);
  }

  Future<void> _openSettings(int initialTabIndex) async {
    final res = await showDialog<SettingsModalResult>(
      context: context,
      builder: (_) => SettingsModal(
        initialConfig: _config,
        initialTabIndex: initialTabIndex,
        onAddServerRequest: _promptAddServer,
      ),
    );
    if (res == null || !mounted) return;
    await _configService.save(res.config);
    if (!mounted) return;
    setState(() => _config = res.config);
    if (res.serverChanged) {
      await _loadUrl(res.config.serverUrl);
    } else {
      await _syncConfigToWebView();
    }
  }

  Future<void> _goBack() async {
    final controller = _webController;
    if (controller != null && await controller.canGoBack()) {
      await controller.goBack();
    }
  }

  Future<void> _goForward() async {
    final controller = _webController;
    if (controller != null && await controller.canGoForward()) {
      await controller.goForward();
    }
  }

  Future<void> _reload() async {
    setState(() => _connectionError = null);
    await _webController?.reload();
  }

  Future<void> _openCurrentInSafari() async {
    final current = await _webController?.currentUrl();
    final target = (current != null && current.isNotEmpty)
        ? current
        : _config.serverUrl;
    await _launchExternal(target);
  }

  Future<void> _reconnectFromOffline() async {
    final normalized = ServerEntry.normalizeServerUrl(_offlineUrlController.text);
    if (normalized.isEmpty) return;

    final updatedServers = _config.servers.map((s) {
      if (s.id == _config.activeServerId) {
        return ServerEntry(id: s.id, name: s.name, url: normalized);
      }
      return s;
    }).toList();
    final next = _config.copyWith(servers: updatedServers);
    await _configService.save(next);
    if (!mounted) return;
    setState(() {
      _config = next;
      _connectionError = null;
    });
    await _loadUrl(normalized);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: DocmostTheme.bgLowest,
      body: SafeArea(
        child: Stack(
          children: <Widget>[
            if (_isBootstrapping)
              const Center(
                child: CircularProgressIndicator(color: DocmostTheme.blue),
              )
            else if (_connectionError != null || _config.serverUrl.isEmpty)
              _buildOfflineCard()
            else if (_webController != null)
              WebViewWidget(controller: _webController!),
            if (_loadingProgress > 0 && _loadingProgress < 100)
              const Positioned(
                top: 0,
                left: 0,
                right: 0,
                child: LinearProgressIndicator(
                  minHeight: 2,
                  color: DocmostTheme.blue,
                  backgroundColor: Colors.transparent,
                ),
              ),
            if (_isMenuOpen)
              AppMenuDropdownOverlay(
                config: _config,
                onClose: () => setState(() => _isMenuOpen = false),
                onGoBack: _goBack,
                onGoForward: _goForward,
                onReload: _reload,
                onGoHome: () => _loadUrl(_config.serverUrl),
                onSelectServer: _selectServer,
                onDeleteServer: _deleteServer,
                onAddServer: _addServerFromMenu,
                onOpenSettingsTab: _openSettings,
                onOpenInSafari: _openCurrentInSafari,
              ),
            Positioned(
              top: 8,
              right: 12,
              child: AppMenuTriggerButton(
                isOpen: _isMenuOpen,
                activeServerName: _config.activeServer.name,
                onTap: () => setState(() => _isMenuOpen = !_isMenuOpen),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildOfflineCard() {
    return Center(
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(20),
        child: Container(
          constraints: const BoxConstraints(maxWidth: 440),
          padding: const EdgeInsets.all(28),
          decoration: BoxDecoration(
            color: DocmostTheme.bgCard,
            borderRadius: BorderRadius.circular(10),
            border: Border.all(color: DocmostTheme.border),
            boxShadow: const <BoxShadow>[
              BoxShadow(
                color: Color(0xBF000000),
                blurRadius: 36,
                offset: Offset(0, 16),
              ),
            ],
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  gradient: const LinearGradient(
                    colors: <Color>[DocmostTheme.blue, Color(0xFF7950F2)],
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                  ),
                  borderRadius: BorderRadius.circular(12),
                ),
                alignment: Alignment.center,
                child: const Text(
                  'D',
                  style: TextStyle(
                    fontSize: 28,
                    fontWeight: FontWeight.w800,
                    color: Colors.white,
                  ),
                ),
              ),
              const SizedBox(height: 16),
              const Text(
                'Docmost iOS',
                style: TextStyle(
                  fontSize: 21,
                  fontWeight: FontWeight.w700,
                  color: DocmostTheme.textPrimary,
                ),
              ),
              const SizedBox(height: 4),
              const Text(
                'Clean Dark Edition • Flutter & WKWebView',
                style: TextStyle(fontSize: 12, color: DocmostTheme.textMuted),
              ),
              if (_connectionError != null) ...<Widget>[
                const SizedBox(height: 18),
                Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(12),
                  decoration: BoxDecoration(
                    color: DocmostTheme.dangerTint,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: DocmostTheme.danger),
                  ),
                  child: Text(
                    'Verbindungsfehler: $_connectionError',
                    style: const TextStyle(
                      fontSize: 12,
                      color: Color(0xFFFCA5A5),
                    ),
                  ),
                ),
              ],
              const SizedBox(height: 18),
              const Align(
                alignment: Alignment.centerLeft,
                child: Text(
                  'Docmost Instanz URL',
                  style: TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.w500,
                    color: DocmostTheme.textLabel,
                  ),
                ),
              ),
              const SizedBox(height: 6),
              TextField(
                controller: _offlineUrlController,
                keyboardType: TextInputType.url,
                autocorrect: false,
                onSubmitted: (_) => _reconnectFromOffline(),
                style: const TextStyle(
                  color: DocmostTheme.textPrimary,
                  fontSize: 13,
                ),
                decoration: InputDecoration(
                  hintText: kDefaultDocmostUrl,
                  hintStyle: const TextStyle(color: DocmostTheme.textMuted),
                  filled: true,
                  fillColor: DocmostTheme.bgLowest,
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 11,
                  ),
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
              const SizedBox(height: 16),
              SizedBox(
                width: double.infinity,
                child: ElevatedButton(
                  onPressed: _reconnectFromOffline,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: DocmostTheme.blue,
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(vertical: 12),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(6),
                    ),
                  ),
                  child: const Text(
                    'Verbinden & Öffnen',
                    style: TextStyle(fontSize: 14, fontWeight: FontWeight.w600),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
