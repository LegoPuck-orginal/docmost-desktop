import 'package:flutter/material.dart';

import '../theme/docmost_theme.dart';

class AddServerResult {
  const AddServerResult({required this.name, required this.url});

  final String name;
  final String url;
}

class AddServerDialog extends StatefulWidget {
  const AddServerDialog({super.key});

  @override
  State<AddServerDialog> createState() => _AddServerDialogState();
}

class _AddServerDialogState extends State<AddServerDialog> {
  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _urlController = TextEditingController();
  String? _errorText;

  @override
  void dispose() {
    _nameController.dispose();
    _urlController.dispose();
    super.dispose();
  }

  void _submit() {
    final rawUrl = _urlController.text.trim();
    if (rawUrl.isEmpty) {
      setState(() {
        _errorText = 'Bitte eine Server-URL eingeben.';
      });
      return;
    }
    Navigator.of(context).pop(
      AddServerResult(
        name: _nameController.text.trim(),
        url: rawUrl,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      backgroundColor: Colors.transparent,
      insetPadding: const EdgeInsets.symmetric(horizontal: 20, vertical: 24),
      child: Container(
        constraints: const BoxConstraints(maxWidth: 440),
        decoration: BoxDecoration(
          color: DocmostTheme.bgCard,
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: DocmostTheme.border),
          boxShadow: const <BoxShadow>[
            BoxShadow(
              color: Color(0xCC000000),
              blurRadius: 36,
              offset: Offset(0, 16),
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
              padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 14),
              child: Row(
                children: <Widget>[
                  const Expanded(
                    child: Text(
                      'Neuen Server hinzufügen',
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
            const Divider(height: 1, color: DocmostTheme.borderSubtle),
            Padding(
              padding: const EdgeInsets.all(18),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: <Widget>[
                  const Text(
                    'Anzeigename',
                    style: TextStyle(
                      color: DocmostTheme.textLabel,
                      fontSize: 12,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                  const SizedBox(height: 6),
                  TextField(
                    key: const Key('add-server-name-input'),
                    controller: _nameController,
                    style: const TextStyle(
                      color: DocmostTheme.textPrimary,
                      fontSize: 13,
                    ),
                    decoration: _inputDecoration('z. B. Firmen-Wiki oder Home'),
                  ),
                  const SizedBox(height: 14),
                  const Text(
                    'Docmost Server-URL',
                    style: TextStyle(
                      color: DocmostTheme.textLabel,
                      fontSize: 12,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                  const SizedBox(height: 6),
                  TextField(
                    key: const Key('add-server-url-input'),
                    controller: _urlController,
                    keyboardType: TextInputType.url,
                    autocorrect: false,
                    onSubmitted: (_) => _submit(),
                    style: const TextStyle(
                      color: DocmostTheme.textPrimary,
                      fontSize: 13,
                    ),
                    decoration: _inputDecoration('https://doc.meinedomain.de'),
                  ),
                  if (_errorText != null) ...<Widget>[
                    const SizedBox(height: 8),
                    Text(
                      _errorText!,
                      style: const TextStyle(
                        color: DocmostTheme.dangerLight,
                        fontSize: 12,
                      ),
                    ),
                  ],
                ],
              ),
            ),
            Container(
              color: DocmostTheme.bgFooter,
              padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 12),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: <Widget>[
                  TextButton(
                    onPressed: () => Navigator.of(context).pop(),
                    style: TextButton.styleFrom(
                      backgroundColor: DocmostTheme.bgHover,
                      foregroundColor: DocmostTheme.textBody,
                      padding: const EdgeInsets.symmetric(
                        horizontal: 14,
                        vertical: 10,
                      ),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(6),
                        side: const BorderSide(color: DocmostTheme.border),
                      ),
                    ),
                    child: const Text('Abbrechen'),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton(
                    key: const Key('add-server-submit-btn'),
                    onPressed: _submit,
                    style: ElevatedButton.styleFrom(
                      backgroundColor: DocmostTheme.blue,
                      foregroundColor: Colors.white,
                      padding: const EdgeInsets.symmetric(
                        horizontal: 16,
                        vertical: 10,
                      ),
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(6),
                      ),
                    ),
                    child: const Text(
                      'Hinzufügen & Verbinden',
                      style: TextStyle(fontWeight: FontWeight.w600),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  InputDecoration _inputDecoration(String hint) {
    return InputDecoration(
      hintText: hint,
      hintStyle: const TextStyle(color: DocmostTheme.textMuted, fontSize: 13),
      filled: true,
      fillColor: DocmostTheme.bgLowest,
      contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(6),
        borderSide: const BorderSide(color: DocmostTheme.border),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(6),
        borderSide: const BorderSide(color: DocmostTheme.blue),
      ),
    );
  }
}
