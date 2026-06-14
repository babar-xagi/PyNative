package com.pynative.experiment;

import android.app.Activity;
import android.graphics.Color;
import android.graphics.Typeface;
import android.os.Bundle;
import android.util.Log;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.TextView;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import org.json.JSONException;
import org.json.JSONObject;

public class MainActivity extends Activity {
    private static final int ELEMENT_KIND = 0;
    private static final int ELEMENT_VALUE = 1;
    private static final int ELEMENT_COLOR = 2;
    private static final int ELEMENT_BACKGROUND_COLOR = 3;
    private static final int ELEMENT_FONT_SIZE = 4;
    private static final int ELEMENT_FONT_WEIGHT = 5;
    private static final int ELEMENT_WIDTH = 6;
    private static final int ELEMENT_HEIGHT = 7;
    private static final int ELEMENT_PADDING = 8;
    private static final int ELEMENT_MARGIN = 9;
    private static final int ELEMENT_ALIGN = 10;

    private int count = 0;
    private TextView statusText;
    private String runtimeInitJson = "{\"ok\":false,\"error\":\"not initialized\"}";
    private final List<TextView> textViews = new ArrayList<>();
    private final List<EditText> inputViews = new ArrayList<>();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        int rootPadding = dp(GeneratedApp.ROOT_PADDING);
        root.setPadding(rootPadding, rootPadding + dp(16), rootPadding, rootPadding);
        root.setBackgroundColor(parseColorOr(
                GeneratedApp.ROOT_BACKGROUND_COLOR,
                Color.rgb(248, 250, 252)
        ));

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

        renderGeneratedElements(root);
        runtimeInitJson = initializeRuntimeFromAssets();

        statusText = new TextView(this);
        statusText.setText("Android screen loaded. Nodes: "
                + GeneratedApp.NODE_COUNT
                + ". "
                + PyNativeBridge.status()
                + " "
                + runtimeInitStatus());
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

        Log.i("PyNative", "Loaded Android screen from " + GeneratedApp.SOURCE_PATH);

