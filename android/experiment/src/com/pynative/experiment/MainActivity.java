package com.pynative.experiment;

import android.app.Activity;
import android.graphics.Color;
import android.os.Bundle;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.TextView;

public class MainActivity extends Activity {
    private int count = 0;
    private TextView countText;
    private TextView greetingText;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        root.setGravity(Gravity.CENTER_HORIZONTAL);
        root.setPadding(40, 56, 40, 40);
        root.setBackgroundColor(Color.rgb(248, 250, 252));

        TextView title = new TextView(this);
        title.setText("PyNative Android Experiment");
        title.setTextSize(24);
        title.setTextColor(Color.rgb(15, 23, 42));
        root.addView(title, matchWrap());

        TextView subtitle = new TextView(this);
        subtitle.setText("Native Android UI shell built from the local SDK.");
        subtitle.setTextSize(15);
        subtitle.setTextColor(Color.rgb(71, 85, 105));
        subtitle.setPadding(0, 8, 0, 28);
        root.addView(subtitle, matchWrap());

        EditText nameInput = new EditText(this);
        nameInput.setHint("Username");
        nameInput.setSingleLine(true);
        root.addView(nameInput, matchWrap());

        greetingText = new TextView(this);
        greetingText.setText("Type a username, then tap the button.");
        greetingText.setTextSize(16);
        greetingText.setTextColor(Color.rgb(15, 118, 110));
        greetingText.setPadding(0, 18, 0, 18);
        root.addView(greetingText, matchWrap());

        countText = new TextView(this);
        countText.setText("Count: 0");
        countText.setTextSize(18);
        countText.setTextColor(Color.rgb(15, 23, 42));
        countText.setPadding(0, 6, 0, 12);
        root.addView(countText, matchWrap());

        Button button = new Button(this);
        button.setText("Increase");
        button.setOnClickListener(view -> {
            count += 1;
            countText.setText("Count: " + count);

            String username = nameInput.getText().toString().trim();
            if (username.isEmpty()) {
                greetingText.setText("Hello from Android.");
            } else {
                greetingText.setText("Hello, " + username + ".");
            }
        });
        root.addView(button, matchWrap());

        setContentView(root);
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
