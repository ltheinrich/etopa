package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.EditText
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.utils.Common

class MainActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    lateinit var preferences: SharedPreferences
    lateinit var pin: EditText
    lateinit var unlock: Button
    var pinSet = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("etopan")

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        pin = findViewById(R.id.pin)
        unlock = findViewById(R.id.unlock)
        pinSet = preferences.contains("pin")

        if (!pinSet) {
            unlock.text = getString(R.string.set_pin)
        }

        pin.requestFocus()
        common.login("test", "test") { response ->
            Log.d("test", response.toString())
        }
    }
}