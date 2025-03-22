import 'dart:io';
import 'package:path_provider/path_provider.dart';
import 'package:shared_preferences/shared_preferences.dart';

class FileUtils {
  static const String _lastFileKey = 'last_opened_file';
  
  /// Get the last opened file path
  static Future<String?> getLastOpenedFile() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString(_lastFileKey);
  }

  /// Save the last opened file path
  static Future<void> saveLastOpenedFile(String filePath) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_lastFileKey, filePath);
  }

  /// Read file content
  static Future<String> readFile(String filePath) async {
    final file = File(filePath);
    return await file.readAsString();
  }

  /// Write content to file
  static Future<void> writeFile(String filePath, String content) async {
    final file = File(filePath);
    await file.writeAsString(content);
  }

  /// Get the application documents directory
  static Future<String> getAppDir() async {
    final dir = await getApplicationDocumentsDirectory();
    return dir.path;
  }

  /// Check if file exists
  static Future<bool> fileExists(String filePath) async {
    final file = File(filePath);
    return await file.exists();
  }

  /// Get file extension
  static String getFileExtension(String filePath) {
    return filePath.split('.').last.toLowerCase();
  }

  /// Check if file is a MetaMark file
  static bool isMetaMarkFile(String filePath) {
    return getFileExtension(filePath) == 'mmk';
  }
} 