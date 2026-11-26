import 'package:flutter/material.dart';

/// Docmost Mantine Dark color palette and shared ThemeData.
class DocmostTheme {
  DocmostTheme._();

  static const Color bgLowest = Color(0xFF141414);
  static const Color bgFooter = Color(0xFF1A1A1A);
  static const Color bgCard = Color(0xFF1F1F1F);
  static const Color bgElevated = Color(0xFF242424);
  static const Color bgHover = Color(0xFF2E2E2E);

  static const Color border = Color(0xFF383838);
  static const Color borderSubtle = Color(0xFF2E2E2E);
  static const Color borderHover = Color(0xFF4A4A4A);

  static const Color textPrimary = Color(0xFFF1F5F9);
  static const Color textBody = Color(0xFFC9C9C9);
  static const Color textLabel = Color(0xFFB8B8B8);
  static const Color textMuted = Color(0xFF828282);

  static const Color blue = Color(0xFF228BE6);
  static const Color blueHover = Color(0xFF1C7ED6);
  static const Color blueLight = Color(0xFF339AF0);
  static const Color blueBadge = Color(0xFF74C0FC);
  static const Color blueTint = Color(0x26228BE6);

  static const Color teal = Color(0xFF12B886);
  static const Color tealTint = Color(0x1F12B886);

  static const Color danger = Color(0xFFEF4444);
  static const Color dangerLight = Color(0xFFFF6B6B);
  static const Color dangerTint = Color(0x33EF4444);

  static ThemeData get darkTheme {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.dark,
      scaffoldBackgroundColor: bgLowest,
      canvasColor: bgCard,
      colorScheme: const ColorScheme.dark(
        primary: blue,
        secondary: blueLight,
        surface: bgCard,
        error: danger,
        onPrimary: Colors.white,
        onSecondary: Colors.white,
        onSurface: textBody,
      ),
      dividerColor: borderSubtle,
      fontFamily: '.SF Pro Text',
      textTheme: const TextTheme(
        bodyMedium: TextStyle(color: textBody, fontSize: 13),
        bodySmall: TextStyle(color: textMuted, fontSize: 11),
        titleMedium: TextStyle(
          color: textPrimary,
          fontSize: 15,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }
}
