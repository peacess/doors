import 'package:flutter/material.dart';
import 'package:idl/idl/chat_chat_generated.dart';
import 'package:scrollable_positioned_list/scrollable_positioned_list.dart';

class MessagesWidget extends StatefulWidget {
  const MessagesWidget(this.texts, {super.key});

  final ValueNotifier<List<TextMessageT>> texts;

  @override
  State<MessagesWidget> createState() => _MessagesWidgetState();
}

class _MessagesWidgetState extends State<MessagesWidget> {
  final ItemScrollController _itemScrollController = ItemScrollController();
  final TextEditingController _controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder(
      valueListenable: widget.texts,
      builder: (_, value, _) {
        return Column(
          children: [
            Expanded(
              child: ScrollablePositionedList.builder(
                itemScrollController: _itemScrollController,
                itemCount: value.length,
                itemBuilder: (_, index) {
                  final text = value[index];
                  return Container(padding: const EdgeInsets.all(10.0), child: Text(text.text ?? ""));
                },
              ),
            ),
            TextField(
              controller: _controller,
              decoration: InputDecoration(
                prefix: IconButton(onPressed: _enter, icon: Icon(Icons.done)),
                suffix: IconButton(onPressed: _enter, icon: Icon(Icons.done)),
              ),
              onSubmitted: (_) => _enter(),
              textInputAction: TextInputAction.send,
            ),
          ],
        );
      },
    );
  }

  void _enter() {
    // final text = _controller.text;
    //todo send text message
  }
}
