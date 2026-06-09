import 'package:flutter/foundation.dart';

import '../common.dart';

void androidChannelInit() {
  gFFI.setMethodCallHandler((method, arguments) {
    debugPrint("flutter got android msg,$method,$arguments");
    try {
      switch (method) {
        case "on_state_changed":
          {
            final name = arguments["name"] as String;
            final value = arguments["value"] as String == "true";
            debugPrint("from jvm:on_state_changed,$name:$value");
            gFFI.serverModel.changeStatue(name, value);
            break;
          }
        case "on_android_permission_result":
          {
            final type = arguments["type"] as String;
            final result = arguments["result"] as bool;
            AndroidPermissionManager.complete(type, result);
            break;
          }
        case "msgbox":
          {
            final type = arguments["type"] as String;
            final title = arguments["title"] as String;
            final text = arguments["text"] as String;
            final link = (arguments["link"] ?? '') as String;
            msgBox(gFFI.sessionId, type, title, text, link, gFFI.dialogManager);
            break;
          }
      }
    } catch (e) {
      debugPrintStack(label: "MethodCallHandler err:$e");
    }
    return "";
  });
}