        setContentView(root);
    }

    private void renderGeneratedElements(LinearLayout root) {
        for (String[] element : GeneratedApp.ELEMENTS) {
            String kind = elementValue(element, ELEMENT_KIND);
            String value = elementValue(element, ELEMENT_VALUE);

            if ("Text".equals(kind)) {
                TextView textView = new TextView(this);
                textView.setText(value);
                applyTextStyle(textView, element, 18, Color.rgb(15, 23, 42));
                textViews.add(textView);
                root.addView(textView, paramsFor(element));
            } else if ("Input".equals(kind)) {
                EditText input = new EditText(this);
                input.setHint(value);
                input.setSingleLine(true);
                applyTextStyle(input, element, 16, Color.rgb(15, 23, 42));
                inputViews.add(input);
                root.addView(input, paramsFor(element));
            } else if ("Button".equals(kind)) {
                Button button = new Button(this);
                button.setText(value);
                applyTextStyle(button, element, 16, Color.rgb(15, 23, 42));
                button.setOnClickListener(view -> handleButtonClick(value));
                root.addView(button, paramsFor(element));
            } else if ("Image".equals(kind)) {
                TextView imageView = new TextView(this);
                imageView.setText("Image: " + value);
                applyTextStyle(imageView, element, 16, Color.rgb(71, 85, 105));
                root.addView(imageView, paramsFor(element));
            }
        }
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

        String eventJson = buildButtonEventJson(label, inputText);
        String responseJson = PyNativeBridge.dispatchEventJson(eventJson);
        int nativeEvents = nativeEventCount(responseJson);
        applyRuntimeResponse(responseJson);

        Log.i(
                "PyNative",
                "Android button event #" + count
                        + " label=" + label
                        + " pythonCallback=" + GeneratedApp.HAS_PYTHON_CALLBACKS
                        + " rustBridge=" + PyNativeBridge.isAvailable()
                        + " rustEvents=" + nativeEvents
                        + " event=" + eventJson
                        + " response=" + responseJson
        );
    }

    private String initializeRuntimeFromAssets() {
        try {
            String runtime = readAsset("pynative/runtime.json");
            String app = readAsset("pynative/app.py");
            String tree = readAsset("pynative/widget_tree.json");
            String response = PyNativeBridge.initializeRuntimeJson(runtime, app, tree);
            Log.i("PyNative", "Runtime init response=" + response);
            return response;
        } catch (IOException error) {
            Log.w("PyNative", "Runtime assets unavailable.", error);
            return "{\"ok\":false,\"error\":\"runtime assets unavailable\"}";
        }
    }

    private String runtimeInitStatus() {
        try {
            JSONObject response = new JSONObject(runtimeInitJson);
            if (response.optBoolean("runtime_loaded", false)) {
                return "Runtime assets loaded. App bytes: "
                        + response.optInt("app_source_len", 0)
                        + ".";
            }
            return "Runtime init failed.";
        } catch (JSONException error) {
            return "Runtime init response invalid.";
        }
    }

    private void applyRuntimeResponse(String responseJson) {
        try {
            JSONObject response = new JSONObject(responseJson);
            int nativeEvents = response.optInt("native_events", -1);
            String updatedText = response.optString("updated_text", "");

            if (!updatedText.isEmpty() && !textViews.isEmpty()) {
                textViews.get(0).setText(updatedText);
            }

            if (nativeEvents >= 0) {
                statusText.setText(statusText.getText()
                        + ". Rust events: "
                        + nativeEvents
                        + ". Update: "
                        + response.optString("updated_by", "none"));
            }
        } catch (JSONException error) {
            Log.w("PyNative", "Could not apply Rust event response: " + responseJson, error);
        }
    }

    private String buildButtonEventJson(String label, String inputText) {
        try {
            JSONObject event = new JSONObject();
            event.put("kind", "button_click");
            event.put("label", label);
            event.put("ui_count", count);
            event.put("input", inputText);
            event.put("has_python_callbacks", GeneratedApp.HAS_PYTHON_CALLBACKS);
            event.put("source_path", GeneratedApp.SOURCE_PATH);
            return event.toString();
        } catch (JSONException error) {
            return "{\"kind\":\"button_click\",\"error\":\"json_build_failed\"}";
        }
    }

    private int nativeEventCount(String responseJson) {
        try {
            JSONObject response = new JSONObject(responseJson);
            return response.optInt("native_events", -1);
        } catch (JSONException error) {
            Log.w("PyNative", "Could not parse Rust event response: " + responseJson, error);
            return -1;
        }
    }

    private String readAsset(String path) throws IOException {
        try (InputStream input = getAssets().open(path)) {
            ByteArrayOutputStream output = new ByteArrayOutputStream();
            byte[] buffer = new byte[4096];
            int read;
            while ((read = input.read(buffer)) != -1) {
                output.write(buffer, 0, read);
            }
            return output.toString(StandardCharsets.UTF_8.name());
        }
    }

    private LinearLayout.LayoutParams matchWrap() {
        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT
        );
        params.setMargins(0, 0, 0, 12);
        return params;
    }

    private LinearLayout.LayoutParams paramsFor(String[] element) {
        int width = intOr(elementValue(element, ELEMENT_WIDTH), ViewGroup.LayoutParams.MATCH_PARENT);
        int height = intOr(elementValue(element, ELEMENT_HEIGHT), ViewGroup.LayoutParams.WRAP_CONTENT);

        LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                width > 0 ? dp(width) : width,
                height > 0 ? dp(height) : height
        );

        int margin = dp(intOr(elementValue(element, ELEMENT_MARGIN), 0));
        params.setMargins(0, margin, 0, margin + dp(12));

        String align = elementValue(element, ELEMENT_ALIGN);
        if ("center".equalsIgnoreCase(align)) {
            params.gravity = Gravity.CENTER_HORIZONTAL;
        } else if ("end".equalsIgnoreCase(align) || "right".equalsIgnoreCase(align)) {
            params.gravity = Gravity.END;
        }

        return params;
    }

    private void applyTextStyle(TextView view, String[] element, int defaultSize, int defaultColor) {
        view.setTextColor(parseColorOr(elementValue(element, ELEMENT_COLOR), defaultColor));

        String backgroundColor = elementValue(element, ELEMENT_BACKGROUND_COLOR);
        if (!backgroundColor.isEmpty()) {
            view.setBackgroundColor(parseColorOr(backgroundColor, Color.TRANSPARENT));
        }

        view.setTextSize(intOr(elementValue(element, ELEMENT_FONT_SIZE), defaultSize));

        if ("bold".equalsIgnoreCase(elementValue(element, ELEMENT_FONT_WEIGHT))) {
            view.setTypeface(Typeface.DEFAULT, Typeface.BOLD);
        }

        int padding = dp(intOr(elementValue(element, ELEMENT_PADDING), 0));
        if (padding > 0) {
            view.setPadding(padding, padding, padding, padding);
        }

        String align = elementValue(element, ELEMENT_ALIGN);
        if ("center".equalsIgnoreCase(align)) {
            view.setGravity(Gravity.CENTER);
        } else if ("end".equalsIgnoreCase(align) || "right".equalsIgnoreCase(align)) {
            view.setGravity(Gravity.END);
        }
    }

    private int parseColorOr(String value, int fallback) {
        if (value == null || value.trim().isEmpty()) {
            return fallback;
        }

        try {
            return Color.parseColor(value.trim());
        } catch (IllegalArgumentException ignored) {
            return fallback;
        }
    }

    private int intOr(String value, int fallback) {
        if (value == null || value.trim().isEmpty()) {
            return fallback;
        }

        try {
            return Integer.parseInt(value.trim());
        } catch (NumberFormatException ignored) {
            return fallback;
        }
    }

    private int dp(int value) {
        if (value <= 0) {
            return value;
        }
        return Math.round(value * getResources().getDisplayMetrics().density);
    }

    private String elementValue(String[] element, int index) {
        if (index >= element.length || element[index] == null) {
            return "";
        }
        return element[index];
    }
}
