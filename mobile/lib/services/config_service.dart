import 'package:shared_preferences/shared_preferences.dart';

import '../models/app_config.dart';

/// Persists multi-server settings and viewer preferences locally on iOS.
class ConfigService {
  static const String _storageKey = 'docmost_app_config_v1';

  Future<AppConfig> load() async {
    final prefs = await SharedPreferences.getInstance();
    return AppConfig.decodeOrDefault(prefs.getString(_storageKey));
  }

  Future<void> save(AppConfig config) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_storageKey, config.encode());
  }
}
