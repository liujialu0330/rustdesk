import 'package:flutter/material.dart';

import '../../common.dart';
import 'home_page.dart';

class ServerPage extends StatelessWidget implements PageShape {
  @override
  final title = translate("Local screen");

  @override
  final icon = const Icon(Icons.phonelink_lock);

  @override
  final appBarActions = const <Widget>[];

  ServerPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Text(
          translate("Local screen sharing is disabled in this build."),
          textAlign: TextAlign.center,
        ),
      ),
    );
  }
}

