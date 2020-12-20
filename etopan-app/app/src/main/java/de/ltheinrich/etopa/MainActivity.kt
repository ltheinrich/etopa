package de.ltheinrich.etopa

import android.annotation.SuppressLint
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.Menu
import android.view.MenuItem
import android.view.WindowManager
import android.view.inputmethod.EditorInfo
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityMainBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.inputString

class MainActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityMainBinding
    private var pinSet: String? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        common.extendedMenu = false
        binding.toolbar.root.title =
            getString(R.string.app_name) + ": " + getString(R.string.unlock)
        setSupportActionBar(binding.toolbar.root)
        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)

        window.setSoftInputMode(WindowManager.LayoutParams.SOFT_INPUT_STATE_ALWAYS_VISIBLE)
        binding.pin.editText?.requestFocus()
        pinSet = preferences.getString("pin_set", null)
        if (pinSet == null) {
            binding.unlock.text = getString(R.string.set_pin)
        }

        binding.unlock.setOnClickListener {
            unlock()
        }
        binding.pin.editText?.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO)
                unlock()
            true
        }
    }

    @SuppressLint("CommitPrefEdits")
    private fun unlock() {
        common.toast(R.string.unlocking)
        common.hideKeyboard(this)
        val pinHash = common.hashPin(inputString(binding.pin))
        binding.pin.editText?.text?.clear()

        if (pinSet == null) {
            common.setPin(preferences.edit(), pinHash)
        } else if (!common.decrypt(pinHash, pinSet!!).contains("etopan_pin_set")) {
            return common.toast(R.string.incorrect_pin, 500)
        } else {
            common.pinHash = pinHash
        }

        common.extendedMenu = true
        common.decryptLogin(preferences)

        if (pinSet == null) {
            // binding.unlock.text = getString(R.string.unlock)
            common.openActivity(SettingsActivity::class, Pair("incorrectLogin", "incorrectLogin"))
        } else {
            login()
        }
    }

    private fun login() {
        if (preferences.getString("token", "").isNullOrEmpty()) {
            common.newLogin(preferences)
        } else {
            tokenLogin()
        }
    }

    private fun tokenLogin() {
        common.toast(R.string.logging_in)
        common.request(
            "user/valid",
            { responseValid ->
                if (responseValid.has("valid") && responseValid.getBoolean("valid")) {
                    common.openActivity(AppActivity::class)
                } else {
                    common.newLogin(preferences)
                }
            },
            Pair("username", common.username),
            Pair("token", common.token),
            error_handler = { common.offlineLogin(preferences) })
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}