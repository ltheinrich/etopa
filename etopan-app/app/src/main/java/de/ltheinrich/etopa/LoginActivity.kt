package de.ltheinrich.etopa

import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.view.View
import android.view.inputmethod.EditorInfo
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import de.ltheinrich.etopa.utils.Common

class LoginActivity : AppCompatActivity() {

    private val common: Common = Common.getInstance(this)
    private lateinit var preferences: SharedPreferences
    private lateinit var instance: TextView
    private lateinit var username: TextView
    private lateinit var password: TextView
    private lateinit var key: TextView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_login)
        System.loadLibrary("etopan")

        preferences = getSharedPreferences("etopa", Context.MODE_PRIVATE)
        instance = findViewById(R.id.instance);
        username = findViewById(R.id.username);
        password = findViewById(R.id.password);
        key = findViewById(R.id.key);

        instance.text = common.instance
        username.text = common.username

        if (!preferences.getString("token", "").isNullOrEmpty()) {
            common.toast(R.string.logging_in)
            common.request(
                "user/valid",
                { response ->
                    if (response.has("valid") && response.getBoolean("valid")) {
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

        key.setOnEditorActionListener { _, actionId, _ ->
            if (actionId == EditorInfo.IME_ACTION_DONE || actionId == EditorInfo.IME_ACTION_GO) {
                common.toast(R.string.logging_in)
                loginClick(null)
            }
            false
        }
    }

    fun loginClick(view: View?) {
        common.hideKeyboard()
        if (instance.text.isNotEmpty() && username.text.isNotEmpty() && password.text.isNotEmpty() && key.text.isNotEmpty()) {
            common.instance = instance.text.toString()
            common.username = username.text.toString()
            common.passwordHash = common.hashPassword(password.text.toString())
            common.keyHash = common.hashKey(key.text.toString())
            common.request("user/login", { response ->
                if (response.has("token")) {
                    val token = response.getString("token")
                    common.token = token
                    val editor = preferences.edit()
                    editor.putString("instance", common.encrypt(common.pinHash, common.instance))
                    editor.putString("username", common.encrypt(common.pinHash, common.username))
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
            }, Pair("username", common.username), Pair("password", common.passwordHash))

        } else {
            common.toast(R.string.inputs_empty)
        }
    }
}
