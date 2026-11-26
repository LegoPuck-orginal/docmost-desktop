import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';

import '../theme/docmost_theme.dart';
import '../utils/attachment_classifier.dart';

/// Mantine Dark modal document viewer using iOS WKWebView native QuickLook/PDFKit or OnlyOffice.
class OfficeViewerModal extends StatefulWidget {
  const OfficeViewerModal({
    super.key,
    required this.attachment,
    required this.onlyofficeUrl,
    required this.onOpenExternal,
  });

  final AttachmentInfo attachment;
  final String onlyofficeUrl;
  final ValueChanged<String> onOpenExternal;

  @override
  State<OfficeViewerModal> createState() => _OfficeViewerModalState();
}

class _OfficeViewerModalState extends State<OfficeViewerModal> {
  late final WebViewController _controller;
  late bool _useOnlyOffice;
  bool _isLoading = true;

  bool get _hasOnlyOffice =>
      widget.onlyofficeUrl.trim().isNotEmpty &&
      widget.attachment.kind != AttachmentKind.officePdf;

  @override
  void initState() {
    super.initState();
    _useOnlyOffice = _hasOnlyOffice;

    _controller = WebViewController()
      ..setJavaScriptMode(JavaScriptMode.unrestricted)
      ..setBackgroundColor(DocmostTheme.bgLowest)
      ..setNavigationDelegate(
        NavigationDelegate(
          onPageStarted: (_) {
            if (mounted) setState(() => _isLoading = true);
          },
          onPageFinished: (_) {
            if (mounted) setState(() => _isLoading = false);
          },
        ),
      );
    _loadCurrentMode();
  }

  void _loadCurrentMode() {
    final targetUrl = _useOnlyOffice
        ? AttachmentClassifier.buildOnlyOfficeViewerUrl(
            onlyofficeServerUrl: widget.onlyofficeUrl,
            fileUrl: widget.attachment.url,
          )
        : widget.attachment.url;

    final uri = Uri.tryParse(targetUrl);
    if (uri != null) {
      _controller.loadRequest(uri);
    }
  }

  void _toggleMode() {
    if (!_hasOnlyOffice) return;
    setState(() {
      _useOnlyOffice = !_useOnlyOffice;
      _isLoading = true;
    });
    _loadCurrentMode();
  }

  @override
  Widget build(BuildContext context) {
    final extIcon =
        AttachmentClassifier.iconForExtension(widget.attachment.extensionBadge);
    return Dialog(
      backgroundColor: Colors.transparent,
      insetPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 18),
      child: Container(
        width: double.infinity,
        height: MediaQuery.sizeOf(context).height * 0.86,
        constraints: const BoxConstraints(maxWidth: 960),
        decoration: BoxDecoration(
          color: DocmostTheme.bgCard,
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: DocmostTheme.border),
          boxShadow: const <BoxShadow>[
            BoxShadow(
              color: Color(0xCC000000),
              blurRadius: 42,
              offset: Offset(0, 18),
            ),
          ],
        ),
        clipBehavior: Clip.antiAlias,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              decoration: const BoxDecoration(
                color: DocmostTheme.bgElevated,
                border: Border(bottom: BorderSide(color: DocmostTheme.borderSubtle)),
              ),
              child: Row(
                children: <Widget>[
                  Text(extIcon, style: const TextStyle(fontSize: 16)),
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
                    padding:
                        const EdgeInsets.symmetric(horizontal: 7, vertical: 2),
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
                      child: Icon(
                        Icons.close,
                        size: 18,
                        color: DocmostTheme.textMuted,
                      ),
                    ),
                  ),
                ],
              ),
            ),
            if (_isLoading)
              const LinearProgressIndicator(
                minHeight: 2,
                color: DocmostTheme.blue,
                backgroundColor: DocmostTheme.bgLowest,
              ),
            Expanded(
              child: WebViewWidget(controller: _controller),
            ),
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              decoration: const BoxDecoration(
                color: DocmostTheme.bgFooter,
                border: Border(top: BorderSide(color: DocmostTheme.borderSubtle)),
              ),
              child: Wrap(
                alignment: WrapAlignment.end,
                spacing: 8,
                runSpacing: 8,
                children: <Widget>[
                  if (_hasOnlyOffice)
                    TextButton(
                      onPressed: _toggleMode,
                      style: TextButton.styleFrom(
                        backgroundColor: DocmostTheme.bgHover,
                        foregroundColor: DocmostTheme.textBody,
                      ),
                      child: Text(
                        _useOnlyOffice
                            ? 'iOS Vorschau nutzen'
                            : 'Mit OnlyOffice öffnen',
                      ),
                    ),
                  TextButton(
                    onPressed: () => widget.onOpenExternal(widget.attachment.url),
                    style: TextButton.styleFrom(
                      backgroundColor: DocmostTheme.bgHover,
                      foregroundColor: DocmostTheme.textBody,
                    ),
                    child: const Text('In Safari / Dateien öffnen ↗'),
                  ),
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
