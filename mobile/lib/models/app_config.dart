import 'dart:convert';

const String kDefaultDocmostUrl = 'https://doc.home.klehausen.de';
const String kAppVersion = '0.1.7';

class ServerEntry {
  const ServerEntry({
    required this.id,
    required this.name,
    required this.url,
  });

  final String id;
  final String name;
  final String url;

  factory ServerEntry.fromJson(Map<String, dynamic> json) {
    return ServerEntry(
      id: (json['id'] as String?) ?? 'default',
      name: (json['name'] as String?) ?? 'Docmost Server',
      url: normalizeServerUrl((json['url'] as String?) ?? kDefaultDocmostUrl),
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'name': name,
      'url': url,
    };
  }

  static String normalizeServerUrl(String raw) {
    var clean = raw.trim();
    if (clean.isEmpty) return '';
    if (!clean.startsWith('http://') && !clean.startsWith('https://')) {
      clean = 'https://$clean';
    }
    return clean.replaceAll(RegExp(r'/+$'), '');
  }

  String get displayHost =>
      url.replaceFirst(RegExp(r'^https?://', caseSensitive: false), '');
}

class AppConfig {
  const AppConfig({
    required this.servers,
    required this.activeServerId,
    this.excalidrawPenMode = true,
    this.excalidrawPalmRejection = true,
    this.excalidrawDarkTheme = true,
    this.excalidrawQuickToolbar = true,
    this.uiZoomLevel = 1.0,
    this.onlyofficeUrl = '',
    this.mediaPlayerEnabled = true,
    this.officeViewerEnabled = true,
  });

  final List<ServerEntry> servers;
  final String activeServerId;
  final bool excalidrawPenMode;
  final bool excalidrawPalmRejection;
  final bool excalidrawDarkTheme;
  final bool excalidrawQuickToolbar;
  final double uiZoomLevel;
  final String onlyofficeUrl;
  final bool mediaPlayerEnabled;
  final bool officeViewerEnabled;

  factory AppConfig.defaults() {
    return const AppConfig(
      servers: <ServerEntry>[
        ServerEntry(
          id: 'default',
          name: 'Home Server',
          url: kDefaultDocmostUrl,
        ),
      ],
      activeServerId: 'default',
    );
  }

  ServerEntry get activeServer {
    if (servers.isEmpty) {
      return const ServerEntry(
        id: 'default',
        name: 'Home Server',
        url: kDefaultDocmostUrl,
      );
    }
    for (final entry in servers) {
      if (entry.id == activeServerId) return entry;
    }
    return servers.first;
  }

  String get serverUrl => activeServer.url;

  AppConfig copyWith({
    List<ServerEntry>? servers,
    String? activeServerId,
    bool? excalidrawPenMode,
    bool? excalidrawPalmRejection,
    bool? excalidrawDarkTheme,
    bool? excalidrawQuickToolbar,
    double? uiZoomLevel,
    String? onlyofficeUrl,
    bool? mediaPlayerEnabled,
    bool? officeViewerEnabled,
  }) {
    return AppConfig(
      servers: servers ?? this.servers,
      activeServerId: activeServerId ?? this.activeServerId,
      excalidrawPenMode: excalidrawPenMode ?? this.excalidrawPenMode,
      excalidrawPalmRejection:
          excalidrawPalmRejection ?? this.excalidrawPalmRejection,
      excalidrawDarkTheme: excalidrawDarkTheme ?? this.excalidrawDarkTheme,
      excalidrawQuickToolbar:
          excalidrawQuickToolbar ?? this.excalidrawQuickToolbar,
      uiZoomLevel: uiZoomLevel ?? this.uiZoomLevel,
      onlyofficeUrl: onlyofficeUrl ?? this.onlyofficeUrl,
      mediaPlayerEnabled: mediaPlayerEnabled ?? this.mediaPlayerEnabled,
      officeViewerEnabled: officeViewerEnabled ?? this.officeViewerEnabled,
    );
  }

