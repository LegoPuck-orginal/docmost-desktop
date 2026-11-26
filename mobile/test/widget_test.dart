import 'package:docmost_ios/models/app_config.dart';
import 'package:docmost_ios/theme/docmost_theme.dart';
import 'package:docmost_ios/widgets/add_server_dialog.dart';
import 'package:docmost_ios/widgets/app_menu_dropdown.dart';
import 'package:docmost_ios/widgets/settings_modal.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('AppMenuDropdownOverlay renders servers and actions', (tester) async {
    var selectedId = '';
    var openedTab = -1;

    await tester.pumpWidget(
      MaterialApp(
        theme: DocmostTheme.darkTheme,
        home: Scaffold(
          body: AppMenuDropdownOverlay(
            config: AppConfig.defaults(),
            onClose: () {},
            onGoBack: () {},
            onGoForward: () {},
            onReload: () {},
            onGoHome: () {},
            onSelectServer: (id) => selectedId = id,
            onDeleteServer: (_) {},
            onAddServer: () {},
            onOpenSettingsTab: (tab) => openedTab = tab,
            onOpenInSafari: () {},
          ),
        ),
      ),
    );

    expect(find.text('Home Server'), findsOneWidget);
    expect(find.text('Stift & Excalidraw'), findsOneWidget);
    expect(find.text('Office & Medien-Viewer'), findsOneWidget);

    await tester.tap(find.text('Home Server'));
    expect(selectedId, 'default');

    await tester.tap(find.text('Stift & Excalidraw'));
    expect(openedTab, 1);
  });

  testWidgets('SettingsModal switches tabs and saves changes', (tester) async {
    SettingsModalResult? savedResult;

    await tester.pumpWidget(
      MaterialApp(
        theme: DocmostTheme.darkTheme,
        home: Scaffold(
          body: Builder(
            builder: (context) => ElevatedButton(
              onPressed: () async {
                savedResult = await showDialog<SettingsModalResult>(
                  context: context,
                  builder: (_) => SettingsModal(
                    initialConfig: AppConfig.defaults(),
                    onAddServerRequest: () async => null,
                  ),
                );
              },
              child: const Text('Open'),
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.text('Open'));
    await tester.pumpAndSettle();

    expect(find.text('Docmost iOS Einstellungen'), findsOneWidget);
    await tester.tap(find.text('🖊️ Apple Pencil'));
    await tester.pumpAndSettle();
    expect(find.text('Stift-Modus (Apple Pencil Only)'), findsOneWidget);

    await tester.tap(find.byKey(const Key('settings-save-btn')));
    await tester.pumpAndSettle();
    expect(savedResult, isNotNull);
    expect(savedResult!.config.serverUrl, kDefaultDocmostUrl);
  });

  testWidgets('AddServerDialog validates and returns entered server', (tester) async {
    AddServerResult? result;

    await tester.pumpWidget(
      MaterialApp(
        theme: DocmostTheme.darkTheme,
        home: Scaffold(
          body: Builder(
            builder: (context) => ElevatedButton(
              onPressed: () async {
                result = await showDialog<AddServerResult>(
                  context: context,
                  builder: (_) => const AddServerDialog(),
                );
              },
              child: const Text('Add'),
            ),
          ),
        ),
      ),
    );

    await tester.tap(find.text('Add'));
    await tester.pumpAndSettle();

    await tester.enterText(
      find.byKey(const Key('add-server-name-input')),
      'Office',
    );
    await tester.enterText(
      find.byKey(const Key('add-server-url-input')),
      'https://notes.example.org',
    );
    await tester.tap(find.byKey(const Key('add-server-submit-btn')));
    await tester.pumpAndSettle();

    expect(result, isNotNull);
    expect(result!.name, 'Office');
    expect(result!.url, 'https://notes.example.org');
  });
}
