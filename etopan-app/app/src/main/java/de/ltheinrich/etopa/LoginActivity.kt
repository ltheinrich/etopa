package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.KeyEvent
import android.view.Menu
import android.view.MenuItem
import android.view.View
import android.view.inputmethod.EditorInfo
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.databinding.ActivityLoginBinding
import de.ltheinrich.etopa.utils.Common
import de.ltheinrich.etopa.utils.inputString

class LoginActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var binding: ActivityLoginBinding

    /*
    TODO:
    MOVE LOGIN COMPLETELY TO SETTINGS!
    MOVE LOGIN COMPLETELY TO SETTINGS!
    MOVE LOGIN COMPLETELY TO SETTINGS!
    MOVE LOGIN COMPLETELY TO SETTINGS!
    MOVE LOGIN COMPLETELY TO SETTINGS!
     */

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityLoginBinding.inflate(layoutInflater)
        setContentView(binding.root)
        common.settingsVisible = true
        setSupportActionBar(binding.toolbar.root)

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)

        binding.username.editText?.setText(common.username)

        if (!preferences.getString("token", "")
                .isNullOrEmpty() && !intent.hasExtra("noAutoLogin")
        ) {
            common.toast(R.string.logging_in)
            common.request(
                "user/valid",
                { responseValid ->
                    if (responseValid.has("valid") && responseValid.getBoolean("valid")) {
                        common.openActivity(AppActivity::class)
                    } else {
                        common.request("user/login",
                            { response ->
                                if (response.has("token")) {
                                    val token = response.getString("token")
                                    common.token = token
                                    val editor = preferences.edit()
                                    editor.putString(
                                        "token",
                                        common.encrypt(common.pinHash, common.token)
                                    )
                                    editor.apply()
                                    common.openActivity(AppActivity::class)
                                } else {
                                    common.toast(R.string.incorrect_login)
                                }
                            },
                            Pair("username", common.username),
                            Pair("password", common.passwordHash),
                            error_handler = {
                                common.toast(R.string.network_unreachable)
                                if (preferences.contains("secretStorage")) {
                                    common.openActivity(AppActivity::class)
                                }
                            })
                    }
                },
                Pair("username", common.username),
                Pair("token", common.token),
                error_handler = {
                    common.toast(R.string.network_unreachable)
                    if (preferences.contains("secretStorage")) {
                        common.openActivity(AppActivity::class)
                    }
                })
        }

        binding.key.editText?.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO) {
                common.toast(R.string.logging_in)
                loginClick(null)
            }
            false
        }
    }

    fun loginClick(@SuppressWarnings("unused") view: View?) {
        common.hideKeyboard()
        if (inputString(binding.username).isNotEmpty() &&
            inputString(binding.password).isNotEmpty() && inputString(binding.key).isNotEmpty()
        ) {
            common.username = binding.username.editText?.text.toString()
            common.passwordHash = common.hashPassword(binding.password.editText?.text.toString())
            common.keyHash = common.hashKey(binding.key.editText?.text.toString())
            common.request(
                "user/login",
                { response ->
                    if (response.has("token")) {
                        val token = response.getString("token")
                        common.token = token
                        val editor = preferences.edit()
                        editor.putString(
                            "username",
                            common.encrypt(common.pinHash, common.username)
                        )
                        editor.putString(
                            "passwordHash",
                            common.encrypt(common.pinHash, common.passwordHash)
                        )
                        editor.putString("keyHash", common.encrypt(common.pinHash, common.keyHash))
                        editor.putString("token", common.encrypt(common.pinHash, common.token))
                        editor.apply()
                        common.openActivity(AppActivity::class)
                    } else {
                        common.toast(R.string.incorrect_login)
                    }
                },
                Pair("username", common.username),
                Pair("password", common.passwordHash),
                error_handler = {
                    common.toast(R.string.network_unreachable)
                })

        } else {
            common.toast(R.string.inputs_empty)
        }
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        if (keyCode == KeyEvent.KEYCODE_BACK) {
            common.openActivity(MainActivity::class)
            return true
        }

        return super.onKeyDown(keyCode, event)
    }

    override fun onOptionsItemSelected(item: MenuItem) = common.handleMenu(item)
    override fun onCreateOptionsMenu(menu: Menu?): Boolean = common.createMenu(menu)
}
