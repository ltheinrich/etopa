package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.utils.Common
import java.util.*

class MainActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var pin: EditText
    private lateinit var unlock: Button

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

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
                val splitAt = Random().nextInt(30)
                val uuid = UUID.randomUUID().toString()
                val pinSetEncrypted =
                    common.encrypt(
                        pinHash,
                        uuid.substring(0, splitAt) + "etopan_pin_set" + uuid.substring(splitAt)
                    )
                preferences.edit().putString("pin_set", pinSetEncrypted).apply()
                common.toast(R.string.pin_set)
            } else if (!common.decrypt(pinHash, pinSet).contains("etopan_pin_set")) {
                pin.text.clear()
                //common.hideKeyboard()
                common.toast(R.string.incorrect_pin, 500)
                return@setOnClickListener
            }

            common.pinHash = pinHash
            common.decryptLogin(preferences)
            common.openActivity(LoginActivity::class)
        }
    }
}