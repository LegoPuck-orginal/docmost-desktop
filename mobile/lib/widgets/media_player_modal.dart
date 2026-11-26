import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_wkwebview/webview_flutter_wkwebview.dart';

import '../theme/docmost_theme.dart';
import '../utils/attachment_classifier.dart';

/// Mantine Dark modal media player using cookie-sharing iOS WKWebView inline playback.
class MediaPlayerModal extends StatefulWidget {
  const MediaPlayerModal({
    super.key,
    required this.attachment,
    required this.serverBaseUrl,
    required this.onOpenExternal,
  });

  final AttachmentInfo attachment;
  final String serverBaseUrl;
  final ValueChanged<String> onOpenExternal;

  @override
  State<MediaPlayerModal> createState() => _MediaPlayerModalState();
}

class _MediaPlayerModalState extends State<MediaPlayerModal> {
  late final WebViewController _controller;

  @override
  void initState() {
    super.initState();
    late final PlatformWebViewControllerCreationParams params;
    if (WebViewPlatform.instance is WebKitWebViewPlatform) {
      params = WebKitWebViewControllerCreationParams(
        allowsInlineMediaPlayback: true,
        mediaTypesRequiringUserAction: const <PlaybackMediaTypes>{},
      );
    } else {
      params = const PlatformWebViewControllerCreationParams();
    }

    _controller = WebViewController.fromPlatformCreationParams(params)
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setBackgroundColor(DocmostTheme.bgLowest)
      ..loadHtmlString(
        _buildPlayerHtml(),
        baseUrl: widget.serverBaseUrl.isEmpty ? null : widget.serverBaseUrl,
      );
  }

  String _buildPlayerHtml() {
    const htmlEscape = HtmlEscape();
    final safeUrl = htmlEscape.convert(widget.attachment.url);
    final safeName = htmlEscape.convert(widget.attachment.displayName);
    final isAudio = widget.attachment.kind == AttachmentKind.mediaAudio;

    final playerElement = isAudio
        ? '''
<div style="display:flex;flex-direction:column;align-items:center;gap:18px;width:100%;padding:24px 16px;box-sizing:border-box;">
  <div style="width:72px;height:72px;border-radius:50%;background:linear-gradient(135deg,#228be6,#339af0);display:flex;align-items:center;justify-content:center;font-size:32px;box-shadow:0 0 24px rgba(34,139,230,0.45);">🎵</div>
  <div style="color:#f1f5f9;font-weight:600;font-size:14px;text-align:center;word-break:break-word;">$safeName</div>
  <audio controls autoplay style="width:92%;max-width:480px;" src="$safeUrl"></audio>
</div>
'''
        : '''
<video controls playsinline autoplay style="max-width:100%;max-height:100%;border-radius:6px;background:#000;" src="$safeUrl"></video>
''';

    return '''
<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0">
<style>
  html, body {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    background: #141414;
    color: #c9c9c9;
    font-family: -apple-system, BlinkMacSystemFont, sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
</head>
<body>$playerElement</body>
</html>
''';
  }

  @override
  Widget build(BuildContext context) {
    final isAudio = widget.attachment.kind == AttachmentKind.mediaAudio;
    return Dialog(
      backgroundColor: Colors.transparent,
      insetPadding: const EdgeInsets.symmetric(horizontal: 14, vertical: 24),
      child: Container(
        constraints: BoxConstraints(
          maxWidth: 760,
          maxHeight: MediaQuery.sizeOf(context).height * 0.8,
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
            Container(
              color: DocmostTheme.bgElevated,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              child: Row(
                children: <Widget>[
                  Text(isAudio ? '🎵' : '🎬', style: const TextStyle(fontSize: 16)),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      widget.attachment.displayName,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        color: DocmostTheme.textPrimary,
                        fontSize: 14,
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 2),
                    decoration: BoxDecoration(
                      color: DocmostTheme.bgHover,
                      borderRadius: BorderRadius.circular(4),
                      border: Border.all(color: DocmostTheme.border),
                    ),
                    child: Text(
                      widget.attachment.extensionBadge,
                      style: const TextStyle(
                        color: DocmostTheme.blueBadge,
                        fontSize: 11,
                        fontWeight: FontWeight.w700,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  InkWell(
                    onTap: () => Navigator.of(context).pop(),
                    child: const Padding(
                      padding: EdgeInsets.all(4),
                      child: Icon(Icons.close, size: 18, color: DocmostTheme.textMuted),
                    ),
                  ),
                ],
              ),
            ),
            const Divider(height: 1, color: DocmostTheme.borderSubtle),
            SizedBox(
              height: isAudio ? 240 : 320,
              child: WebViewWidget(controller: _controller),
            ),
            Container(
              color: DocmostTheme.bgFooter,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: <Widget>[
                  TextButton(
                    onPressed: () => widget.onOpenExternal(widget.attachment.url),
                    style: TextButton.styleFrom(
                      backgroundColor: DocmostTheme.bgHover,
                      foregroundColor: DocmostTheme.textBody,
                    ),
                    child: const Text('In Safari öffnen ↗'),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton(
                    onPressed: () => Navigator.of(context).pop(),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: DocmostTheme.blue,
                      foregroundColor: Colors.white,
                    ),
                    child: const Text('Schließen'),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
