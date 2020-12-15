package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.Menu
import android.view.MenuItem
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityMainBinding
import de.ltheinrich.etopa.utils.Common
import java.util.*

class MainActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        common.settingsVisible = false
        setSupportActionBar(binding.toolbar.root)

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        binding.pin.requestFocus()

        val pinSet = preferences.getString("pin_set", null)
        if (pinSet == null) {
            binding.unlock.text = getString(R.string.set_pin)
        }

        binding.unlock.setOnClickListener {
            val pinHash = common.hashPin(binding.pin.text.toString())
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
                binding.pin.text.clear()
                //common.hideKeyboard()
                common.toast(R.string.incorrect_pin, 500)
                return@setOnClickListener
            }

            common.pinHash = pinHash
            common.decryptLogin(preferences)
            common.openActivity(LoginActivity::class)
        }
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}