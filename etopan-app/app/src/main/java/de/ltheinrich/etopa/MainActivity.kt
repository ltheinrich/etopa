package de.ltheinrich.etopa

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.utils.Common

class MainActivity : AppCompatActivity() {
    private val common: Common = Common.getInstance(this)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("etopan")

        val textView: TextView = findViewById(R.id.textView)
        common.login("test", "test") { response -> textView.text = response.toString() }
    }
}