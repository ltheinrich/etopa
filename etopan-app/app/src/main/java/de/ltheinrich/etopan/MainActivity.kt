package de.ltheinrich.etopan

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.TextView

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val text = findViewById<TextView>(R.id.hello_world)

        System.loadLibrary("etopan")
        text.text = hello("there!")
    }

    external fun hello(to: String): String
}
