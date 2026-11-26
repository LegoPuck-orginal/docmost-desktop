import 'package:docmost_ios/models/app_config.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AppConfig', () {
    test('defaults contain klehausen home server', () {
      final cfg = AppConfig.defaults();
      expect(cfg.servers.length, 1);
      expect(cfg.serverUrl, kDefaultDocmostUrl);
      expect(cfg.activeServer.name, 'Home Server');
      expect(cfg.excalidrawPenMode, isTrue);
      expect(cfg.mediaPlayerEnabled, isTrue);
      expect(cfg.officeViewerEnabled, isTrue);
    });

    test('adding, switching and deleting servers enforces at least one server', () {
      final initial = AppConfig.defaults();
      final added = initial.withAddedServer(
        name: 'Team Wiki',
        rawUrl: 'wiki.example.com/',
      );

      expect(added.servers.length, 2);
      expect(added.activeServer.name, 'Team Wiki');
      expect(added.serverUrl, 'https://wiki.example.com');

      final switched = added.withActiveServer('default');
      expect(switched.serverUrl, kDefaultDocmostUrl);

      final deletedDefault = switched.withDeletedServer('default');
      expect(deletedDefault.servers.length, 1);
      expect(deletedDefault.activeServer.name, 'Team Wiki');

      // Must keep the last remaining server
      final cannotDeleteLast = deletedDefault.withDeletedServer(
        deletedDefault.activeServerId,
      );
      expect(cannotDeleteLast.servers.length, 1);
    });

    test('JSON round-trip preserves all fields', () {
      final cfg = AppConfig.defaults()
          .withAddedServer(name: 'Lab', rawUrl: 'http://192.168.1.50:3000')
          .copyWith(
            uiZoomLevel: 1.25,
            onlyofficeUrl: 'https://office.example.com',
            excalidrawPalmRejection: false,
          );

      final decoded = AppConfig.decodeOrDefault(cfg.encode());
      expect(decoded.servers.length, 2);
      expect(decoded.activeServer.name, 'Lab');
      expect(decoded.serverUrl, 'http://192.168.1.50:3000');
      expect(decoded.uiZoomLevel, 1.25);
      expect(decoded.onlyofficeUrl, 'https://office.example.com');
      expect(decoded.excalidrawPalmRejection, isFalse);
    });
  });
}
