package com.pynative.experiment;

import android.app.Activity;
import android.graphics.Color;
import android.os.Bundle;
import android.util.Log;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.TextView;
import java.util.ArrayList;
import java.util.List;

public class MainActivity extends Activity {
    private int count = 0;
    private TextView statusText;
    private final List<TextView> textViews = new ArrayList<>();
    private final List<EditText> inputViews = new ArrayList<>();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        root.setPadding(40, 56, 40, 40);
        root.setBackgroundColor(Color.rgb(248, 250, 252));

        TextView title = new TextView(this);
        title.setText(GeneratedApp.TITLE);
        title.setTextSize(24);
        title.setTextColor(Color.rgb(15, 23, 42));
        root.addView(title, matchWrap());

        TextView subtitle = new TextView(this);
        subtitle.setText("Source: " + GeneratedApp.SOURCE_PATH);
        subtitle.setTextSize(15);
        subtitle.setTextColor(Color.rgb(71, 85, 105));
        subtitle.setPadding(0, 8, 0, 28);
        root.addView(subtitle, matchWrap());

        for (String text : GeneratedApp.TEXTS) {
            TextView textView = new TextView(this);
            textView.setText(text);
            textView.setTextSize(18);
            textView.setTextColor(Color.rgb(15, 23, 42));
            textView.setPadding(0, 6, 0, 12);
            textViews.add(textView);
            root.addView(textView, matchWrap());
        }

        for (String image : GeneratedApp.IMAGES) {
            TextView imageView = new TextView(this);
            imageView.setText("Image: " + image);
            imageView.setTextSize(16);
            imageView.setTextColor(Color.rgb(71, 85, 105));
            root.addView(imageView, matchWrap());
        }

        for (String placeholder : GeneratedApp.INPUT_PLACEHOLDERS) {
            EditText input = new EditText(this);
            input.setHint(placeholder);
            input.setSingleLine(true);
            inputViews.add(input);
            root.addView(input, matchWrap());
        }

        statusText = new TextView(this);
        statusText.setText("Android screen loaded. Nodes: " + GeneratedApp.NODE_COUNT);
        statusText.setTextSize(16);
        statusText.setTextColor(Color.rgb(15, 118, 110));
        statusText.setPadding(0, 18, 0, 18);
        root.addView(statusText, matchWrap());

        if (GeneratedApp.BUTTON_LABELS.length == 0) {
            TextView empty = new TextView(this);
            empty.setText("No buttons were exported from this app.");
            empty.setTextSize(14);
            empty.setTextColor(Color.rgb(100, 116, 139));
            root.addView(empty, matchWrap());
        }

        for (String label : GeneratedApp.BUTTON_LABELS) {
            Button button = new Button(this);
            button.setText(label);
            button.setOnClickListener(view -> handleButtonClick(label));
            root.addView(button, matchWrap());
        }

        Log.i("PyNative", "Loaded Android screen from " + GeneratedApp.SOURCE_PATH);

        setContentView(root);
    }

    private void handleButtonClick(String label) {
        count += 1;

        if (!textViews.isEmpty()) {
            TextView first = textViews.get(0);
            if (first.getText().toString().startsWith("Count:")) {
                first.setText("Count: " + count);
            }
        }

        String inputText = "";
        if (!inputViews.isEmpty()) {
            inputText = inputViews.get(0).getText().toString().trim();
        }

        if (inputText.isEmpty()) {
            statusText.setText("Tap " + count + ": " + label);
        } else {
            statusText.setText("Tap " + count + ": " + label + " for " + inputText);
        }

        Log.i(
                "PyNative",
                "Android button event #" + count
                        + " label=" + label
                        + " pythonCallback=" + GeneratedApp.HAS_PYTHON_CALLBACKS
        );
    }

    private LinearLayout.LayoutParams matchWrap() {
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        );
        params.setMargins(0, 0, 0, 12);
        return params;
    }
}
