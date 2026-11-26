enum AttachmentKind {
  mediaVideo,
  mediaAudio,
  officePdf,
  officeDocument,
  other,
}

class AttachmentInfo {
  const AttachmentInfo({
    required this.kind,
    required this.url,
    required this.displayName,
    required this.extensionBadge,
  });

  final AttachmentKind kind;
  final String url;
  final String displayName;
  final String extensionBadge;

  bool get isMedia =>
      kind == AttachmentKind.mediaVideo || kind == AttachmentKind.mediaAudio;

  bool get isOffice =>
      kind == AttachmentKind.officePdf || kind == AttachmentKind.officeDocument;
}

class AttachmentClassifier {
  AttachmentClassifier._();

  static const Set<String> videoExtensions = <String>{
    '.mp4',
    '.webm',
    '.mov',
    '.mkv',
    '.m4v',
  };

  static const Set<String> audioExtensions = <String>{
    '.mp3',
    '.wav',
    '.ogg',
    '.m4a',
    '.aac',
    '.flac',
  };

  static const Set<String> officeExtensions = <String>{
    '.docx',
    '.doc',
    '.pptx',
    '.ppt',
    '.xlsx',
    '.xls',
    '.odt',
    '.ods',
    '.odp',
    '.pdf',
  };

  static AttachmentInfo classify({
    required String url,
    String? title,
    String? downloadName,
  }) {
    final cleanUrl = url.split('?').first.split('#').first.toLowerCase();
    final titleLower = (title ?? '').trim().toLowerCase();
    final downloadLower = (downloadName ?? '').trim().toLowerCase();

    final matchedExt = _findExtension(<String>[cleanUrl, downloadLower, titleLower]);
    final fallbackFileName = _extractFileName(url);
    final rawName = (downloadName != null && downloadName.trim().isNotEmpty)
        ? downloadName.trim()
        : (title != null && title.trim().isNotEmpty)
            ? title.trim()
            : fallbackFileName;

    final badge = matchedExt.isNotEmpty
        ? matchedExt.substring(1).toUpperCase()
        : 'FILE';

    AttachmentKind kind = AttachmentKind.other;
    if (videoExtensions.contains(matchedExt)) {
      kind = AttachmentKind.mediaVideo;
    } else if (audioExtensions.contains(matchedExt)) {
      kind = AttachmentKind.mediaAudio;
    } else if (matchedExt == '.pdf') {
      kind = AttachmentKind.officePdf;
    } else if (officeExtensions.contains(matchedExt)) {
      kind = AttachmentKind.officeDocument;
    }

    return AttachmentInfo(
      kind: kind,
      url: url,
      displayName: rawName.isEmpty ? 'Datei' : rawName,
      extensionBadge: badge,
    );
  }

  static String buildOnlyOfficeViewerUrl({
    required String onlyofficeServerUrl,
    required String fileUrl,
  }) {
    var base = onlyofficeServerUrl.trim();
    if (base.isEmpty) return '';
    if (!base.endsWith('/')) {
      base = '$base/';
    }
    return '${base}web-apps/apps/documenteditor/main/index.html?mode=view&url=${Uri.encodeComponent(fileUrl)}';
  }

  static String iconForExtension(String extLowerWithoutDot) {
    switch (extLowerWithoutDot.toLowerCase()) {
      case 'docx':
      case 'doc':
      case 'odt':
        return '📝';
      case 'pptx':
      case 'ppt':
      case 'odp':
        return '📊';
      case 'xlsx':
      case 'xls':
      case 'ods':
      case 'csv':
        return '📈';
      case 'pdf':
        return '📄';
      default:
        return '📁';
    }
  }

  static String _findExtension(List<String> candidates) {
    final allExts = <String>[
      ...videoExtensions,
      ...audioExtensions,
      ...officeExtensions,
    ];
    for (final candidate in candidates) {
      for (final ext in allExts) {
        if (candidate.endsWith(ext)) return ext;
      }
    }
    return '';
  }

  static String _extractFileName(String url) {
    final withoutQuery = url.split('?').first.split('#').first;
    final parts = withoutQuery.split('/');
    if (parts.isEmpty) return '';
    try {
      return Uri.decodeComponent(parts.last);
    } catch (_) {
      return parts.last;
    }
  }
}
