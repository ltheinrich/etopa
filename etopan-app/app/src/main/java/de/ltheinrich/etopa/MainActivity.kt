package de.ltheinrich.etopa

import android.app.Activity
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.View
import android.view.inputmethod.InputMethodManager
import android.widget.Button
import android.widget.EditText
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.utils.Common


class MainActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    lateinit var preferences: SharedPreferences
    lateinit var pin: EditText
    lateinit var unlock: Button

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("etopan")

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        pin = findViewById(R.id.pin)
        unlock = findViewById(R.id.unlock)
        pin.requestFocus()

        val pinSet = preferences.getString("pin_set", null)
        if (pinSet == null) {
            unlock.text = getString(R.string.set_pin)
        }

        unlock.setOnClickListener {
            val pinHash = common.hashPin(pin.text.toString())
            if (pinSet == null) {
                val pinSetEncrypted = common.encrypt(pinHash, "pin_set")
                preferences.edit().putString("pin_set", pinSetEncrypted).apply()
                common.toast(R.string.pin_set)
            } else if (common.decrypt(pinHash, pinSet) != "pin_set") {
                pin.text.clear()
                hideKeyboard(this@MainActivity)
                common.toast(R.string.incorrect_pin)
                return@setOnClickListener
            }

            common.openActivity(
                LoginActivity::class,
                Pair("pin", common.hashPin(pin.text.toString()))
            )
        }
    }

    private fun hideKeyboard(activity: Activity) {
        val imm: InputMethodManager =
            activity.getSystemService(Activity.INPUT_METHOD_SERVICE) as InputMethodManager
        var view: View? = activity.currentFocus
        if (view == null) {
            view = View(activity)
        }
        imm.hideSoftInputFromWindow(view.windowToken, 0)
    }
}