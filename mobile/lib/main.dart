import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'screens/docmost_home_screen.dart';
import 'theme/docmost_theme.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  SystemChrome.setSystemUIOverlayStyle(
    const SystemUiOverlayStyle(
      statusBarColor: DocmostTheme.bgLowest,
      statusBarBrightness: Brightness.dark,
      statusBarIconBrightness: Brightness.light,
      systemNavigationBarColor: DocmostTheme.bgLowest,
      systemNavigationBarIconBrightness: Brightness.light,
    ),
  );
  runApp(const DocmostIosApp());
}

class DocmostIosApp extends StatelessWidget {
  const DocmostIosApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Docmost',
      debugShowCheckedModeBanner: false,
      theme: DocmostTheme.darkTheme,
      home: const DocmostHomeScreen(),
    );
  }
}
