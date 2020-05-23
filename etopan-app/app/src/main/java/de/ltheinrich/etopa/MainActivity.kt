package de.ltheinrich.etopa

import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.os.Bundle
import android.view.View
import android.view.inputmethod.EditorInfo
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity


class MainActivity : AppCompatActivity() {

    private lateinit var preferences: SharedPreferences
    private lateinit var instance: TextView
    private lateinit var username: TextView
    private lateinit var password: TextView
    private lateinit var key: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        instance = findViewById(R.id.instance);
        username = findViewById(R.id.username);
        password = findViewById(R.id.password);
        key = findViewById(R.id.key);

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        instance.text = preferences.getString("instance", "").orEmpty()
        username.text = preferences.getString("username", "").orEmpty()
        password.text = preferences.getString("password", "").orEmpty()

        key.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO) {
                loginClick(null)
            }
            false
        }

        System.loadLibrary("etopan")
    }

    fun loginClick(view: View?) {
        (view?:key).visibility = View.INVISIBLE
        if (instance.text.isNotEmpty() && username.text.isNotEmpty() && password.text.isNotEmpty() && key.text.isNotEmpty()) {
            val editor = preferences.edit()
            if (instance.text.toString() != preferences.getString(
                    "instance",
                    ""
                ) || username.text.toString() != preferences.getString(
                    "username",
                    ""
                ) || password.text.toString() != preferences.getString("password", "")
            ) {
                editor.remove("token")
            }
            editor.putString("instance", instance.text.toString())
            editor.putString("username", username.text.toString())
            editor.putString("password", password.text.toString())
            editor.commit()
            val keyHash = hashKey(key.text.toString())
            openApp(keyHash)
        } else {
            Toast.makeText(this@MainActivity, R.string.inputs_empty, Toast.LENGTH_LONG).show()
        }
        (view?:key).visibility = View.VISIBLE
    }

    private fun openApp(key: String) {
        val app = Intent(this@MainActivity, AppActivity::class.java)
        app.putExtra("key", key)
        this@MainActivity.startActivity(app)
    }

    private external fun hashKey(to: String): String
}
