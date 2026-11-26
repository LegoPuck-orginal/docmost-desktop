import 'package:docmost_ios/models/app_config.dart';
import 'package:docmost_ios/utils/attachment_classifier.dart';
import 'package:docmost_ios/webview/docmost_injection_script.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('AttachmentClassifier', () {
    test('detects video, audio, pdf and office attachments accurately', () {
      final mp4 = AttachmentClassifier.classify(
        url: 'https://doc.home.klehausen.de/api/files/clip.mp4?token=123',
      );
      expect(mp4.kind, AttachmentKind.mediaVideo);
      expect(mp4.isMedia, isTrue);
      expect(mp4.extensionBadge, 'MP4');

      final mp3 = AttachmentClassifier.classify(
        url: 'https://doc.home.klehausen.de/api/files/123',
        title: 'meeting-note.m4a',
      );
      expect(mp3.kind, AttachmentKind.mediaAudio);
      expect(mp3.isMedia, isTrue);
      expect(mp3.extensionBadge, 'M4A');

      final pdf = AttachmentClassifier.classify(
        url: 'https://doc.home.klehausen.de/files/manual.pdf',
      );
      expect(pdf.kind, AttachmentKind.officePdf);
      expect(pdf.isOffice, isTrue);
      expect(pdf.extensionBadge, 'PDF');

      final docx = AttachmentClassifier.classify(
        url: 'https://doc.home.klehausen.de/files/spec.docx',
      );
      expect(docx.kind, AttachmentKind.officeDocument);
      expect(docx.isOffice, isTrue);
      expect(docx.extensionBadge, 'DOCX');
    });

    test('builds valid OnlyOffice viewer URL', () {
      final url = AttachmentClassifier.buildOnlyOfficeViewerUrl(
        onlyofficeServerUrl: 'https://onlyoffice.example.com',
        fileUrl: 'https://doc.home.klehausen.de/files/test.pptx',
      );
      expect(
        url,
        startsWith(
          'https://onlyoffice.example.com/web-apps/apps/documenteditor/main/index.html?mode=view&url=',
        ),
      );
      expect(url, contains('test.pptx'));
    });

    test('builds injection script with Excalidraw and DocmostBridge hooks', () {
      final script = DocmostInjectionScript.build(AppConfig.defaults());
      expect(script, contains('__docmostMobileInitialized'));
      expect(script, contains('DocmostBridge'));
      expect(script, contains('docmost-pen-toolbar'));
      expect(script, contains('__docmostSyncConfig'));
    });
  });
}