  AppConfig withActiveServer(String serverId) {
    final exists = servers.any((s) => s.id == serverId);
    if (!exists) return this;
    return copyWith(activeServerId: serverId);
  }

  AppConfig withAddedServer({required String name, required String rawUrl}) {
    final normalized = ServerEntry.normalizeServerUrl(rawUrl);
    if (normalized.isEmpty) return this;
    final id = 'srv_${DateTime.now().microsecondsSinceEpoch}';
    final displayName = name.trim().isEmpty
        ? normalized.replaceFirst(RegExp(r'^https?://'), '')
        : name.trim();
    final updated = List<ServerEntry>.from(servers)
      ..add(ServerEntry(id: id, name: displayName, url: normalized));
    return copyWith(servers: updated, activeServerId: id);
  }

  AppConfig withDeletedServer(String serverId) {
    if (servers.length <= 1) return this;
    final remaining = servers.where((s) => s.id != serverId).toList();
    if (remaining.isEmpty) return this;
    final nextActive = remaining.any((s) => s.id == activeServerId)
        ? activeServerId
        : remaining.first.id;
    return copyWith(servers: remaining, activeServerId: nextActive);
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'servers': servers.map((s) => s.toJson()).toList(),
      'active_server_id': activeServerId,
      'excalidraw_pen_mode': excalidrawPenMode,
      'excalidraw_palm_rejection': excalidrawPalmRejection,
      'excalidraw_dark_theme': excalidrawDarkTheme,
      'excalidraw_quick_toolbar': excalidrawQuickToolbar,
      'ui_zoom_level': uiZoomLevel,
      'onlyoffice_url': onlyofficeUrl,
      'media_player_enabled': mediaPlayerEnabled,
      'office_viewer_enabled': officeViewerEnabled,
    };
  }

  factory AppConfig.fromJson(Map<String, dynamic> json) {
    final rawServers = json['servers'];
    final parsedServers = <ServerEntry>[];
    if (rawServers is List) {
      for (final item in rawServers) {
        if (item is Map<String, dynamic>) {
          parsedServers.add(ServerEntry.fromJson(item));
        } else if (item is Map) {
          parsedServers.add(ServerEntry.fromJson(Map<String, dynamic>.from(item)));
        }
      }
    }
    if (parsedServers.isEmpty) {
      parsedServers.addAll(AppConfig.defaults().servers);
    }

    final rawActive = json['active_server_id'] as String?;
    final activeId = (rawActive != null && parsedServers.any((s) => s.id == rawActive))
        ? rawActive
        : parsedServers.first.id;

    return AppConfig(
      servers: parsedServers,
      activeServerId: activeId,
      excalidrawPenMode: (json['excalidraw_pen_mode'] as bool?) ?? true,
      excalidrawPalmRejection:
          (json['excalidraw_palm_rejection'] as bool?) ?? true,
      excalidrawDarkTheme: (json['excalidraw_dark_theme'] as bool?) ?? true,
      excalidrawQuickToolbar:
          (json['excalidraw_quick_toolbar'] as bool?) ?? true,
      uiZoomLevel: ((json['ui_zoom_level'] as num?) ?? 1.0).toDouble(),
      onlyofficeUrl: (json['onlyoffice_url'] as String?) ?? '',
      mediaPlayerEnabled: (json['media_player_enabled'] as bool?) ?? true,
      officeViewerEnabled: (json['office_viewer_enabled'] as bool?) ?? true,
    );
  }

  String encode() => jsonEncode(toJson());

  static AppConfig decodeOrDefault(String? raw) {
    if (raw == null || raw.trim().isEmpty) return AppConfig.defaults();
    try {
      final decoded = jsonDecode(raw);
      if (decoded is Map<String, dynamic>) {
        return AppConfig.fromJson(decoded);
      }
      if (decoded is Map) {
        return AppConfig.fromJson(Map<String, dynamic>.from(decoded));
      }
    } catch (_) {
      // Fallback safely to default config on corrupted local storage
    }
    return AppConfig.defaults();
  }
}
