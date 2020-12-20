package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import android.view.inputmethod.EditorInfo
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivitySettingsBinding
import de.ltheinrich.etopa.utils.*

class SettingsActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivitySettingsBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivitySettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)
        binding.toolbar.root.title =
            getString(R.string.app_name) + ": " + getString(R.string.settings)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)

        binding.pin.editText?.setText(emptyPin)
        binding.instance.editText?.setText(common.instance)

        if (common.username.isNotEmpty())
            binding.username.editText?.setText(common.username)
        if (common.passwordHash.isNotEmpty())
            binding.password.editText?.setText(emptyPassword)
        if (common.keyHash.isNotEmpty())
            binding.key.editText?.setText(emptyPassword)

        binding.save.setOnClickListener {
            save()
        }
        binding.key.editText?.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO)
                save()
            true
        }
    }

    private fun save() {
        common.hideKeyboard(this)

        val pin = inputString(binding.pin)
        val instance = inputString(binding.instance)
        val username = inputString(binding.username)
        val password = inputString(binding.password)
        val key = inputString(binding.key)

        common.instance = instance
        common.username = username
        common.passwordHash = common.hashPassword(password)
        common.keyHash = common.hashKey(key)

        common.encryptLogin(
            preferences,
            common.hashPin(pin)
                .let {
                    if (it == emptyPinHash) {
                        common.pinHash
                    } else {
                        it
                    }
                })

        common.toast(R.string.settings_saved)
        common.newLogin(preferences)
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            if (intent.hasExtra("incorrectLogin")) {
                common.openActivity(MainActivity::class)
            } else {
                common.openActivity(AppActivity::class)
            }
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}
