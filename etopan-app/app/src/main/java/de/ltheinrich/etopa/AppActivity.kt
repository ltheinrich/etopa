package de.ltheinrich.etopa

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import android.os.Bundle
import android.view.KeyEvent

class AppActivity : Activity() {

    lateinit var preferences: SharedPreferences
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_app)

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
    }

    private fun openMain() {
        val main = Intent(this@AppActivity, LoginActivity::class.java)
        this@AppActivity.startActivity(main)
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            openMain()
            return true
        }

        return super.onKeyDown(keyCode, event)
    }
}
